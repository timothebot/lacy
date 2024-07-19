use std::env;
use std::fs;
use std::path::PathBuf;

fn find_matching_path(elements: &[String], current_path: PathBuf) -> Option<PathBuf> {
    let mut path = current_path;

    for (index, element) in elements.iter().enumerate() {
        if index == 0 && element.starts_with('/'){
            continue;
        }
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

fn lacy_cd(args: &[String]) {
    let mut current_path = env::current_dir().expect("Failed to get current directory");
    let first_arg = args.first().expect("No arguments provided");
    if first_arg == "/" {
        current_path = PathBuf::from("/");
    }
    if let Some(new_path) = find_matching_path(args, current_path) {
        println!("{}", new_path.display());
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: y <path_element1> <path_element2> ...");
    } else {
        lacy_cd(&args);
    }
}
