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
        overlays = [
          (final: prev: {
            inherit libcbc;
          })
        ];
      };

      osi-clp = pkgs.stdenv.mkDerivation {
        pname = "osi-clp";
        version = "999";

        src = pkgs.fetchFromGitHub {
          owner = "alicevision";
          repo = "osi_clp";
          rev = "b4ef9610c2ac058bf508ee9574511726c5bc5a1e";
          hash = "sha256-0Sz4/7CRKrArIUy/XxGIP7WMmICqDJ0VxZo62thChYQ=";
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        buildInputs = with pkgs; [
          cgl
          clp
          coin-utils
          osi
        ];

        #   meta = {
        # description = "";
        # homepage = "";
        # license = lib.licenses.;
        # maintainers = with lib.maintainers;
        # [  ];
        # };
      };

      libcbc = pkgs.stdenv.mkDerivation {
        pname = "libcbc";
        version = "2.10.12";

        src = pkgs.fetchFromGitHub {
          owner = "coin-or";
          repo = "Cbc";
          rev = "782d275a8efa569eb818551561b424948e19653c";
          hash = "sha256-0Sz4/7CRKrArIUy/XxGIP7WMmICqDJ0VxZo62thChYQ=";
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        buildInputs = with pkgs; [
          cgl
          clp
          coin-utils
          osi
          osi-clp
        ];

        #   meta = {
        # description = "";
        # homepage = "";
        # license = lib.licenses.;
        # maintainers = with lib.maintainers;
        # [  ];
        # };
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
            "p04"
            "p05"
          ]
      );
    in
    {
      packages.${system} = {
        inherit cg_ss_25 osi-clp libcbc;
        default = cg_ss_25;
      };

      apps.${system} = apps;

      devShells.${system} = {
        default = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              treefmtEval.config.build.wrapper

              cargo
              clippy
              gcc
              osi
              osi-clp
              pkg-config
              fontconfig
              qhull
              rust-analyzer
              rustc
              rustfmt
            ] ++ [
              self.packages.${system}.default
            ];
        };
      };

      formatter.${system} = treefmtEval.config.build.wrapper;
    };
}
