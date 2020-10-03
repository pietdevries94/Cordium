{ pkgs ? import ./nix/pkgs.nix {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gtk3
    glib
    glib-networking
    libsoup
    webkitgtk
    pkg-config

    rustc
    cargo
    niv
    gnome3.glade
  ];
}
