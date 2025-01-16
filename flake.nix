{
  description = "A Nix flake for perfmode (Fan/Performance Control for ASUS TUF Gaming laptops)";

  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
    in
    utils.lib.eachDefaultSystem (system:
      if system in supportedSystems then
        let
          pkgs = import nixpkgs { inherit system; };
          naersk-lib = pkgs.callPackage naersk { };
        in
        {
          defaultPackage = naersk-lib.buildPackage ./.;
          devShell = with pkgs; mkShell {
            buildInputs = [
              pkgs.cargo
              pkgs.rustc
              pkgs.rustfmt
              pkgs.pre-commit
              pkgs.rustPackages.clippy
            ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
        }
      else
        { }
    );
}

