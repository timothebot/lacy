{
  description = "Fast magical cd alternative for lazy terminal navigators";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flakelight.url = "github:nix-community/flakelight";
    home-manager.url = "github:nix-community/home-manager";
  };

  outputs =
    { flakelight, ... }:
    flakelight ./. (
      { config, ... }:
      {
        package =
          {
            lib,
            fetchFromGitHub,
            rustPlatform,
            nix-update-script,
            ...
          }:
          let
            manifest = (lib.importTOML ./Cargo.toml).package;
          in
          rustPlatform.buildRustPackage (finalAttrs: {
            pname = manifest.name;
            version = manifest.version;

            src = ./.;

            passthru.updateScript = nix-update-script { };

            cargoHash = "sha256-rNTRcQJptVi/ruCd56oHHN9n+Z3NhUNyrvXf27Sovtw=";

            meta = {
              description = "Fast magical cd alternative for lacy terminal navigators";
              homepage = "https://github.com/timothebot/lacy";
              platforms = lib.platforms.all;
              license = lib.licenses.mit;
              mainProgram = "lacy";
              maintainers = with lib.maintainers; [ ];
            };
          });

        homeModule =
          {
            config,
            lib,
            pkgs,
            ...
          }:
          with lib;
          let
            cfg = config.programs.lacy;
          in
          {
            options.programs.lacy = {
              enable = mkEnableOption "lacy - magical cd alternative";

              package = mkOption {
                type = types.package;
                default = pkgs.lacy;
                description = "The lacy package to use.";
              };
            };

            config = mkIf cfg.enable {
              programs.bash.initExtra = mkIf config.programs.bash.enable ''
                eval "$(${cfg.package}/bin/lacy init bash)"
              '';

              programs.zsh.initContent = mkIf config.programs.zsh.enable ''
                eval "$(${cfg.package}/bin/lacy init zsh)"
              '';
            };
          };

        devShell = {
          packages = pkgs: with pkgs; [ cargo rustc ];
        };

        app = { lacy, ... }: "${lacy}/bin/lacy";

        systems = [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ];
      }
    );
}
