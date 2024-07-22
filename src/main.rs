use clap::{Parser, Subcommand};
use complete::complete_path;
use init::get_shell_config;
use prompt::get_matching_path;

mod complete;
mod init;
mod prompt;
mod ui;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct LacyCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Prompt { path: String },
    Init { shell: String },
    Complete { path: String },
}

fn main() {
    let cli = LacyCli::parse();

    match cli.command {
        Commands::Prompt { path } => {
            let args: Vec<String> = path.split(' ').map(|s| s.to_string()).collect();
            println!("{}", get_matching_path(&args));
        }
        Commands::Init { shell } => get_shell_config(shell.as_str()),
        Commands::Complete { path } => {
            println!("{}", complete_path(path.as_str()));
        }
    }
}
