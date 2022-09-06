{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      buildInputs = with pkgs; [
        vulkan-loader
        wayland
        wayland-protocols
        libxkbcommon
      ];
      rustNightly = pkgs.rust-bin.nightly.latest.default.override {
        extensions = ["rust-src"];
      };

      # NB: we don't need to overlay our custom toolchain for the *entire*
      # pkgs (which would require rebuidling anything else which uses rust).
      # Instead, we just want to update the scope that crane will use by appending
      # our specific toolchain there.
      craneLib = (crane.mkLib pkgs).overrideToolchain rustNightly;
      my-crate = craneLib.${system}.buildPackage {
        src = ./.;
        inherit buildInputs;

        nativeBuildInputs = with pkgs; [
          rustNightly
          pkg-config
          gtk-layer-shell
          gtk3
        ];
      };
    in {
      checks = {
        inherit my-crate;
      };

      packages.default = my-crate;

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;
        nativeBuildInputs = [rustNightly];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        # Extra inputs can be added here
      };
    });
}
