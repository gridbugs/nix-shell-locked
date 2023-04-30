{
  description = "nix-shell-locked";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, flake-compat, ... }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs {
      inherit system ;
    };
  in rec {
    packages.nix-shell-locked = pkgs.callPackage ./default.nix {};
    defaultPackage = packages.nix-shell-locked;
  }
  );
}
