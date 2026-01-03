{
  pkgs,
  lib,
  config,
  system,
  ...
}:
let
  cosmocc = pkgs.stdenvNoCC.mkDerivation rec {
    pname = "cosmocc-bin";
    version = "4.0.2";
    src = pkgs.fetchzip {
      url = "https://cosmo.zip/pub/cosmocc/cosmocc-${version}.zip";
      hash = "sha256-6KZv7KU2rJhwfc9k6z9I6ZdfIS1KqRFLZUo8YyuD7ZY=";
      stripRoot = false;
    };
    phases = [ "installPhase" ];
    installPhase = ''
      mkdir -p $out
      cp -r $src/* $out/
    '';
  };
in
{
  languages.rust = {
    enable = true;
    channel = "stable";
    # rustflags = "-C target-feature=+crt-static -C strip=symbols";
    mold.enable = true;
    targets = [
      "aarch64-unknown-linux-gnu"
      "x86_64-unknown-linux-gnu"
      "aarch64-unknown-linux-musl"
      "x86_64-unknown-linux-musl"
    ];
  };

  env = {
    COSMO = "${cosmocc}";
    CARGO_LINKER = "${pkgs.clang_18}/bin/clang";
  };

  packages = [
    cosmocc
  ];

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
