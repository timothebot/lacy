use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct LacyCli {
    #[command(subcommand)]
    pub command: LacyCommand,
}

#[derive(Subcommand, Debug)]
pub enum LacyCommand {
    Prompt(Prompt),
    Init(Init),
    Complete(Complete),
}

#[derive(Debug, Parser)]
pub struct Prompt {
    pub path: String,

    /// Returns all result separated by \n instead of showing selector ui
    ///
    /// This is allows you to integrate a custom fuzzy tool if wanted
    #[arg(long)]
    pub return_all: bool,
}

#[derive(Debug, Parser)]
pub struct Init {
    /// Currently supported shells: bash, fish, zsh
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
pub struct Complete {
    pub path: String,
}
