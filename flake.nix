{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell rec {
          SKIA_NINJA_COMMAND = "${pkgs.ninja}/bin/ninja";
          SKIA_GN_COMMAND = "${pkgs.gn}/bin/gn";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib/libclang.so";
          # nativeBuildInputs = with pkgs; [ rustc cargo ];
          nativeBuildInputs = with pkgs; [ wayland ninja fontconfig libiconv clang pkg-config ];
          buildInputs = with pkgs; [ fontconfig freetype wayland libxkbcommon libGL xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi openssl gn ];
          shellHook = ''
            export CC="${pkgs.clang}/bin/clang"
            export CXX="${pkgs.clang}/bin/clang++"
            export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
            export RUST_BACKTRACE=1
            export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.lib.concatMapStringsSep ":" (inp: "${inp.out}/lib") buildInputs}
          '';
        };
      }
    );
}
