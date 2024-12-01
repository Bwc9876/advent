{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    forAllSystems = nixpkgs.lib.genAttrs [
      "aarch64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
      "x86_64-linux"
    ];
    pkgsFor = system: import nixpkgs {inherit system;};
  in {
    packages =
      forAllSystems
      (system: let
        pkgs = pkgsFor system;
      in {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "advent";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      });
    devShells = forAllSystems: (
      system: let
        pkgs = pkgsFor system;
      in {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            just
          ];
        };
      }
    );
  };
}
