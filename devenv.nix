{ pkgs, ... }:

{
  packages = [
    pkgs.cocogitto
    pkgs.cargo-llvm-cov
    pkgs.cargo-cyclonedx
  ];

  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "nightly";
    components = [
      "rustc"
      "rust-src"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "miri"
      "llvm-tools-preview"
    ];

    # FIXME: doesn't work? (05-09-2026)
    # Toolchain can be configured via devenv itself, but it makes more sense to use the native way.
    # toolchainFile = ./rust-toolchain.toml;

    # Fast Rust-written linker.
    wild.enable = true;
    # Very fast codegen backend. Rust support is experimental and requires nightly.
    cranelift.enable = true;
  };
}
