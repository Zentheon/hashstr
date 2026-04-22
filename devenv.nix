{ pkgs, ... }:

{
  packages = [
    pkgs.openssl
    pkgs.cocogitto
  ];

  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "nightly";

    components = [ "rustc" "rust-src" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
