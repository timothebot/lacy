use std::env;
use std::fs;
use std::path::PathBuf;

fn find_matching_path(elements: Vec<String>, current_path: PathBuf) -> Option<PathBuf> {
    let mut path = current_path;

    for element in elements {
        let element = element.to_lowercase();
        let dirs: Vec<_> = fs::read_dir(&path)
            .ok()?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if entry.file_type().ok()?.is_dir() {
                    Some(entry.file_name())
                } else {
                    None
                }
            })
            .collect();

        if dirs.len() == 1 {
            path.push(&dirs[0]);
        } else {
            let matches: Vec<_> = dirs
                .iter()
                .filter(|d| d.to_string_lossy().to_lowercase().contains(&element))
                .collect();

            if let Some(matched_dir) = matches.first() {
                path.push(matched_dir);
            } else {
                eprintln!("Error: No matching directory found for '{}'", element);
                return None;
            }
        }
    }

    Some(path)
}

pub fn get_matching_path(args: &[String]) -> String {
    let mut args = args.to_vec();
    let mut current_path = env::current_dir().expect("Failed to get current directory");
    let first_arg = args.first().expect("No arguments provided");
    if first_arg == "/" {
        current_path = PathBuf::from("/");
        args.remove(0);
    } else if first_arg.starts_with("..") {
        for _ in 0..first_arg.matches(".").count() - 1 {
            current_path.pop();
        }
        args.remove(0);
    }
    if let Some(new_path) = find_matching_path(args, current_path) {
        return new_path.display().to_string();
    }
    return String::new();
}
