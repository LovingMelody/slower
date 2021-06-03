{ pkgs ? import <nixpkgs> { } }:
with pkgs;

{
  slower = rustPlatform.buildRustPackage rec {
    pname = "slower";
    version = "0.2.0";
    src = ./.;

    cargoSha256 = "1yafy23gg0zx6x04s064lb31vlxi9gs5j530ibzqyv65bnxn6bgv";

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
