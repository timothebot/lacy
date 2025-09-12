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
          rustPlatform.buildRustPackage (finalAttrs: {
            pname = "lacy";
            version = "0.3.0";

            src = ./.;

            passthru.updateScript = nix-update-script { };

            # Remove in 0.3.1 once tests do not rely on folders
            doCheck = false;

            cargoHash = "sha256-N5avoN3QCCYMF29Cvbwha+iBAXPncOttWxGpVZ70EqI=";

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
          packages =
            pkgs: with pkgs; [
              python314
            ];
        };

        app = { lacy, ... }: "${lacy}/bin/lacy";

        systems = [
          "x86_64-linux"
          "aarch64-linux"
          "i686-linux"
          "armv7l-linux"
        ];

      }
    );
}
 