use crate::{
    cmd::{Complete, Run},
    query::resolve_query,
};

impl Run for Complete {
    fn run(&self) {
        println!(
            "{}",
            resolve_query(&self.path.as_str())
                .iter()
                .map(|path_buf| path_buf.display().to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
