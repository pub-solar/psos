with import <nixpkgs> {}; stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustup
    pkgconfig
    stdenv
    ncurses
  ];
}
