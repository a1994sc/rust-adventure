{
  description = "Rust playground";

  inputs = {
    # keep-sorted start block=yes case=no
    fenix = {
      url = "git+https://github.com/nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "rust-analyzer-src";
    };
    flake-parts = {
      url = "git+https://github.com/hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nixpkgs.url = "git+https://github.com/nixos/nixpkgs?ref=nixpkgs-unstable";
    process-compose-flake.url = "git+https://github.com/Platonic-Systems/process-compose-flake";
    rust-analyzer-src = {
      flake = false;
      url = "git+https://github.com/rust-lang/rust-analyzer?ref=refs/tags/nightly";
    };
    services-flake.url = "git+https://github.com/juspay/services-flake";
    systems.url = "git+https://github.com/nix-systems/default";
    treefmt-nix = {
      url = "git+https://github.com/numtide/treefmt-nix";
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
          f-core = inputs.fenix.packages.${system};
          f-wasm =
            with inputs.fenix.packages.${system};
            combine [
              stable.toolchain
              targets.wasm32-unknown-unknown.stable.rust-std
            ];
          f-zarf =
            with inputs.fenix.packages.${system};
            combine [
              stable.toolchain
              targets.x86_64-unknown-linux-gnu.stable.rust-std
            ];
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
            f-zarf
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
            # RUSTFLAGS = CARGO_RUSTFLAGS;
          };
        in
        {
          packages =
            let
              inherit ((pkgs.lib.importTOML ./Cargo.toml).package) version name;
            in {
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
                      version
                      ;
                    env = env // {
                      RUSTFLAGS = "-C relocation-model=static -C strip=symbols";
                    };
                    src = ./.;
                    cargoBuildFlags = "-p ${name}";
                    cargoLock.lockFile = ./Cargo.lock;
                  };
              # link: https://fasterthanli.me/series/building-a-rust-service-with-nix/part-11
              image = pkgs.dockerTools.buildImage {
                name = "ghcr.io/a1994sc/rust/" + self'.packages.default.pname;
                # https://discourse.nixos.org/t/passing-git-commit-hash-and-tag-to-build-with-flakes/11355/2
                tag = version + "-" + (if (self ? shortRev) then self.shortRev else "dirty");
                copyToRoot = (pkgs.runCommand "project" {} "cp -r ${self'.packages.default}/bin $out");
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
              # services.redis-cluster."cluster1".enable = true;
              services.redis."redis".enable = true;
            };
        };
    };
}
