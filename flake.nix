{
  inputs = {
    # nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    nixpkgs.url = "github:nixos/nixpkgs?ref=master";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";

    nix-filter.url = "github:numtide/nix-filter";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  nixConfig = {
    extra-trusted-public-keys =
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, ... }@inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
      pkgFor = pkgs:
        with inputs.nix-filter.lib;
        with pkgs;
        with lib;
        with rustPlatform; {
          mandelbrot =
            let manifest = (importTOML ./mandelbrot/Cargo.toml).package;
            in buildRustPackage {
              inherit (manifest) version;
              pname = manifest.name;
              cargoLock.lockFile = ./mandelbrot/Cargo.lock;
              src = cleanSource ./mandelbrot;
              # src = filter { root = ./mandelbrot; };
            };
          hello-R = let manifest = (importTOML ./hello-R/Cargo.toml).package;
          in buildRustPackage {
            inherit (manifest) version;
            pname = manifest.name;
            cargoLock.lockFile = ./hello-R/Cargo.lock;
            src = cleanSource ./hello-R;
            # src = filter { root = ./hello-R; };
          };
          actix-gcd =
            let manifest = (importTOML ./actix-gcd/Cargo.toml).package;
            in buildRustPackage {
              inherit (manifest) version;
              pname = manifest.name;
              cargoLock.lockFile = ./actix-gcd/Cargo.lock;
              src = cleanSource ./actix-gcd;
              # src = cleanSourceWith {
              #   src = ./actix-gcd;
              #   filter = path: type: true;
              # };
            };
        };
    in {
      packages = forEachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (inputs.rust-overlay.overlays.default) ];
          };
        in rec {
          devenv-up = self.devShells.${system}.default.config.procfileScript;
          inherit (pkgFor pkgs) hello-R actix-gcd mandelbrot;
          default = mandelbrot;
        });

      devShells = forEachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (inputs.rust-overlay.overlays.default) ];
          };
          inherit (pkgFor pkgs) actix-gcd hello-R mandelbrot;
        in with pkgs;
        with lib; {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [{
              packages = [ actix-gcd hello-R mandelbrot ];
              languages = { rust.enable = true; };

              enterShell = ''
                ${hello-R}/bin/hello-R 24 8 16
              '';

              scripts = with pkgs; {
                actix-gcd-tree.exec = "${nix-tree}/bin/nix-tree ${actix-gcd}";
                mandelbrot-tree.exec = "${nix-tree}/bin/nix-tree ${mandelbrot}";
              };
              processes.hello.exec = "${hello-R}/bin/hello-R";
              pre-commit.hooks = {
                # fourmolu.enable = true;
                # stylish-haskell.enable = true;
                nixfmt.enable = true;
              };
            }];
          };
        });
    };
}
