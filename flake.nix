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
          # f-core = inputs.fenix.packages.${system};
          # f-wasm =
          #   with inputs.fenix.packages.${system};
          #   combine [
          #     stable.toolchain
          #     targets.wasm32-unknown-unknown.stable.rust-std
          #   ];
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
                      (dprintWasmPluginUrl "markdown" "0.17.0")
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
            inputs.fenix.packages.${system}.stable.toolchain
            pkg-config
            # Use mold for faster linking
            mold
            clang_15
          ];
          buildInputs = with pkgs; [
            openssl
            cargo-cross
          ];
          env = {
            CARGO_LINKER = "clang";
            CARGO_RUSTFLAGS = "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
            RUSTFLAGS = "-C relocation-model=static -C strip=symbols";
          };
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
                    "org.opencontainers.image.description" = "Playground application of ${name}";
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
                    stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
                  }
                  {
                    pname = name;
                    inherit
                      nativeBuildInputs
                      buildInputs
                      env
                      version
                      ;
                    src = ./.;
                    cargoBuildFlags = "-p ${name}";
                    cargoLock.lockFile = ./Cargo.lock;
                  };
              # # Doesn't work that well, but want to eventually get an image with just the static binary in it......
              # minImage = pkgs.dockerTools.streamLayeredImage {
              #   inherit (img) tag config uid gid name;
              #   maxLayers = 2;
              #   contents = [ self'.packages.default ];
              #   # Any mkdir running in this step won't actually make it to the image,
              #   # hence we use the tmpDir derivation in the contents
              #   fakeRootCommands = ''
              #     find nix/store -maxdepth 1 ! -name "*-${self'.packages.default.pname}-*" -type d -delete
              #   '';
              # };
              # link: https://fasterthanli.me/series/building-a-rust-service-with-nix/part-11
              image = pkgs.dockerTools.buildImage {
                inherit (img) tag config uid gid name;
                copyToRoot = pkgs.runCommand "project" { } ''
                  mkdir -p $out/bin
                  cp ${self'.packages.default}/bin/${self'.packages.default.pname} $out/bin
                '';
              };
            };
          devShells.default =
            pkgs.mkShell.override
              {
                stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
              }
              {
                inherit nativeBuildInputs buildInputs env;
                name = "rust";
                # Used for development and testing
                packages = with pkgs; [
                  typos
                  gnumake
                  clippy
                  cargo-machete
                  process-compose
                  cargo-watch
                  nodePackages.typescript-language-server
                  vscode-langservers-extracted
                ];
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
