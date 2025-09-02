{
  description = "My Rust project as a Nix flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustPlatform = pkgs.rustPlatform;
      in {
        packages.default = rustPlatform.buildRustPackage {
          pname = "jotter";
          version = "0.4.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          doCheck = false;

          # optional if you have tests/examples that need extra deps
          nativeBuildInputs = [ pkgs.sqlite ];
          buildInputs = [ pkgs.sqlite ];
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/jotter";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [ pkgs.rustc pkgs.cargo ];
        };
      });
}
