use crate::{
    cmd::{Prompt, Run},
    query::resolve_query,
    ui,
};

impl Run for Prompt {
    fn run(&self) {
        let mut query = self.path.as_str();
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
            0 => {}
            1 => {
                println!("{}", results.first().unwrap().display().to_string());
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
