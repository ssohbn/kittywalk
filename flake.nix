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
      buildInputs = [
        (pkgs.rust-bin.stable.latest.default.override {
          targets = [ "arm-unknown-linux-gnueabi" "x86_64-unknown-linux-gnu" ];
        })

        # need these for hidapi
        pkgs.libusb1
        pkgs.pkg-config
      ];
    };
  };
}
