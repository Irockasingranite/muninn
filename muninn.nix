{ lib, rustPlatform, cmake, pkg-config, gtk3, glibc, wrapGAppsHook }:

rustPlatform.buildRustPackage {
  pname = "muninn";
  version = "1.1.0";

  src = ./.;

  cargoHash = "sha256-9aFsDxouaQcRqa6hZ5HBPQNO01j3v2lpfoY9dk/YzxY=";

  nativeBuildInputs = [ cmake pkg-config wrapGAppsHook ];

  buildInputs = [ gtk3 glibc ];

  meta = with lib; {
    description = "A 1D timeseries visualization tool";
    homepage = "https://git.tpi.uni-jena.de/srenkhoff/muninn";
    license = licenses.mit;
  };
}
