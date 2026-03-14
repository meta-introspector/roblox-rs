{
  description = "gist-bridge: pastebin → GitHub Gist forwarder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        gist-bridge = pkgs.writeShellApplication {
          name = "gist-bridge";
          runtimeInputs = with pkgs; [ gh jq curl coreutils gnused gnugrep ];
          text = builtins.readFile ./scripts/gist-bridge.sh;
        };
      in {
        packages.default = gist-bridge;
        packages.gist-bridge = gist-bridge;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ lune cargo rustc rustfmt clippy gh jq curl ];
          shellHook = ''
            echo "roblox-rs dev shell"
            echo "  lune $(lune --version)"
            echo "  cargo $(cargo --version)"
            echo "  gh $(gh --version | head -1)"
            echo "  Run: lune run scripts/monster_gyroscope"
            echo "  Run: gist-bridge watch"
          '';
        };

        nixosModules.gist-bridge = { config, lib, pkgs, ... }: {
          options.services.gist-bridge = {
            enable = lib.mkEnableOption "gist-bridge pastebin to gist forwarder";
            user = lib.mkOption { type = lib.types.str; default = "mdupont"; };
            spoolDir = lib.mkOption { type = lib.types.str; default = "/mnt/data1/spool/uucp/pastebin"; };
          };
          config = lib.mkIf config.services.gist-bridge.enable {
            systemd.services.gist-bridge = {
              description = "Gist Bridge — pastebin to GitHub Gist forwarder";
              after = [ "network.target" ];
              wantedBy = [ "multi-user.target" ];
              serviceConfig = {
                Type = "simple";
                ExecStart = "${gist-bridge}/bin/gist-bridge watch";
                Restart = "on-failure";
                RestartSec = 30;
                User = config.services.gist-bridge.user;
              };
            };
          };
        };
      });
}
