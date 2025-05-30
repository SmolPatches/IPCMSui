{
  description = "sui devshell using overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    sui-overlay.url = "github:SmolPatches/suichain-overlay?ref=feat/mac-support";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    sui-overlay,
    flake-utils,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin"];
    systemOutputs = flake-utils.lib.eachSystem systems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      devnet = sui-overlay.lib.mkSuiRelease pkgs {
        # new package for sui 1.49.0
        type = "testnet"; # testnet devnet mainnet
        version_number = "1.49.1";
        vhash = "sha256-ZTr0+0KLYrCQtt1OZSPiSwHRLO0Ukbel5SUzZFOKti8=";
      };
    in {
      packages.install = devnet; # package to install this devnet version outside of flake
      devShells.default = pkgs.mkShell {
        name = "sui-dev-shell";
        packages = [
          devnet
          pkgs.git
          pkgs.zsh
        ];
        
        shellHook = ''
          # export SHELL=${pkgs.zsh}/bin/zsh
        '';
      };
      formatter = pkgs.alejandra;
    });
  in
    systemOutputs;
}
