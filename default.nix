{ pkgs, ... }:
pkgs.rustPlatform.buildRustPackage {
  pname = "cg_ws_25";
  version = "0.0.1";

  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  nativeBuildInputs = with pkgs; [ cmake pkg-config freetype expat fontconfig ];
  buildInputs = with pkgs; [ fontconfig libcbc ];
}
