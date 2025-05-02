{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    let
      overlay = final: prev: {
        kibadda = (prev.kibadda or { }) // {
          pinentry = final.pkgs.rustPlatform.buildRustPackage {
            name = "pinentry";
            cargoHash = "sha256-s/A62R+bPif4zqq0YuvqMRRdsmIvDRMaw7j07tW4P1k=";
            src = self;
            meta.mainProgram = "pinentry-minimal-server";
          };
        };
      };

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    flake-utils.lib.eachSystem supportedSystems (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            overlay
          ];
        };
      in
      {
        packages = rec {
          default = pinentry;
          inherit (pkgs.kibadda) pinentry;
        };

        devShells = {
          default = pkgs.mkShell {
            name = "pinentry-development-shell";
            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
            ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
        };
      }
    )
    // {
      overlays.default = overlay;
    };
}
