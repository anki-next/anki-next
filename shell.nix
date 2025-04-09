let
  pins = import ./npins;
  pkgs = import pins.nixpkgs { };
in
pkgs.mkShellNoCC {
  packages = with pkgs; [
    nixfmt-rfc-style
    nodejs_23
    corepack_23
    rustup
    protobuf_21
  ];
}
