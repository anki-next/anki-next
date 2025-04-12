let
  pins = import ./npins;
  pkgs = import pins.nixpkgs { };
  packages = import ./nix/deps.nix pkgs;
in
pkgs.callPackage ./nix/anki.nix { }
