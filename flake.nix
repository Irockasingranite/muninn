{
  description = "A flake for building and working with muninnn";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; };

  outputs = { self, nixpkgs, flake-utils }:
    (

      flake-utils.lib.eachDefaultSystem (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in {
          formatter = pkgs.nixfmt;
          packages.default = pkgs.callPackage ./muninn.nix { };
          devShells.default = pkgs.callPackage ./shell.nix { };
        }));
}
