use clap::{Parser, Subcommand};

use crate::command::{complete::complete_path, init::get_shell_config, prompt::get_matching_path};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct LacyCli {
    #[command(subcommand)]
    command: Commands,
}

impl LacyCli {
    pub fn run() {
        let cli = LacyCli::parse();
        match cli.command {
            Commands::Prompt { path } => {
                let args: Vec<String> = path.split(' ').map(|s| s.to_string()).collect();
                println!("{}", get_matching_path(&args, true));
            }
            Commands::Init { shell } => get_shell_config(shell.as_str()),
            Commands::Complete { path } => {
                println!("{}", complete_path(path.as_str()));
            }
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    Prompt { path: String },
    Init { shell: String },
    Complete { path: String },
}
