{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
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

            # Utils
            just
            minify
          ];

          LD_LIBRARY_PATH = libPath;
        };
      });
}
