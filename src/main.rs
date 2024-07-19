use clap::{Parser, Subcommand};
use init::get_shell_config;
use prompt::get_matching_path;

mod init;
mod prompt;

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
    Complete,
}

fn main() {
    let cli = LacyCli::parse();

    match cli.command {
        Commands::Prompt { path } => {
            let args: Vec<String> = path.split(' ').map(|s| s.to_string()).collect();
            get_matching_path(&args);
        }
        Commands::Init { shell } => {
            get_shell_config(shell.as_str())
        }
        Commands::Complete => {
            println!("TODO Complete");
        }
    }
}
