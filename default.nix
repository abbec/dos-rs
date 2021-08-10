let
  sources = import ./nix/sources.nix;
  pkgs = with
      {
        overlay = _: pkgs:
          {
            niv = (import sources.niv {}).niv;
          };
      };
    import sources.nixpkgs
      {
        overlays = [
          overlay
          (import sources.rust)
        ];

        config = { };
      };
in
pkgs.stdenv.mkDerivation {
  name = "dos-rs";
  nativeBuildInputs = [
    (pkgs.rust-bin.nightly."2021-08-09".default.override {
      extensions = [ "rust-src" ];
    })
    pkgs.dosbox
    pkgs.nasm
  ];

  WATCOM=pkgs.open-watcom-bin;
  CARGO_BUILD_TARGET="dos.json";
}
