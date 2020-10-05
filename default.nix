{ 
  pkgs ? import ./nix/pkgs.nix {},
  cargoSha256 ? "0fxbrlvfxyy2xy1kviz1xd09qpw5cm1974qpafr182xb2w0pbwhi"
}:

let
  desktopItem = pkgs.makeDesktopItem rec {
    name = "cordium";
    exec = name;
    comment = "A messaging app in GTK and Rust";
    desktopName = "Cordium";
    categories = "Network;";
  };
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "cordium";
  version = "0.1.0-dev";

  src = pkgs.nix-gitignore.gitignoreSource [] ./.;

  buildInputs = with pkgs; [
    gtk3
    glib
    glib-networking
    libsoup
    webkitgtk
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  inherit cargoSha256;
  verifyCargoDeps = true;

  postInstall = ''
    ln -s ${desktopItem}/share $out/share
  '';

  meta = with pkgs.stdenv.lib; {
    description = "Cordium is a messaging app in GTK and Rust";
    homepage = https://github.com/pietdevries94/Cordium;
    license = licenses.mit;
    platforms = platforms.linux;
  };
}