{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{self, ... }: {

    # not proud of this formatting
    devShells.x86_64-linux.default
    = let
    pkgs = import inputs.nixpkgs {
      system = "x86_64-linux";
      overlays = [ (import inputs.rust-overlay) ];
    };

    in pkgs.mkShell {

      # need these for hidapi and pretty sure i put them in nativeBuildInputs

      nativeBuildInputs = [
        pkgs.libusb1
        pkgs.pkg-config
      ];

      buildInputs = [
        (pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-gnu" ];
        })
      ];
    };

    
    # for the pi
    devShells.aarch64-linux.default
    = let
    pkgs = import inputs.nixpkgs {
      system = "aarch64-linux";
      overlays = [ (import inputs.rust-overlay) ];
    };

    in pkgs.mkShell {

      # need these for hidapi and pretty sure i put them in nativeBuildInputs
      nativeBuildInputs = [
        pkgs.libusb1
        pkgs.pkg-config
      ];

      buildInputs = [
        (pkgs.rust-bin.stable.latest.default.override {
          targets = [ "arm-unknown-linux-gnueabi"];
        })
      ];
    };

  };
}
