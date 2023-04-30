{
  description = "nix-shell-locked";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in rec {
        packages.nix-shell-locked = pkgs.callPackage ./default.nix { };
        defaultPackage = packages.nix-shell-locked;
        devShell = with pkgs;
          mkShell rec {
            buildInputs = [
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" "rust-analysis" ];
              })
              rust-analyzer
              cargo-watch
            ];

            # Allows rust-analyzer to find the rust source
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

            # Without this graphical frontends can't find the GPU adapters
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";

          };
      });
}
