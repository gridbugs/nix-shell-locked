{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
, rustPlatform ? pkgs.rustPlatform
}:
rustPlatform.buildRustPackage rec {
  pname = "nix-shell-locked";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "sha256-eeYFJqFWKlJgB9UK/j4+SH8yaXKRwI9AyqwXWLBrHjs=";

  meta = with lib; {
    homepage = "https://github.com/gridbugs/nix-shell-locked";
    description = "Wrapper of `nix shell` that reads a lockfiles to get the nixpkgs revision to use when sourcing packages to install in transient shell";
    license = licenses.mit;
  };
}
