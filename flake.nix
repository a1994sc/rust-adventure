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
      in
      {
        packages = rec {
          default = rust-testing;
          rust-testing = pkgs.rustPlatform.buildRustPackage {
            pname = "rust-testing";
            version = (pkgs.lib.importTOML ./Cargo.toml).package.version;
            src = ./.;
            cargoBuildFlags = "-p rust-testing";

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };
        devShells = {
          default = pkgs.mkShell {
            name = "rust";
            packages = with pkgs; [
              f-core.stable.toolchain
              cargo-watch
              nodePackages.typescript-language-server
              vscode-langservers-extracted
            ];
          };
          wasm = pkgs.mkShell {
            name = "rust-wasm";
            packages = with pkgs; [
              f-wasm
              nodejs_22
              wasm-pack
              cargo-watch
              llvmPackages.bintools
              nodePackages.typescript-language-server
              vscode-langservers-extracted
            ];
            env = {
              CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
            };
          };
        };
        formatter = treefmtEval.config.build.wrapper;
      }
    );
}
