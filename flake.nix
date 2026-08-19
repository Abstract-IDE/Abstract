{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rust = pkgs.rust-bin.stable."1.91.1".default;
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            rust
            pkgs.pkg-config
            pkgs.openssl
            pkgs.luajit
          ];

          PKG_CONFIG_PATH = pkgs.lib.makeSearchPath "lib/pkgconfig" [
            pkgs.openssl.dev
            pkgs.luajit
          ];
        };
      });
}
