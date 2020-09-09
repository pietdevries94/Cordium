{ sources ? import ./nix/sources.nix}:

let
  niv = (import sources.niv {}).niv;
  pkgs = import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    latest.rustChannels.stable.rust
    niv
  ];
}