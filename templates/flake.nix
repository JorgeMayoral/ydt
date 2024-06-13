{
  description = "Basic Nix DevShell Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
        # Add your devShell dependencies here
      nativeBuildInputs = with pkgs; {{packages}};

      # Add your shellHook commands here
      shellHook = ''
      '';
    };
  };
}
