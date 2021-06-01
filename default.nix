{ pkgs ? import <nixpkgs> { } }:
with pkgs;

{
  slower = rustPlatform.buildRustPackage rec {
    pname = "slower";
    version = "0.1";
    src = ./.;

    cargoSha256 = "sha256-SmIheV4rg15JeJCRta0qJ8v7SP0z8XB5JzlKvzq53DY=";

    nativeBuildInputs = [ ];
    buildInputs = [ ];

    meta = with lib; {
      description = "Rate limit stdout";
      longDescription = "Rate limit stdout output to make logs readable";
      homepage = "https://github.com/fuzen-py/slower";
      license = licenses.mit;
      maintainers = with maintainers; [ fuzen ];
    };
  };
}
