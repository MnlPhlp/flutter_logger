{
description = "Flutter 3.13.x";
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
  flake-utils.url = "github:numtide/flake-utils";
};
outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        config = {
          allowUnfree = true;
        };
      };
    in
    {
      devShell =
        with pkgs; mkShell rec {
          buildInputs = [
            flutter
          ];
        };
    });
}
