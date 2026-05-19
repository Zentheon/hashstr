{ pkgs, ... }:

{
  packages = [
    pkgs.cocogitto
    pkgs.cargo-llvm-cov
    pkgs.cargo-cyclonedx
    pkgs.toml-cli
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

  scripts.bump-crate.exec = ''
    if [ -z "$1" ]; then
        echo "Usage: $0 <path> <semver>"
        echo "Example: $0 1.2.0 */Cargo.toml"
        exit 1
    fi
    VER=$1
    shift 1

    for cargo_toml_path in $@; do
        if [ -f "$cargo_toml_path" ]; then
          echo "Bumping version of '$cargo_toml_path' to: $VER"
          new_toml=$(toml set $cargo_toml_path package.version $VER)
          echo "$new_toml" > "$cargo_toml_path"
        fi
    done
  '';
  scripts.bump-dep.exec = ''
    if [ -z "$1" ]; then
        echo "Usage: $0 <path> <package> <semver>"
        echo "Example: $0 slick_dependency 0.7.1 */Cargo.toml"
        exit 1
    fi
    PACKAGE=$1
    VER=$2
    shift 2

    for cargo_toml_path in $@; do
        if [ -f "$cargo_toml_path" ]; then
          if ! toml get $cargo_toml_path dependencies.$PACKAGE.version > /dev/null 2>&1; then
              continue # We don't want to add the field if it doesn't exist
          fi
          echo "Bumping dependency '$PACKAGE' of '$cargo_toml_path' to: $VER"
          new_toml=$(toml set $cargo_toml_path dependencies.$PACKAGE.version $VER)
          echo "$new_toml" > "$cargo_toml_path"
        fi
    done
  '';
}
