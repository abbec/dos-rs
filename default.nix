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

  nightlyDate = "2021-09-27";

  rustBin = (pkgs.rust-bin.nightly."${nightlyDate}".default.override {
      extensions = [ "rust-src" ];
    });
  rustAnalyzer = pkgs.rust-bin.nightly."${nightlyDate}".rust-analyzer-preview;

  # rust-analyzer cannot handle symlinks
  # so we need to create a derivation with the
  # correct rust source without symlinks
  rustSrcNoSymlinks = pkgs.stdenv.mkDerivation {
    name = "rust-src-no-symlinks";

    rustWithSrc = (rustBin.override {
      extensions = [ "rust-src" ];
    });
    rust = rustBin;

    builder = builtins.toFile "builder.sh" ''
      source $stdenv/setup
      mkdir -p $out
      cp -r -L $rustWithSrc/lib/rustlib/src/rust/library/. $out/
      '';
  };
in
pkgs.stdenv.mkDerivation {
  name = "dos-rs";
  nativeBuildInputs = [
    rustBin
    pkgs.dosbox
    pkgs.nasm
  ] ++ pkgs.lib.optional pkgs.lib.inNixShell rustAnalyzer;

  WATCOM=pkgs.open-watcom-bin;

  shellHook=''
    run() {
      make run
    }
  '';
}
