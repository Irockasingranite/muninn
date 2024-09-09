{ lib, rustPlatform, cmake, pkg-config, gtk4, wrapGAppsHook }:

rustPlatform.buildRustPackage {
  pname = "muninn";
  version = "2.0.0";

  src = ./.;

  cargoHash = "sha256-/x5NkiCZM2wCiuN5MJE8r3Zi0zmDyjKEBYfOm+u4uFE=";

  nativeBuildInputs = [ cmake pkg-config wrapGAppsHook ];

  buildInputs = [ gtk4 ];

  meta = with lib; {
    description = "A 1D timeseries visualization tool";
    homepage = "https://git.tpi.uni-jena.de/srenkhoff/muninn";
    license = licenses.mit;
  };
}
