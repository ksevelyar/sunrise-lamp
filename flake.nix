{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    nixpkgs-unstable,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
        librepcb = nixpkgs-unstable.legacyPackages.${system}.librepcb;
      in {
        devShell = pkgs.mkShell {
          SSID = "ssid";
          PASS = "pass";
          TIMEZONE = "Europe/Moscow";
          SCHEDULE = "00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40";
          UPDATE_INTERVAL = "1";
          WIFI_TRANSMIT_POWER = "19";
          DEFMT_LOG = "info,embassy_net=warn";

          buildInputs = with pkgs; [
            (rust-bin.nightly.latest.default.override {
              extensions = ["rust-src"];
              targets = ["riscv32imc-unknown-none-elf"];
            })

            (writeShellScriptBin "ci" ''
              set -euo pipefail
              cargo build --release
              cargo fmt --all -- --check --color always
              cargo clippy --all-features --workspace -- -D warnings
            '')

            (writeShellScriptBin "open-schematic" ''
              set -euo pipefail
              project="$PWD/doc/hardware/librepcb/sunrise-lamp.lpp"
              if [[ ! -f "$project" ]]; then
                echo "Run open-schematic from the repository root" >&2
                exit 1
              fi
              exec ${librepcb}/bin/librepcb "$project"
            '')

            (writeShellScriptBin "render-schematic" ''
              set -euo pipefail
              project="$PWD/doc/hardware/librepcb/sunrise-lamp.lpp"
              if [[ ! -f "$project" ]]; then
                echo "Run render-schematic from the repository root" >&2
                exit 1
              fi
              export QT_QPA_PLATFORM=offscreen
              exec ${librepcb}/bin/librepcb-cli open-project --run-job 'Schematic SVG' --outdir "$PWD/doc/hardware" "$project"
            '')

            librepcb
            probe-rs-tools
            rust-analyzer
          ];
        };
      }
    );
}
