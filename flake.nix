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
          hello-R = let manifest = (importTOML ./hello-R/Cargo.toml).package;
          in buildRustPackage {
            inherit (manifest) version;
            pname = manifest.name;
            cargoLock.lockFile = ./hello-R/Cargo.lock;
            src = cleanSource ./hello-R;
            # src = filter { root = ./hello-R; };
          };
        };
    in {
      packages = forEachSystem (system:
        let pkgs = import nixpkgs { inherit system; };
        in rec {
          devenv-up = self.devShells.${system}.default.config.procfileScript;
          inherit (pkgFor pkgs) hello-R;
          default = hello-R;
        });

      devShells = forEachSystem (system:
        let
          pkgs = import nixpkgs { inherit system; };
          inherit (pkgFor pkgs) hello-R;
        in with pkgs;
        with lib; {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [{
              packages = [ hello-R ];
              languages = { rust.enable = true; };

              enterShell = ''
                ${hello-R}/bin/hello-R 24 8 16
              '';

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
