{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
, rustPlatform ? pkgs.rustPlatform
}:
rustPlatform.buildRustPackage rec {
  pname = "nix-shell-locked";
  version = "0.2.1";

  src = ./.;

  cargoSha256 = "sha256-Z534AwnhKojbAA+w2+n1k4FE6cEzSfAsXHMbarN4IgE=";

  meta = with lib; {
    homepage = "https://github.com/gridbugs/nix-shell-locked";
    description = "Wrapper of `nix shell` that reads a lockfiles to get the nixpkgs revision to use when sourcing packages to install in transient shell";
    license = licenses.mit;
  };
}
