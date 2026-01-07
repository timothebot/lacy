use clap::{Parser, Subcommand};

const HELP_TEMPLATE: &str = "
 _     _   ____ __
| |   / \\ / _\\ V /
| |_ | o ( (_ \\ / 
|___||_n_|\\__||_| 

v{version}
https://github.com/timothebot/lacy

{about}{before-help}

{usage-heading}
{tab}{usage}

{all-args}{after-help}
";

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    override_usage="lacy init --help",
    help_template=HELP_TEMPLATE
)]
pub struct LacyCli {
    #[command(subcommand)]
    pub command: LacyCommand,
}

#[derive(Subcommand, Debug)]
#[command(version, help_template=HELP_TEMPLATE)]
pub enum LacyCommand {
    /// Return all paths matching the given query.
    Prompt(Prompt),

    /// Generate the shell configuration
    Init(Init),

    /// Get shell completions for the given query.
    Complete(Complete),
}

#[derive(Debug, Parser)]
#[command(version, help_template=HELP_TEMPLATE)]
pub struct Prompt {
    pub query: String,

    /// Returns all result separated by \n instead of showing selector ui
    ///
    /// This is allows you to integrate a custom fuzzy tool if want
    #[arg(long)]
    pub return_all: bool,
}

#[derive(Debug, Parser)]
#[command(
    version,
    help_template=HELP_TEMPLATE,
    after_help = "
To get started, you must include lacy in your shell config.
You can find more about why this is required in the docs.

If you don't know what shell you are using, run:
$ echo $SHELL

ZSH:
$ echo \"eval \\\"\\$(lacy init zsh)\\\"\" >> ~/.zshrc

Bash:
$ echo \"eval \\\"\\$(lacy init bash)\\\"\" >> ~/.bashrc

Fish:
$ echo \"lacy init fish | source\" >> ~/.config/fish/config.fish"

PowerShell:
$ echo \"lacy init powershell | Out-String | iex\" >> $PROFILE"
)]
pub struct Init {
    /// Currently supported shells: bash, fish, zsh, powershell
    pub shell: String,

    /// Allows you to specify another command than cd, e.g. z
    #[arg(long, default_value = "cd")]
    pub cd_cmd: String,

    /// Define what alias the lacy command has
    #[arg(long, default_value = "y")]
    pub cmd: String,

    /// What fuzzy tool should be used for cases where lacy finds multiple
    /// matching folders. If not specified, lacy will use a custom UI.
    #[arg(long)]
    pub custom_fuzzy: Option<String>,
}

#[derive(Debug, Parser)]
#[command(version, help_template=HELP_TEMPLATE)]
pub struct Complete {
    #[arg(default_value = "")]
    pub query: String,

    /// Return only the names of the folder instead of the whole path
    #[arg(long)]
    pub basename: bool,
}
