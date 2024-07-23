use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::ui;

/// Check if a directory is a match for a given part and return a score
fn lazy_path_matching(dir: &str, part: &str, real_path: &bool) -> i32 {
    let mut score = 0;
    let dir = dir.split("/").last().unwrap_or("");
    if *real_path {
        if dir.to_lowercase().starts_with(part.to_lowercase().as_str()) {
            score += 20;
        }
    }
    if dir.to_lowercase().contains(part.to_lowercase().as_str()) {
        score += 10;
    }
    let mut dir_name_mut = dir.to_string();
    for c in part.chars() {
        if dir_name_mut.to_lowercase().contains(c.to_ascii_lowercase()) {
            score += 1;
            // strip the char to avoid multiple matches
            dir_name_mut = dir_name_mut.replacen(c, "", 1);
        } else {
            score -= 5;
        }
    }
    if dir.to_lowercase() == part.to_lowercase() {
        score += 50;
    }
    score
}

/// Returns a part of the path that matches the given part
/// If there are multiple matches, return a list of possible paths
fn resolve_path_part(
    part: &String,
    current_path: PathBuf,
    real_path: bool,
) -> Result<PathBuf, Option<Vec<(PathBuf, i32)>>> {
    let dirs_res = fs::read_dir(&current_path);
    let Ok(dirs) = dirs_res else {
        return Result::Err(None);
    };

    let possible_dirs: Vec<PathBuf> = dirs
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
        .collect();

    // score all valid directories
    let scored_dirs: HashMap<PathBuf, i32> = possible_dirs
        .iter()
        .filter(|dir| {
            dir.to_str()
                .map(|dir_str| lazy_path_matching(dir_str, part, &real_path) > 0)
                .unwrap_or(false)
        })
        .map(|dir| {
            let score = lazy_path_matching(dir.to_str().unwrap(), part, &real_path);
            return (dir.clone(), score);
        })
        .collect();

    if possible_dirs.is_empty() {
        // TODO: search subdirectories
        return Result::Err(None);
    }
    if scored_dirs.len() == 1 {
        return Result::Ok(scored_dirs.keys().next().unwrap().clone());
    }

    // everything below the average score is discarded
    let average_score: f32 = scored_dirs.values().sum::<i32>() as f32 / scored_dirs.len() as f32;
    // sort by score
    let mut paths: Vec<(PathBuf, i32)> = scored_dirs
        .iter()
        .filter(|(_, score)| **score as f32 >= average_score)
        .map(|(dir, score)| {
            let mut new_path = current_path.clone();
            new_path.push(dir);
            (new_path, *score)
        })
        .collect();

    if paths.len() == 1 {
        return Result::Ok(paths[0].0.clone());
    }

    paths.sort_by(|a, b| b.1.cmp(&a.1));
    Result::Err(Some(paths))
}

/// select one of the given paths
fn choose_path(possible_paths: Vec<(PathBuf, i32)>) -> PathBuf {
    let possible_paths_str: Vec<String> = possible_paths
        .iter()
        .map(|path| format!("{}: {}", path.1, path.0.to_str().unwrap()))
        .collect();
    PathBuf::from(ui::select(
        "Multiple possibilities found",
        possible_paths_str.iter().map(|s| s.as_str()).collect(),
    ))
}

/// Loop through all parts of the given path and return the matching path
fn find_matching_path(parts: Vec<String>, current_path: PathBuf) -> Option<PathBuf> {
    let mut path = current_path;

    for part in parts {
        if part.starts_with("/") {
            for subpart in part.split("/").skip(1) {
                let res = resolve_path_part(&subpart.to_string(), path, true);
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
        let res = resolve_path_part(&part, path, false);
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

/// Get a matching path by the given arguments
pub fn get_matching_path(args: &[String]) -> String {
    let mut args = args.to_vec();
    let mut current_path = env::current_dir().expect("Failed to get current directory");
    let first_arg = args.first().unwrap_or(&String::from("")).clone();
    if *&first_arg.is_empty() {
        return current_path.display().to_string();
    }

    if first_arg.starts_with("/") {
        current_path = PathBuf::from("/");
        if first_arg.len() == 1 {
            args.remove(0);
        }
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
