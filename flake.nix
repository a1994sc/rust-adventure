{
  description = "Rust playground";

  inputs = {
    # keep-sorted start block=yes case=no
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # keep-sorted end
  };

  outputs =
    {
      self,
      fenix,
      nixpkgs,
      flake-utils,
      treefmt-nix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        f-core = fenix.packages.${system};
        f-wasm =
          with fenix.packages.${system};
          combine [
            stable.toolchain
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
        treefmtEval = treefmt-nix.lib.evalModule pkgs (
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
          f-core.stable.toolchain
          pkg-config
          # Use mold for faster linking
          mold
          clang
        ];
        buildInputs = with pkgs; [
          openssl
        ];
        env = {
          ROCKET_CLI_COLORS = "false";
          CARGO_LINKER = "clang";
          CARGO_RUSTFLAGS = "-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      in
      {
        packages = rec {
          default = rust-testing;
          rust-testing = pkgs.rustPlatform.buildRustPackage {
            pname = "rust-testing";
            inherit nativeBuildInputs buildInputs env;
            inherit ((pkgs.lib.importTOML ./Cargo.toml).package) version;
            src = ./.;
            cargoBuildFlags = "-p rust-testing";

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs env;
          name = "rust";
          # Used for development and testing
          packages = with pkgs; [
            process-compose
            cargo-watch
            nodePackages.typescript-language-server
            vscode-langservers-extracted
          ];
        };
        formatter = treefmtEval.config.build.wrapper;
      }
    );
}
