{ pkgs, ... }:
pkgs.rustPlatform.buildRustPackage {
  pname = "cg_ws_25";
  version = "0.0.1";

  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
