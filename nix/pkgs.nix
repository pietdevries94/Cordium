{ sources ? import ./sources.nix}:

let
  niv = self: super: (import sources.niv {});
in
  (import sources.nixpkgs { 
    overlays = [
      niv
    ];
  })