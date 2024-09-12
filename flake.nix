{
  description = "Rust playground";

  inputs = {
    # keep-sorted start block=yes case=no
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    services-flake.url = "github:juspay/services-flake";
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    zarf-dev = {
      url = "github:zarf-dev/zarf/v0.39.0";
      flake = false;
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
            overlays = [ (import inputs.rust-overlay) ];
          };
          treefmtEval = inputs.treefmt-nix.lib.evalModule pkgs (
            { pkgs, ... }:
            {
              projectRootFile = "flake.nix";
              # keep-sorted start block=yes case=no
              programs.dprint = {
                enable = true;
                settings = {
                  includes = [ "**/*.{json,md,toml}" ];
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
          nativeBuildInputs = with pkgs; [
            (rust-bin.stable.latest.complete.override {
              targets = [
                "x86_64-unknown-linux-musl"
                "aarch64-unknown-linux-musl"
              ];
            })
          ];
          buildInputs = with pkgs; [
            gnumake
            openssl
          ];
          env = rec {
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
            CARGO_LINKER = "${pkgs.clang_18}/bin/clang";
            CARGO_RUSTFLAGS = "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold " + RUSTFLAGS;
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
            RUSTFLAGS = "-C target-feature=+crt-static -C strip=symbols";
          };
          preBuild = ''
            cp -f ${inputs.zarf-dev}/zarf.schema.json schema/zarf.schema.json
          '';
          scrtipt = with pkgs; [ (writeShellScriptBin "gather-zarf-schema" preBuild) ];
        in
        {
          packages =
            let
              inherit ((pkgs.lib.importTOML ./Cargo.toml).package) version name;
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
                  };
                };
                uid = 60000;
                gid = 60000;
              };
            in
            {
              default =
                pkgs.rustPlatform.buildRustPackage.override
                  {
                    # inherit (pkgs.pkgsMusl) stdenv;
                    stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
                  }
                  {
                    pname = name;
                    inherit
                      nativeBuildInputs
                      buildInputs
                      env
                      version
                      preBuild
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
                  ;
                copyToRoot = pkgs.runCommand "project" { } ''
                  mkdir -p $out/bin
                  cp ${self'.packages.default}/bin/${self'.packages.default.pname} $out/bin
                '';
              };
              # CI packages
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
                # inherit (pkgs.pkgsMusl) stdenv;
                stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
              }
              {
                inherit nativeBuildInputs buildInputs env;
                name = "rust";
                # Used for development and testing
                packages =
                  with pkgs;
                  [
                    typos
                    gnumake
                    clippy
                    cargo-machete
                    process-compose
                    cargo-watch
                    cargo-expand
                    nodePackages.typescript-language-server
                    vscode-langservers-extracted
                  ]
                  ++ scrtipt;
              };
          formatter = treefmtEval.config.build.wrapper;
          process-compose.redis-service =
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
