use std::env;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;

/// check if a directory is a match for a given part
/// TODO: return a score instead of 1 or 0
fn lazy_path_matching(dir: &str, part: &str) -> i32 {
    let mut score = 0;
    if dir.to_lowercase().contains(part.to_lowercase().as_str()) {
        score += 1;
    }
    score
}

/// return a part of the path that matches the given part
/// if there are multiple matches, return a list of possible paths
fn resolve_path_part(
    part: &String,
    current_path: PathBuf,
) -> Result<PathBuf, Option<Vec<PathBuf>>> {
    let dirs_res = fs::read_dir(&current_path);
    let Ok(dirs) = dirs_res else {
        return Result::Err(None);
    };

    let matching_dirs: Vec<PathBuf> = dirs
        .filter_map(|entry| {
            let Ok(entry) = entry else {
                return None;
            };
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    return Some(entry.path());
                }
            }
            None
        })
        .filter(|dir| {
            dir.file_name()
                .and_then(|name| name.to_str())
                .map(|name| lazy_path_matching(name, part) > 0)
                .unwrap_or(false)
        })
        .collect();
    if matching_dirs.len() == 1 {
        return Result::Ok(matching_dirs[0].clone());
    }
    if matching_dirs.is_empty() {
        // if dirs_count < &5 {
        //     return match recursive_subdir_search(part, dirs) {
        //         Ok(new_path) => Result::Ok(new_path),
        //         Err(None) => Result::Err(None),
        //         Err(Some(possible_paths)) => Result::Err(Some(possible_paths)),
        //     };
        // }
        return Result::Err(None);
    }
    let paths: Vec<PathBuf> = matching_dirs
        .iter()
        .map(|dir| {
            let mut new_path = current_path.clone();
            new_path.push(dir);
            new_path
        })
        .collect();

    Result::Err(Some(paths))
}

fn choose_path(possible_paths: Vec<PathBuf>) -> PathBuf {
    eprintln!("Multiple paths found, choosing the first one");
    for path in &possible_paths {
        eprintln!("{}", path.display());
    }
    possible_paths[0].clone()
}

/// search for the directory in the subdirectories of the given directories
fn recursive_subdir_search(part: &String, dirs: ReadDir) -> Result<PathBuf, Option<Vec<PathBuf>>> {
    eprintln!("not implemented yet");
    Err(None)
}

fn find_matching_path(parts: Vec<String>, current_path: PathBuf) -> Option<PathBuf> {
    let mut path = current_path;

    for part in parts {
        if part.starts_with("/") {
            for subpart in part.split("/").skip(1) {
                let res = resolve_path_part(&subpart.to_string(), path);
                match res {
                    Ok(new_path) => {
                        path = new_path;
                    }
                    Err(None) => {
                        return None;
                    }
                    Err(Some(possible_paths)) => {
                        path = choose_path(possible_paths);
                    }
                }
            }
            continue;
        }
        let res = resolve_path_part(&part, path);
        match res {
            Ok(new_path) => {
                path = new_path;
            }
            Err(None) => {
                return None;
            }
            Err(Some(possible_paths)) => {
                path = choose_path(possible_paths);
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
