{
  description = "Helper to build rust packages on nixos";
  inputs = {
    nixpkgs = { url = "nixpkgs/nixos-23.05"; };
    systems = {
      url = "path:./flake.systems.nix";
      flake = false;
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # TODO: crane
    flake-parts = { url = "github:hercules-ci/flake-parts"; };
  };

  /* In case of error 'intellij-rust-native-helper' in intellij idea, see: https://github.com/intellij-rust/intellij-rust/issues/8197
     i.e. run something like:
     nix shell nixpkgs#patchelf -c patchelf --set-interpreter $(nix eval --raw nixpkgs#glibc)/lib64/ld-linux-x86-64.so.2 ~/.local/share/JetBrains/IntelliJIdea2023.1/intellij-rust/bin/linux/x86-64/intellij-rust-native-helper
     (after every upgrade of rust plugin !)
  */

  outputs = inputs@{ flake-parts, fenix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      perSystem = { config, self', inputs', pkgs, lib, system, ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          #toolchain = fenix.packages.${system}.default.toolchain;
          toolchain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            #sha256 = lib.fakeSha256;
            sha256 = "sha256-U2yfueFohJHjif7anmJB5vZbpP7G6bICH4ZsjtufRoU=";
          };
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
          rustSymlinkedToolchain = pkgs.symlinkJoin {
            name = "rust-symlinked-toolchain";
            paths = [ toolchain ];
          };

          nativeBuildInputs = [ pkgs.pkg-config pkgs.stdenv.cc ];
          buildInputs = [ pkgs.openssl ];

        in {

          devShells = {
            default = pkgs.mkShell {
              # nix develop .#default
              nativeBuildInputs = [
                rustSymlinkedToolchain
                # just using 'toolchain' is not good enough for intellij
                # (intellij also needs 'rust-src' in rust-toolchain.toml)
                nativeBuildInputs
                # for development-only
                pkgs.cargo-watch
              ];
              inherit buildInputs;
            };
          };

          packages = {
            default = rustPlatform.buildRustPackage {
              inherit (cargoToml.package) name version;
              inherit nativeBuildInputs;
              inherit buildInputs;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };
          };

        };
    };
}
