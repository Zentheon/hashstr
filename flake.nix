{
  inputs = {
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      naersk,
      ...
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        # Build the Rust toolchain using rust-coolchain.toml
        # Trying to build after modifying the toolchain will give the updated sha256
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "D8GkGd+MrvPyB+dY94Sa8T/znAESyCg8P3OQ30X83AM=";
        };

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        # You can define your own scripts in this file
        scripts = import ./.dev/scripts.nix { inherit pkgs; };
      in
      {
        # `nix build` & `nix run`:
        # https://github.com/nix-community/naersk#buildpackages-parameters
        packages.dingus = naersk'.buildPackage {
          src = ./.;
        };
        # Try to be helpful in case someone happens to try installing as a package
        packages.default = throw "`hashstr` is a Rust library, not a program!";

        # `nix develop` or `use flake` (direnv)
        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cocogitto
              cargo-llvm-cov
              cargo-cyclonedx
              wild
              gcc16 # can be removed in the future; gcc 15 doesn't support wild
            ];
            buildInputs = [
              toolchain
              scripts.bumpCrateScript # used by cog.toml
              scripts.bumpDepScript # used by cog.toml
            ];
            shellHook = "export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS=-Clink-arg=-fuse-ld=wild";
          };
          # Shell profile including only what's necessary for ci jobs
          ci = pkgs.mkShell {
            packages = with pkgs; [
              cargo-cyclonedx
            ];
            buildInputs = [
              toolchain
            ];
          };
        };
      }
    );
}
