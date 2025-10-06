use crate::{
    cmd::{Complete, Run},
    directory::get_current_directory,
    query::Query,
};

impl Run for Complete {
    fn run(&self) {
        let query = Query::from(self.query.clone());
        println!(
            "{}",
            query
                .completions(get_current_directory().as_path())
                .iter()
                .filter_map(|path_buf| {
                    if self.basename {
                        Some(path_buf.file_name()?.display().to_string())
                    } else {
                        Some(path_buf.display().to_string())
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
