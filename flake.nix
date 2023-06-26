# Based on https://github.com/nix-community/naersk/blob/master/examples/multi-target/flake.nix
{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = inputs:
    with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) { inherit system; };
        code = pkgs.callPackage ./. { inherit nixpkgs system rust-overlay; };
        libPath = with pkgs;
          lib.makeLibraryPath [ libGL xorg.libX11 xorg.libXi ];
      in rec {
        devShell = with pkgs;
          mkShell {

          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            cargo-outdated

            libGL
            xorg.libX11
            xorg.libXi
          ];

          LD_LIBRARY_PATH = libPath;
        };
      });
}
