use std::{env, fs};

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
            let mut paths = query
                .results(get_current_directory().as_path())
                .iter()
                .filter_map(|(path, score)| {
                    Some((
                        // Resolve symlinks, basically - so we can then
                        // deduplicate paths that lead to the same directory
                        fs::canonicalize(path)
                            .ok()
                            // Only leave directories and not files after symlinks
                            .filter(|p| p.is_dir())?,
                        // Store original path for display purposes
                        path.to_str()?.to_string(),
                        *score,
                    ))
                })
                .collect::<Vec<_>>();
            paths.sort_unstable_by(|(path_a, _, _), (path_b, _, _)| {
                // sort by path for later deduplication
                path_a.cmp(path_b)
            });
            paths.dedup_by(|(path_a, _, _), (path_b, _, _)| path_a == path_b);
            paths.sort_unstable_by(|(_, name_a, score_a), (_, name_b, score_b)| {
                // now that we've deduplicated it we can sort it for display
                // first by score descending
                score_b
                    .cmp(score_a)
                    // then by name ascending
                    .then(name_a.cmp(name_b))
            });
            paths
                .into_iter()
                .map(|(_, name, _)| name)
                .collect::<Vec<_>>()
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
