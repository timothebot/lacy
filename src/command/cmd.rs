use clap::{Parser, Subcommand};

use crate::command::{complete::complete_path, init::get_shell_config, query::resolve_query};

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
                println!(
                    "{:?}",
                    resolve_query(path.strip_suffix("/").unwrap_or(""))
                );
            }
            Commands::Init { shell } => get_shell_config(shell.as_str()),
            Commands::Complete { path } => {
                println!(
                    "{}",
                    resolve_query(path.as_str())
                        .iter()
                        .map(|path_buf| path_buf.display().to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
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
