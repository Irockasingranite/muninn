let pkgs = import <nixpkgs> { };
in { muninn = pkgs.callPackage ./muninn.nix { }; }
