let
  pins = import ./nix/npins;
  pkgs = import pins.nixpkgs { };
  lib = pkgs.lib;
in
pkgs.mkShellNoCC {
  packages = with pkgs; [
    llvmPackages_19.libcxxClang
    nixfmt-rfc-style
    rustup
    protobuf_21
    just
    git
    (lib.optionals stdenv.isLinux [
      nodejs_23
      corepack_23
      pkg-config
      glib.dev
      gobject-introspection
      at-spi2-atk
      atkmm
      cairo
      gdk-pixbuf
      glib
      gtk3
      harfbuzz
      librsvg
      libsoup_3
      pango
      webkitgtk_4_1
      openssl
    ])
  ];
}
