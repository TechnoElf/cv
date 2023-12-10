{
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-23.05;
    flake-utils.url = github:numtide/flake-utils;
  };

  outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (currentSystem:
    let
      pkgs = import nixpkgs {
        system = currentSystem;
      };
    in with pkgs; {
      devShell = mkShell rec {
        nativeBuildInputs = [];
      };

      packages.default = mkDerivation {};
    }
  );
}
