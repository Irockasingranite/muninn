{ mkShell, muninn }:

mkShell {
  inputsFrom = [ muninn ];

  shellHook = ''
    export XDG_DATA_DIRS=$XDG_DATA_DIRS:$GSETTINGS_SCHEMAS_PATH
  '';
}
