{ sources ? import ./nix/sources.nix}:

let
  niv = (import sources.niv {}).niv;
  pkgs = import sources.nixpkgs {};
in pkgs.mkShell {
  buildInputs = with pkgs; [
    niv
  ];
}