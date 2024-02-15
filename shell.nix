{ mkShell, cargo, rustc, cmake, pkg-config, glibc, gtk3, librsvg }:

mkShell {
  packages = [ cmake cargo rustc pkg-config glibc gtk3 librsvg ];

  shellHook = ''
    export XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH
  '';
}
