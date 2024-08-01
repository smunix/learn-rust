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
        with rustPlatform;
        let
          mk = name:
            let manifest = (importTOML (./. + "/${name}/Cargo.toml")).package;
            in {
              "${name}" = buildRustPackage {
                inherit (manifest) version;
                pname = manifest.name;
                cargoLock.lockFile = ./. + "/${name}/Cargo.lock";
                src = cleanSource (./. + "/${name}");
                nativeBuildInputs = [ pkg-config ];
                bulidInputs = [ fontconfig freetype zlib zstd ];
              };
            };
        in {
          pkgs-rs = attrsets.foldAttrs (drv: s: drv // s) { } (lists.map mk [
            "actix-gcd"
            "hello-R"
            "mandelbrot"
            "progress"
            "csv-serde"
            "json-serde"
            "sort-algos"
            "trie-me"
            "plot-me"
            "huffman-coding"
          ]);
        };
    in {
      packages = forEachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };
        in rec {
          devenv-up = self.devShells.${system}.default.config.procfileScript;
          default = (pkgFor pkgs).pkgs-rs.sort-algos;
        } // ((pkgFor pkgs).pkgs-rs));

      devShells = forEachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };
        in with pkgs;
        with lib; {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = let inherit (pkgFor pkgs) pkgs-rs;
            in [{
              packages = [ cargo cargo-expand cargo-watch cargo-limit ]
                ++ (attrsets.mapAttrsToList (_: id) pkgs-rs);
              languages = { rust.enable = true; };

              enterShell = with pkgs-rs; ''
                ${hello-R}/bin/hello-R 24 8 16
              '';

              scripts = with pkgs;
                (attrsets.mapAttrs' (name: prj: {
                  name = "${name}-tree";
                  value = { exec = "${nix-tree}/bin/nix-tree ${prj}"; };
                }) pkgs-rs) // (attrsets.mapAttrs' (name: prj: {
                  name = "${name}-loop";
                  value = {
                    exec = ''
                      pushd ${name}
                      cargo-watch -c -w . -x run
                      popd
                    '';
                  };
                }) pkgs-rs) // (attrsets.mapAttrs' (name: prj: {
                  name = "${name}-test";
                  value = {
                    exec = ''
                      pushd ${name}
                      # github.com/watchexec/watchexec/issues/76
                      cargo-watch -c -w . -x "test -- --nocapture"
                      popd
                    '';
                  };
                }) pkgs-rs);
              processes.hello.exec = with pkgs-rs; "${hello-R}/bin/hello-R";
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
