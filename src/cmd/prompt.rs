use std::{collections::HashMap, env, fs};

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

        let paths = {
            let map = query
                .results(get_current_directory().as_path())
                .iter()
                .filter_map(|path| {
                    Some((
                        // Resolve symlinks, basically - so we can then
                        // deduplicate paths that lead to the same directory
                        fs::canonicalize(path)
                            .ok()
                            // Only leave directories and not files after symlinks
                            .filter(|p| p.is_dir())?,
                        // Store original path for display purposes
                        path.to_str().map(|s| s.to_string())?,
                    ))
                })
                // Collect this to a hashmap as it intristically deduplicates keys
                .collect::<HashMap<_, _>>();
            // Then return sorted user-displayed strings
            let mut v = map.into_values().collect::<Vec<_>>();
            v.sort_unstable();
            v
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
