{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flakelight.url = "github:nix-community/flakelight";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {
    self,
    nixpkgs,
    flakelight,
    crane,
    fenix,
  }: let
    inherit (nixpkgs) lib;
    selectToolchain = fenix: fenix.complete;
    mkCrane = pkgs: let
      inherit (selectToolchain pkgs.fenix) toolchain;
      craneLib = (crane.mkLib nixpkgs.legacyPackages.${pkgs.system}).overrideToolchain toolchain;
      rawSrc = ./.;
      src = lib.fileset.toSource {
        root = rawSrc;
        fileset = lib.fileset.unions [
          ./.config/hakari.toml
          ./advent-hack
          ./advent_core
          ./src
          ./macros
          ./utils
          ./years
          ./Cargo.lock
          ./Cargo.toml
        ];
      };
      commonArgs = {
        inherit src;
        strictDeps = true;
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      individualCrateArgs =
        commonArgs
        // {
          inherit cargoArtifacts src;
          inherit (craneLib.crateNameFromCargoToml {inherit src;}) version;
          # NB: we disable tests since we'll run them all via cargo-nextest
          doCheck = false;
        };
      buildCrate = pname:
        craneLib.buildPackage (
          individualCrateArgs
          // {
            inherit pname;
            cargoExtraArgs = "-p ${pname}";
          }
        );
    in {
      inherit
        craneLib
        src
        commonArgs
        cargoArtifacts
        individualCrateArgs
        buildCrate
        ;
    };
    years = builtins.attrNames (lib.filterAttrs (_: v: v == "directory") (builtins.readDir ./years));
    allCrates =
      (builtins.map (y: "y_${y}") years)
      ++ [
        "advent"
        "advent_core"
        "utils"
        "macros"
      ];
    forAllCrates = lib.genAttrs' allCrates;
  in
    flakelight ./. {
      lib = {
        inherit years allCrates;
      };
      nixpkgs.overlays = [fenix.overlays.default];
      flakelight.builtinFormatters = false;
      formatters = pkgs: let
        alejandra = "${pkgs.lib.getExe pkgs.alejandra} .";
        rustfmt = "${(selectToolchain pkgs.fenix).rustfmt}/bin/rustfmt .";
        taplo = "${pkgs.lib.getExe pkgs.taplo} fmt .";
      in {
        "*.nix" = alejandra;
        "*.rs" = rustfmt;
        "*.toml" = taplo;
      };
      packages =
        forAllCrates (name: {
          inherit name;
          value = pkgs: (mkCrane pkgs).buildCrate name;
        })
        // {
          default = pkgs: (mkCrane pkgs).buildCrate "advent";
        };
      checks = pkgs: let
        inherit
          (mkCrane pkgs)
          src
          commonArgs
          craneLib
          cargoArtifacts
          ;
      in
        forAllCrates (name: {
          name = "clippy-${name}";
          value = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -p ${name} -- --deny warnings";
            }
          );
        })
        // forAllCrates (name: {
          name = "nextest-${name}";
          value = craneLib.cargoNextest (
            commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
              cargoNextestPartitionsExtraArgs = "--no-tests=pass -p ${name}";
            }
          );
        })
        // {
          advent-hakari = craneLib.mkCargoDerivation {
            inherit src;
            pname = "advent-hakari";
            cargoArtifacts = null;
            doInstallCargoArtifacts = false;

            buildPhaseCargoCommand = ''
              cargo hakari generate --diff  # workspace-hack Cargo.toml is up-to-date
              cargo hakari manage-deps --dry-run  # all workspace crates depend on workspace-hack
              cargo hakari verify
            '';

            nativeBuildInputs = [
              pkgs.cargo-hakari
            ];
          };
        };
      devShell = pkgs:
        (mkCrane pkgs).craneLib.devShell {
          checks = self.checks.${pkgs.system};

          packages = with pkgs; [cargo-hakari];
        };
    };
}
