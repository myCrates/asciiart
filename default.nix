with import <nixpkgs> {}; {
  rustEnv = stdenv.mkDerivation {
    name = "asciiart";
    buildInputs = [ stdenv rustc cargo openssl ];
    shellHook =
      ''
      '';
  };
}
