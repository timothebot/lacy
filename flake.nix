{
    description = "Fast magical cd alternative for lazy terminal navigators";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ flake-parts, nixpkgs, fenix, utils, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];

      flake = {
        homeManagerModules.default = { config, lib, pkgs, ... }:
          with lib;
          let
            cfg = config.programs.lacy;
          in
          {
            options.programs.lacy = {
              enable = mkEnableOption "lacy - magical cd alternative";

              package = mkOption {
                type = types.package;
                default = inputs.self.packages.${pkgs.system}.default;
                description = "The lacy package to use.";
              };
            };

            config = mkIf cfg.enable {
              programs.bash.initExtra = mkIf config.programs.bash.enable ''
                eval "$(${cfg.package}/bin/lacy init zsh)"
              '';

              programs.zsh.initContent = mkIf config.programs.zsh.enable ''
                eval "$(${cfg.package}/bin/lacy init zsh)"
              '';
            };
          };
      };

      perSystem = { config, self', inputs', pkgs, system, manifest, ... }:
      let
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
       in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = manifest.name;
            version = manifest.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = ./.;
            nativeBuildInputs = with pkgs; [
                pkg-config
            ];
          meta = with pkgs.lib; {
            description = "Fast magical cd alternative for lazy terminal navigators";
            homepage = "https://github.com/timothebot/lacy";
            license = licenses.mit;
            maintainers = [ ];
            platforms = platforms.unix;
          };
        };
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.python314
            fenix.packages.${system}.stable.toolchain
          ];
        };

        apps.default = {
          type = "app";
          program = "${self'.packages.default}/bin/lacy";
        };
      };
    };
}
