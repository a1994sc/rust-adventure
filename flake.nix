{
  inputs = {
    # keep-sorted start block=yes case=no
    cargo-doc-live.url = "github:srid/cargo-doc-live";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # keep-sorted end
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
        inputs.process-compose-flake.flakeModule
        inputs.cargo-doc-live.flakeModule
      ];
      perSystem =
        {
          config,
          self',
          pkgs,
          lib,
          ...
        }:
        {
          rust-project.crates."rust-testing".crane.args = {
            buildInputs = lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [ IOKit ]);
          };

          # Add your auto-formatters here.
          # cf. https://nixos.asia/en/treefmt
          treefmt.config = {
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
          };

          devShells.default = pkgs.mkShell {
            inputsFrom = [
              self'.devShells.rust
              config.treefmt.build.devShell
            ];
            packages = [
              pkgs.cargo-watch
              config.process-compose.cargo-doc-live.outputs.package
            ];
          };
          packages.default = self'.packages.rust-nix-template;
        };
    };
}
