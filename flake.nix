{
  description = "roblox-rs: Rust to Luau compiler + Lune standalone runtime";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            lune
            cargo
            rustc
            rustfmt
            clippy
          ];
          shellHook = ''
            echo "roblox-rs dev shell"
            echo "  lune $(lune --version)"
            echo "  cargo $(cargo --version)"
            echo "  Run: lune run scripts/monster_gyroscope"
          '';
        };
      });
}
