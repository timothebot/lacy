use std::path::PathBuf;

use crate::{
    cmd::{Prompt, Run},
    directory::get_current_directory,
    query::Query,
    ui,
};

impl Run for Prompt {
    fn run(&self) {
        let query = Query::from(self.query.clone());

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
                let paths = results
                    .iter()
                    .map(|path_buf| path_buf.display().to_string())
                    .collect::<Vec<String>>();
                if self.return_all {
                    println!("{}", paths.join("\n"));
                    return;
                }
                if let Some(selected) = ui::select("Multiple possibilities found!", paths) {
                    println!("{}", selected);
                }
            }
        };
    }
}
