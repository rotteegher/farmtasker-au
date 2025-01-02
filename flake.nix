{
  description = "A devshell flake for the website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    devenv,
    systems,
    ...
  } @ inputs: let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages =
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
        leptos = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-leptos";
          version = "f03212fa370744c123865c8fccb4068a07d8da7d";

          src = pkgs.fetchFromGitHub {
            owner = "leptos-rs";
            repo = pname;
            rev = "${version}";
            hash = "sha256-KrcQ5DLaUK5vl7SamY6Tw53bOczs2BtCDpaDLbuytwg=";
          };

          cargoHash = "sha256-gaGcKw/zqhiP7LRHTtApGeg7QQe8yDlFVXyjGGTGznQ=";

          buildFeatures = [ "no_downloads" ]; # cargo-leptos will try to install missing dependencies on its own otherwise
          doCheck = false; # Check phase tries to query crates.io
        };
      in {
        # default = pkgs.callPackage ./. {};
        default =
        let
          toolchain = inputs.fenix.packages.${system}.combine [
            inputs.fenix.packages.${system}.minimal.rustc
            inputs.fenix.packages.${system}.minimal.cargo
            inputs.fenix.packages.${system}.targets.wasm32-unknown-unknown.latest.rust-std
          ];
          pkgs = nixpkgs.legacyPackages.${system};
        in

        (pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        }).buildRustPackage {
          pname = "farmtasker-au";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          doCheck = false;

          nativeBuildInputs = [
            leptos

            pkgs.binaryen
            pkgs.dart-sass
          ];

          buildPhase = ''
            mkdir -p $out
            cargo leptos --version
            echo "Building farmtasker-au..."
            cargo leptos build --release
          '';

          installPhase = ''
            mkdir -p $out
            ls -la $out

            sed -i 's|site-root = "target/site"|site-root = "site"|' Cargo.toml
            cp -r target/release/farmtasker-au $out/
            cp -r target/site $out/
            cp -r Cargo.toml $out/
            ls -la $out
          '';

          meta = {
            mainProgram = "../farmtasker-au";
          };
          shellHook = ''
            export LEPTOS_SITE_ADDR="0.0.0.0:8080"
            export LEPTOS_SITE_ROOT="site"
          '';
        };
      });
    devShells =
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
        leptos = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-leptos";
          version = "f03212fa370744c123865c8fccb4068a07d8da7d";

          src = pkgs.fetchFromGitHub {
            owner = "leptos-rs";
            repo = pname;
            rev = "${version}";
            hash = "sha256-KrcQ5DLaUK5vl7SamY6Tw53bOczs2BtCDpaDLbuytwg=";
          };

          cargoHash = "sha256-gaGcKw/zqhiP7LRHTtApGeg7QQe8yDlFVXyjGGTGznQ=";

          buildFeatures = [ "no_downloads" ]; # cargo-leptos will try to install missing dependencies on its own otherwise
          doCheck = false; # Check phase tries to query crates.io
        };
        buildInputs = with pkgs; [
          # Cli
          leptos

          bacon
          cargo-binutils
          cargo-watch
          cargo-shuttle
          cargo-generate
          dart-sass
          leptosfmt
          nodePackages.svelte-language-server
          leptosfmt
          trunk
          binaryen

          sqlx-cli

          # Lib
          openssl
          libclang
          hidapi
          pkg-config
          alsa-lib
          udev
          clang
          lld
        ];
      in {
        default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              # https://devenv.sh/reference/options/
              dotenv.disableHint = true;

              packages = buildInputs;

              languages.javascript = {
                enable = true;
                corepack.enable = true;
                npm = {
                  enable = true;
                  install.enable = true;
                };
              };

              services.nginx.enable = true;

              languages.typescript.enable = true;

              languages.rust = {
                enable = true;
                channel = "nightly";
                toolchain = {
                  rustc = pkgs.rustc-wasm32;
                };
                targets = ["wasm32-unknown-unknown"];
              };

              env = {
                RUST_BACKTRACE = 1;
                LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH";
                XDG_DATA_DIRS = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS";
              };
            }
          ];
        };
      });
  };
}
