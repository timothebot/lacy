use std::{env, fs, path::PathBuf};

use dialoguer::console::Term;

use crate::{
    cmd::{Prompt, Run},
    directory::get_current_directory,
    query::Query,
    ui,
};

impl Run for Prompt {
    fn run(&self) {
        let query = Query::from(self.query.clone());

        if query.parts().is_empty() {
            println!(
                "{}",
                env::var("LACY_NO_ARGS_PATH").unwrap_or(String::from("~"))
            );
            return;
        }

        /*
        _ if first_query_part.starts_with("-")
                && !first_query_part
                    .strip_prefix("-")
                    .unwrap_or_default()
                    .contains("-") =>
            {
                if let Ok(number) = first_query_part
                    .strip_prefix("-")
                    .unwrap_or_default()
                    .parse::<i32>()
                {

                }
                get_current_directory()
            }
             */

        let results: Vec<PathBuf> = query.results(get_current_directory().as_path());
        match results.len() {
            0 => {}
            1 => {
                println!("{}", results.first().unwrap().display());
            }
            _ => {
                let paths = {
                    let mut tmp = results
                        .iter()
                        .map(|path| {
                            // Canonicalize the paths to see if we have two different paths
                            // pointing to the same location
                            fs::canonicalize(path)
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string()
                        })
                        .collect::<Vec<_>>();
                    tmp.sort();
                    tmp.dedup();
                    tmp
                };

                if paths.len() == 1 {
                    println!("{}", paths.first().unwrap());
                } else if self.return_all {
                    println!("{}", paths.join("\n"));
                } else {
                    // Prevents cursor from being hidden when canceling the selection.
                    // See https://github.com/timothebot/lacy/issues/58.
                    _ = ctrlc::set_handler(move || {
                        let term = Term::stderr();
                        _ = term.show_cursor();
                        std::process::exit(1);
                    });

                    if let Some(selected) = ui::select("Multiple possibilities found!", &paths) {
                        println!("{selected}");
                    }
                }
            }
        }
    }
}
