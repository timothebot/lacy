use clap::{Parser, Subcommand};

use crate::{init::shell_config, query::resolve_query, ui};

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
            Commands::Prompt { path, return_all } => {
                let mut query = path.as_str();
                if query.ends_with("/") {
                    let mut chars = query.chars();
                    chars.next_back();
                    query = chars.as_str();
                }
                if query.trim().is_empty() {
                    println!("~");
                    return;
                }
                let results = resolve_query(query);
                match results.len() {
                    0 => {}
                    1 => {
                        println!("{}", results.first().unwrap().display());
                    }
                    _ => {
                        let paths = results
                            .iter()
                            .map(|path_buf| path_buf.display().to_string())
                            .collect::<Vec<String>>();
                        if return_all {
                            println!("{}", paths.join("\n"));
                            return;
                        }
                        if let Some(selected) = ui::select("Multiple possibilities found!", paths) {
                            println!("{}", selected);
                        }
                    }
                };
            }
            Commands::Init {
                shell,
                cd_cmd,
                cmd,
                custom_fuzzy,
            } => {
                println!(
                    "{}",
                    match shell_config(shell.as_str(), cd_cmd, cmd, custom_fuzzy) {
                        Ok(config) => config,
                        Err(err) => format!("An error occurred: {}", err),
                    }
                );
            }
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
    Prompt {
        path: String,

        /// Returns all result separated by \n instead of showing selector ui
        ///
        /// This is allows you to integrate a custom fuzzy tool if wanted
        #[arg(long)]
        return_all: bool,
    },
    Init {
        /// Currently supported shells: bash, fish, zsh
        shell: String,

        /// Allows you to specify another command than cd, e.g. z
        #[arg(long, default_value = "cd")]
        cd_cmd: String,

        /// Define what alias the lacy command has
        #[arg(long, default_value = "y")]
        cmd: String,

        /// What fuzzy tool should be used for cases where lacy finds multiple
        /// matching folders. If not specified, lacy will use a custom UI.
        #[arg(long)]
        custom_fuzzy: Option<String>,
    },
    Complete {
        path: String,
    },
}
