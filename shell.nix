let
  pins = import ./npins;
  pkgs = import pins.nixpkgs { };
  packages = import ./deps.nix pkgs;
in
pkgs.mkShellNoCC {
  inherit packages;
}
