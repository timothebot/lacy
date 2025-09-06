use clap::{Parser, Subcommand};

use crate::{init::get_shell_config, query::resolve_query, ui};

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
                let mut query = path.as_str();
                if query.ends_with("/") {
                    let mut chars = query.chars();
                    chars.next_back();
                    query = chars.as_str();
                }
                if query.trim().len() == 0 {
                    println!("~");
                    return;
                }
                let results = resolve_query(query);
                match results.len() {
                    0 => {},
                    1 => {
                        println!("{}", results.first().unwrap().display().to_string());
                    },
                    _ => {
                        println!("{}", ui::select(
                            "Multiple possibilities found!",
                            results
                                .iter()
                                .map(|path_buf| path_buf.display().to_string())
                                .collect::<Vec<String>>()
                        ));
                    }
                };
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
