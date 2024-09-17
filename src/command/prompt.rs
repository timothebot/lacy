use std::env;
use std::fs;
use std::path::PathBuf;

use crate::ui;

type ScoredPath = (PathBuf, i32);

/// Check if a directory is a match for a given part and return a score
fn lazy_path_matching(dir: &str, part: &str, real_path: &bool) -> i32 {
    let mut score = 0;
    let dir = dir.split("/").last().unwrap_or("");
    if real_path == &true {
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
    if score < 0 {
        score = 0;
    }
    score
}

/// Returns a part of the path that matches the given part
/// If there are multiple matches, return a list of possible paths
fn resolve_path_part(
    part: &String,
    possible_dirs: Vec<PathBuf>,
    real_path: bool,
) -> Result<ScoredPath, Option<Vec<ScoredPath>>> {
    if possible_dirs.is_empty() {
        // TODO: search subdirectories
        return Result::Err(None);
    }

    let scored_dirs = score_possible_dirs(&possible_dirs, part, &real_path);

    if scored_dirs.len() == 1 {
        return Result::Ok(scored_dirs.first().unwrap().clone());
    }

    // everything below the average score is discarded
    let average_score: f32 = scored_dirs
        .iter()
        .map(|(_, score)| *score as f32)
        .sum::<f32>()
        / scored_dirs.len() as f32;

    let highest_score_split = scored_dirs
        .iter()
        .map(|(_, score)| score)
        .max()
        .unwrap_or(&0)
        / 2;
    let mut paths: Vec<ScoredPath> = scored_dirs
        .iter()
        .filter(|scored_path| {
            scored_path.1 as f32 >= average_score && scored_path.1 >= highest_score_split
        })
        .map(|(dir, score)| (dir.clone(), *score))
        .collect();

    if paths.len() == 1 {
        return Result::Ok(paths[0].clone());
    }
    // sort by alphabetical order, then by score
    paths.sort_by(|a, b| a.0.cmp(&b.0));
    paths.sort_by(|a, b| b.1.cmp(&a.1));

    Result::Err(Some(paths))
}

/// select one of the given paths
fn choose_path(possible_paths: Vec<ScoredPath>) -> PathBuf {
    if possible_paths.is_empty() {
        return PathBuf::new();
    }
    if possible_paths.len() == 1 {
        return possible_paths[0].0.clone();
    }
    let possible_paths_str: Vec<String> = possible_paths
        .iter()
        .map(|path| format!("{}: {}", path.1, path.0.to_str().unwrap()))
        .collect();
    PathBuf::from(
        ui::select(
            "Multiple possibilities found",
            possible_paths_str.iter().map(|s| s.as_str()).collect(),
        )
        .split(": ")
        .last()
        .unwrap(),
    )
}

fn find_possible_dirs(current_path: &PathBuf, depth: i32) -> Vec<PathBuf> {
    let dirs_res = fs::read_dir(&current_path);
    let Ok(dirs) = dirs_res else {
        return vec![];
    };

    dirs.filter_map(|entry| {
        let Ok(entry) = entry else {
            return None;
        };
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                if depth == 0 {
                    return Some(vec![entry.path()]);
                }
                return Some(find_possible_dirs(&entry.path(), depth - 1));
            }
        }
        None
    })
    .flatten()
    .collect()
}

fn score_possible_dirs(
    possible_dirs: &Vec<PathBuf>,
    part: &str,
    real_path: &bool,
) -> Vec<ScoredPath> {
    possible_dirs
        .iter()
        .map(|dir| {
            let score = lazy_path_matching(dir.to_str().unwrap(), part, &real_path);
            return (dir.clone(), score);
        })
        .filter(|(_, score)| *score > 0)
        .collect()
}

/// Loop through all parts of the given path and return the matching path
fn find_matching_path(
    parts: Vec<String>,
    current_path: PathBuf,
    only_one_match: bool,
) -> Result<PathBuf, Option<Vec<ScoredPath>>> {
    // TODO: change return type to <ScoredPath, Option<Vec<ScoredPath>>> or something else?
    let mut path = current_path.clone();
    if parts.is_empty() {
        return Result::Ok(path);
    }
    let Some(part) = parts.first() else {
        return Result::Err(None);
    };
    let mut new_parts = parts.clone();
    new_parts.remove(0);

    let res;
    if part == "-" {
        let mut depth = 1;
        if new_parts.is_empty() {
            return Result::Err(None);
        }
        let mut next_normal_path = String::new();
        for next_part in new_parts.clone() {
            if next_part == "-" {
                depth += 1;
                new_parts.remove(0);
            } else {
                next_normal_path = new_parts.remove(0);
                break;
            }
        }
        if next_normal_path.is_empty() {
            return Result::Err(None);
        }
        res = resolve_path_part(
            &next_normal_path,
            find_possible_dirs(&current_path, depth),
            false,
        );
    } else if part.contains("/") {
        // split and remove empty strings
        let mut subparts: Vec<&str> = part.split("/").filter(|s| !s.is_empty()).collect();
        let first_subpart = subparts.remove(0);
        new_parts.insert(0, subparts.join("/"));
        res = resolve_path_part(
            &first_subpart.to_string(),
            find_possible_dirs(&current_path, 0),
            true,
        );
    } else {
        res = resolve_path_part(part, find_possible_dirs(&current_path, 0), false);
    }

    match res {
        Ok(new_path) => {
            path = new_path.0;
        }
        Err(None) => {
            return Result::Err(None);
        }
        Err(Some(possible_paths)) => {
            if possible_paths.is_empty() {
                return Result::Err(None);
            }
            if !only_one_match {
                return Result::Err(Some(possible_paths));
            }
            path = choose_path(possible_paths);
        }
    }

    find_matching_path(new_parts.into(), path, only_one_match)
}

/// Get a matching path by the given arguments
pub fn get_matching_path(args: &[String], only_one_match: bool) -> String {
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
    } else if first_arg == "~" {
        current_path = PathBuf::from(env::var("HOME").unwrap());
        args.remove(0);
    }
    let new_path = find_matching_path(args, current_path, only_one_match);
    match new_path {
        Ok(path) => return path.display().to_string(),
        Err(None) => return String::new(),
        Err(Some(possible_paths)) => {
            return possible_paths
                .iter()
                .map(|path| path.0.display().to_string())
                .collect::<Vec<String>>()
                .join(" ");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn current_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn abs_path(path: &str) -> String {
        let mut path = PathBuf::from(path);
        if !path.is_absolute() {
            path = current_path().join(path);
        }
        let path_str = path.display().to_string();
        if path_str.len() == 1 {
            return path_str;
        }
        path_str
            .strip_suffix('/')
            .unwrap_or(&path_str)
            .replace('"', "")
    }

    fn args_to_string(args: Vec<&str>) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            get_matching_path(&args_to_string(vec![""]), false),
            abs_path("")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["test"]), false),
            abs_path("test")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["tes"]), false),
            abs_path("test")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["es"]), false),
            abs_path("test")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["/"]), false),
            abs_path("/")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["..", "lacy"]), false),
            abs_path("")
        );
    }

    #[test]
    fn test_nonexisting() {
        assert_eq!(
            get_matching_path(
                &args_to_string(vec!["test", "zzzzzzzzz", "zzzzzzzzz"]),
                true
            ),
            String::new()
        );
    }

    #[test]
    fn test_alpha() {
        assert_eq!(
            get_matching_path(&args_to_string(vec!["test", "alp", "alp"]), false),
            abs_path("test/alpha/alpha")
        );
        assert_eq!(
            get_matching_path(
                &args_to_string(vec!["tst", "eps", "bta", "om9", "0"]),
                false
            ),
            abs_path("test/epsilon/beta/omega9/alpha0")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["test", "delta", "gamma"]), false),
            abs_path("test/delta/gamma7")
        );
    }

    #[test]
    fn test_multiple_matches() {
        let result = get_matching_path(&args_to_string(vec!["test", "alpha", "beta", "a"]), false);
        let expected = format!(
            "{} {}",
            abs_path("test/alpha/beta/delta6"),
            abs_path("test/alpha/beta/gamma3"),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dir_skip() {
        assert_eq!(
            get_matching_path(&args_to_string(vec!["test", "gamma", "-", "u"]), false),
            format!(
                "{} {}",
                abs_path("test/gamma/sigm#a/upsilon3"),
                abs_path("test/gamma/sigma9/t@u0"),
            )
        );
        assert_eq!(
            get_matching_path(
                &args_to_string(vec!["test", "alpha", "-", "epsil#on", "et4"]),
                false
            ),
            abs_path("test/alpha/betabeta/epsil#on/eta4")
        );
        assert_eq!(
            get_matching_path(
                &args_to_string(vec!["test", "alpha", "-", "-", "et4"]),
                false
            ),
            abs_path("test/alpha/betabeta/epsil#on/eta4")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["-", "alpha"]), false),
            abs_path("test/alpha")
        );
    }

    #[test]
    fn test_numeric_suffixes() {
        assert_eq!(
            get_matching_path(
                &args_to_string(vec!["test", "epsilon", "beta", "beta2", "3"]),
                false
            ),
            abs_path("test/epsilon/beta/beta2/gamma3")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["test", "beta", "9"]), false),
            abs_path("test/beta/mu9")
        );
    }

    #[test]
    fn test_real_paths() {
        assert_eq!(
            get_matching_path(&args_to_string(vec!["test", "alpha/beta", "del6"]), false),
            abs_path("test/alpha/beta/delta6")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["test", "/alpha/beta", "del6"]), false),
            abs_path("test/alpha/beta/delta6")
        );
        assert_eq!(
            get_matching_path(&args_to_string(vec!["test", "/alpha/beta/", "del6"]), false),
            abs_path("test/alpha/beta/delta6")
        );
    }
}
