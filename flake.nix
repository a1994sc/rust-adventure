{
  description = "Rust playground";

  inputs = {
    # keep-sorted start block=yes case=no
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "rust-analyzer-src";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    pre-commit-hooks = {
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
      url = "github:/cachix/git-hooks.nix";
    };
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    rust-analyzer-src = {
      flake = false;
      url = "github:rust-lang/rust-analyzer/refs/tags/nightly";
    };
    services-flake.url = "github:juspay/services-flake";
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # keep-sorted end
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      ...
    }:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.process-compose-flake.flakeModule
      ];
      perSystem =
        {
          self',
          pkgs,
          lib,
          system,
          ...
        }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ inputs.fenix.overlays.default ];
          };
          treefmtEval = inputs.treefmt-nix.lib.evalModule pkgs (
            { pkgs, ... }:
            {
              projectRootFile = "flake.nix";
              # keep-sorted start block=yes case=no
              programs.dprint = {
                enable = true;
                settings = {
                  includes = [
                    "**/*.toml"
                    "**/*.json"
                    "**/*.md"
                  ];
                  excludes = [
                    "**/target"
                  ];
                  plugins =
                    let
                      dprintWasmPluginUrl = n: v: "https://plugins.dprint.dev/${n}-${v}.wasm";
                    in
                    [
                      (dprintWasmPluginUrl "json" "0.19.3")
                      (dprintWasmPluginUrl "markdown" "0.17.8")
                      (dprintWasmPluginUrl "toml" "0.6.2")
                    ];
                };
              };
              programs.jsonfmt = {
                enable = true;
                package = pkgs.jsonfmt;
              };
              programs.keep-sorted.enable = true;
              programs.nixfmt = {
                enable = true;
                package = pkgs.nixfmt-rfc-style;
              };
              programs.rustfmt.enable = true;
              programs.statix.enable = true;
              # keep-sorted end
              # programs.clippy.enable = true;
              # programs.clippy.settings.allFeatures = true;
              settings.formatter = {
                # keep-sorted start block=yes
                actionlint = {
                  command = pkgs.actionlint;
                  includes = [ "./.github/workflows/*.yml" ];
                };
                jsonfmt.includes = [
                  "*.json"
                  "./.github/*.json"
                  "./.vscode/*.json"
                ];
                # keep-sorted end
              };
            }
          );
          target = (if (lib.hasPrefix "x86_64" system) then "x86_64" else "aarch64") + "-unknown-linux-gnu";
          fenix =
            with inputs.fenix.packages.${system};
            combine [
              stable.toolchain
              targets."${target}".stable.rust-std
            ];
          buildInputs = with pkgs; [
            openssl
            buildPackages.pkg-config
          ];
          nativeBuildInputs = [ fenix ];
          env = rec {
            CARGO_BUILD_TARGET = target;
            CARGO_LINKER = "${pkgs.clang_18}/bin/clang";
            CARGO_RUSTFLAGS = "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold " + RUSTFLAGS;
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
            RUSTFLAGS = "";
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
        in
        {
          packages =
            let
              inherit ((pkgs.lib.importTOML ./Cargo.toml).package) version name;
              inherit ((pkgs.lib.importTOML ./Cargo.toml).workspace.package) authors;
              # https://discourse.nixos.org/t/passing-git-commit-hash-and-tag-to-build-with-flakes/11355/2
              img = {
                name = "ghcr.io/a1994sc/rust/" + self'.packages.default.pname;
                tag = version + "-" + (if (self ? shortRev) then self.shortRev else "dirty");
                config = {
                  Cmd = [ "/bin/${self'.packages.default.pname}" ];
                  Labels = {
                    "org.opencontainers.image.description" = "OCI image of ${name}";
                    "org.opencontainers.image.source" = "https://github.com/a1994sc/rust-adventure";
                    "org.opencontainers.image.version" = version;
                    "org.opencontainers.image.licenses" = "MIT";
                    "org.opencontainers.image.revision" = if (self ? rev) then self.rev else "dirty";
                    "org.opencontainers.image.authors" = lib.strings.trim (builtins.concatStringsSep ", " authors);
                  };
                };
                uid = 60000;
                gid = 60000;
                copyToRoot = self'.packages.default;
              };
            in
            {
              default =
                pkgs.pkgsStatic.rustPlatform.buildRustPackage.override
                  {
                    stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
                  }
                  {
                    pname = name;
                    inherit
                      buildInputs
                      env
                      version
                      nativeBuildInputs
                      ;
                    src = ./.;
                    cargoBuildFlags = "-p ${name}";
                    cargoLock.lockFile = ./Cargo.lock;
                  };
              # link: https://fasterthanli.me/series/building-a-rust-service-with-nix/part-11
              image = pkgs.dockerTools.buildImage {
                inherit (img)
                  tag
                  config
                  uid
                  gid
                  name
                  copyToRoot
                  ;
              };
              # CI "packages"
              version = pkgs.writeText "version" ''
                ${img.name}:${img.tag}
              '';
              versionTag = pkgs.writeText "version" ''
                ${img.name}:${version}
              '';
              tag = self'.packages.image.override {
                tag = version;
                config.Labels."org.opencontainers.image.revision" = version;
              };
            };
          devShells.default =
            pkgs.mkShell.override
              {
                stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
              }
              {
                inherit buildInputs env;
                inherit (self'.packages.default) nativeBuildInputs;
                # inputs.fenix.packages.${system}.stable.toolchain
                name = "rust";
                # Used for development and testing
                packages = with pkgs; [
                  typos
                  gnumake
                  clippy
                  cargo-machete
                  process-compose
                  cargo-watch
                  cargo-expand
                  cargo-typify
                  nodePackages.typescript-language-server
                  vscode-langservers-extracted
                  gdb
                ];
                # When using the "pre-commit" shellHook, it does not load the `env` attributes... so we need to load them manually, but only for the devShell(s).
                shellHook =
                  self'.checks.pre-commit-check.shellHook
                  + "\n"
                  + (builtins.concatStringsSep "\n" (
                    lib.attrsets.mapAttrsToList (k: v: "export ${k}=${lib.strings.escapeShellArg v}") env
                  ));
              };
          formatter = treefmtEval.config.build.wrapper;
          checks.pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              # keep-sorted start case=no
              check-executables-have-shebangs.enable = true;
              detect-private-keys.enable = true;
              end-of-file-fixer.enable = true;
              nixfmt-rfc-style.enable = true;
              trim-trailing-whitespace.enable = true;
              # keep-sorted end
              file-format-nix = {
                enable = true;
                entry = "nix fmt";
                pass_filenames = false;
              };
              file-format-cargo = {
                enable = true;
                entry = "cargo fmt";
                pass_filenames = false;
              };
            };
          };
          process-compose.support-services =
            { config, ... }:
            {
              imports = [
                inputs.services-flake.processComposeModules.default
              ];
              services.redis."redis".enable = true;
            };
        };
    };
}
