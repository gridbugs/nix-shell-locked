{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
, rustPlatform ? pkgs.rustPlatform
}:
rustPlatform.buildRustPackage rec {
  pname = "nix-shell-locked";
  version = "0.2.0";

  src = ./.;

  cargoSha256 = "sha256-v8T56xDj1PG0dDHD4d8ngDQ8C/DGVtYKsZPQnXGt1Rg=";

  meta = with lib; {
    homepage = "https://github.com/gridbugs/nix-shell-locked";
    description = "Wrapper of `nix shell` that reads a lockfiles to get the nixpkgs revision to use when sourcing packages to install in transient shell";
    license = licenses.mit;
  };
}
