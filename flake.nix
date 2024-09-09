{
  description = "A flake for building and working with muninnn";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; };

  outputs = { self, nixpkgs, flake-utils }:
    (

      flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          muninn = pkgs.callPackage ./muninn.nix { };

        in {
          formatter = pkgs.nixfmt;
          packages.muninn = muninn;
          packages.default = muninn;
          devShells.default = pkgs.callPackage ./shell.nix { inherit muninn; };
        }));
}
