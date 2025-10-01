use crate::fuzzy::fuzzy_match_score;
use std::env;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct PathNode {
    location: PathBuf,
    score: i32,
}

/// Get a list of matching directories for the given query
pub fn resolve_query(query: &str) -> Vec<PathBuf> {
    let prepared_query = query
        .replace("  ", " ")
        .replace(" ", "/")
        .replace("//", "/");
    if prepared_query.is_empty() {
        return vec![get_current_directory()];
    }
    let mut query_parts: Vec<&str> = prepared_query.split("/").collect();

    if query_parts.is_empty() {
        return vec![get_current_directory()];
    }
    query_parts.reverse();

    let Some(first_query_part) = query_parts.last() else {
        return vec![get_current_directory()];
    };

    let start_location = match *first_query_part {
        "~" => {
            query_parts.pop();
            PathBuf::from(env::var("HOME").unwrap_or(String::from("/")))
        }
        "" => {
            query_parts.pop();
            PathBuf::from("/")
        }
        _ if first_query_part.starts_with("..") => {
            let mut current_dir = get_current_directory();
            for _ in 0..first_query_part.matches(".").count() - 1 {
                current_dir.pop();
            }
            query_parts.pop();
            current_dir
        }
        _ => get_current_directory(),
    };

    if query_parts.is_empty()
        || (query_parts.len() == 1 && query_parts.first().unwrap_or(&"/") == &"")
    {
        return vec![start_location];
    };

    let mut result_paths = get_matching_paths(
        vec![PathNode {
            location: start_location,
            score: 0,
        }],
        &mut query_parts,
    );

    // sort by alphabetical order, then by score
    result_paths.sort_by(|a, b| a.location.cmp(&b.location));
    result_paths.sort_by(|a, b| a.score.cmp(&b.score));

    // dont return the score
    result_paths
        .iter()
        .map(|path_node| path_node.location.clone())
        .collect()
}

fn get_current_directory() -> PathBuf {
    env::current_dir().unwrap_or(PathBuf::from("/"))
}

/// Returns an unscored vec of all directories for the given path
fn get_all_directories_in(path: &PathBuf) -> Vec<PathBuf> {
    let dirs_res = fs::read_dir(path);
    let Ok(dirs) = dirs_res else {
        return vec![];
    };
    dirs.filter_map(|entry| {
        let Ok(entry) = entry else {
            return None;
        };
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                return Some(vec![entry.path()]);
            }
        }
        None
    })
    .flatten()
    .collect()
}

/// Get all directories in the given path with their score compared to the query
/// Returns all directories when query is -
fn get_scored_directories(path: &PathBuf, query: &str) -> Vec<PathNode> {
    let directories = get_all_directories_in(path);

    // if "-" return all
    if query == "-" {
        return directories
            .iter()
            .map(|dir| PathNode {
                location: dir.into(),
                score: i32::MAX,
            })
            .collect();
    }

    let mut scored_dirs: Vec<PathNode> = vec![];
    for directory in &directories {
        if !directory.is_dir() {
            continue;
        }
        let Some(dir_name) = directory.file_name() else {
            continue;
        };
        let score = fuzzy_match_score(dir_name.to_str().unwrap_or(""), query);

        if score > 0 {
            scored_dirs.push(PathNode {
                location: directory.into(),
                score,
            });
        }
    }
    scored_dirs
}

/// Takes an array of paths and searches them recursivly until all parts of the query
/// are checked and then returns the results.
/// Only high scoring paths are returned.
fn get_matching_paths(possible_paths: Vec<PathNode>, query: &mut Vec<&str>) -> Vec<PathNode> {
    // if query is empty, return paths
    if query.is_empty() {
        return possible_paths;
    }
    // take first part from query
    let Some(current_query_part) = query.pop() else {
        return possible_paths;
    };

    let mut all_possible_dirs: Vec<PathNode> = vec![];
    for possible_path in possible_paths {
        all_possible_dirs.append(&mut get_scored_directories(
            &possible_path.location,
            current_query_part,
        ));
    }

    // remove dirs with low score
    let average_score: f64 = all_possible_dirs
        .iter()
        .map(|scored_path_node| scored_path_node.score as f64)
        .sum::<f64>()
        / all_possible_dirs.len() as f64;

    let half_of_highest_score = all_possible_dirs
        .iter()
        .map(|scored_path_node| scored_path_node.score)
        .max()
        .unwrap_or(0_i32)
        / 2;
    get_matching_paths(
        all_possible_dirs
            .iter()
            .filter(|scored_path| {
                scored_path.score as f64 >= average_score
                    && scored_path.score >= half_of_highest_score
            })
            .map(|scored_path| PathNode {
                location: scored_path.location.clone(),
                score: scored_path.score,
            })
            .collect(),
        query,
    )
}
