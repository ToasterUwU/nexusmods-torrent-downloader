{
  description = "Use `nix develop` to temp install everything you need for working on this project";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            openssl
            pkg-config

            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rustfmt" "rustc-dev" ];
              targets = [ "aarch64-unknown-linux-gnu" "x86_64-pc-windows-gnu" "x86_64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin" ];
            })
            rust-analyzer

            python3
            python3Packages.nodriver
          ];

          # shellHook = ''
          # '';
        };
      }
    );
}
