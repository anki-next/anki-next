{
  stdenv,
  lib,
  n2,
  rustPlatform,
}:
rustPlatform.buildRustPackage rec {
  pname = "anki";
  version = "0.0.0";
  src = ../anki;
  useFetchCargoVendor = true;
  cargoHash = lib.fakeHash;
}
