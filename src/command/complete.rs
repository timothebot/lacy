use crate::command::prompt::get_matching_path;

fn list_dirs(path: &str) -> String {
    let mut dirs = String::new();
    let entries = std::fs::read_dir(path);
    if entries.is_err() {
        return dirs;
    }
    let entries = entries.unwrap();
    for entry in entries.into_iter() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            dirs.push_str(&format!("{} ", entry.file_name().to_str().unwrap()));
        }
    }
    dirs.trim_end().to_string()
}

pub fn complete_path(path: &str) -> String {
    let mut args: Vec<String> = path.split(' ').map(|s| s.to_string()).collect();
    let last_arg = &args.pop().unwrap();
    if !last_arg.starts_with('/') {
        return String::new();
    }
    let current_path = get_matching_path(&args);
    // find all directories in current_path
    let mut possible_dirs = list_dirs(&current_path);
    if last_arg == "/" {
        return possible_dirs;
    }
    let last_arg = last_arg.trim_start_matches('/').to_lowercase();
    // filter out directories that don't match the last element
    possible_dirs = possible_dirs
        .split(' ')
        .filter(|line| line.to_lowercase().starts_with(&last_arg))
        .collect::<Vec<&str>>()
        .join(" ");
    return possible_dirs;
}
