{
  description = "Computational Geometry";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, treefmt-nix }:

    let
      system = "x86_64-linux";
      treefmtEval = treefmt-nix.lib.evalModule pkgs
        {
          # Used to find the project root
          projectRootFile = "flake.nix";

          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };

      pkgs = import nixpkgs {
        inherit system;
      };

      cg_ss_25 = pkgs.callPackage ./default.nix { };

      apps = builtins.listToAttrs (
        builtins.map
          (
            name: {
              inherit name;
              value = {
                type = "app";
                program = "${self.packages.${system}.default}/bin/${name}";
              };
            }
          )
          [
            "main"
            "p01"
            "p01_unique_cases"
            "p02"
            "p03"
          ]
      );
    in
    {
      packages.${system}.default = cg_ss_25;

      apps.${system} = apps;

      devShells.${system} = {
        default = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              treefmtEval.config.build.wrapper

              cargo
              clippy
              gcc
              qhull
              rust-analyzer
              rustc
              rustfmt
            ];
        };
      };

      formatter.${system} = treefmtEval.config.build.wrapper;
    };
}
