{
  description = "Beatsaber rich presence";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        craneLib = crane.mkLib pkgs;
        src = ./.;
        commonArgs = {
          inherit (craneLib.crateNameFromCargoToml { src = ./BSRichPresence; }) pname version;
          inherit src;
          strictDeps = true;
          buildInputs = [
            pkgs.openssl
          ];
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
        };
        app = craneLib.buildPackage (
          commonArgs
          // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          }
        );
      in
      {
        packages = rec {
          inherit app;
          default = app;

        };
        apps = rec {
          app = flake-utils.lib.mkApp { drv = self.packages.${system}.app; };
          default = app;
        };
        devShells.default = craneLib.devShell { };
      }
    );

}
