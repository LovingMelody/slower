{ pkgs ? import <nixpkgs> { } }:
with pkgs;

{
  slower = rustPlatform.buildRustPackage rec {
    pname = "slower";
    version = "0.1.1";
    src = ./.;

    cargoSha256 = "sha256-dhnAtVuabbjR+/kEgRK3s25a0jKTYACho49NZvtquEM=";

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
