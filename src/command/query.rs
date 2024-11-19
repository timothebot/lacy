use std::{fs, path::PathBuf};
use std::env;
use fuzzy_matcher::clangd::fuzzy_match;


#[derive(Debug)]
struct PathNode {
    location: PathBuf,
    score: i64,
}

pub fn resolve_query(query: &str) -> Vec<PathNode> {
    let prepared_query = query.replace(" ", "/");
    let mut query_parts: Vec<&str> = prepared_query.split("/").collect();

    if query_parts.is_empty() {
        return Vec::new();
    }
    query_parts.reverse();

    let Some(first_query_part) = query_parts.pop() else {
        return Vec::new();
    };

    get_matching_paths(
        vec!(PathNode {
            location: get_start_location(first_query_part),
            score: 0,
        }),
        &mut query_parts
    )
}

fn get_start_location(first_query_part: &str) -> PathBuf {
    if first_query_part == "~" {
        return PathBuf::from(env::var("HOME")
            .unwrap_or(String::from("/")));
    }
    PathBuf::from("/")
}

/// Returns an unscored vec of all directories for the given path
fn get_all_directories_in(path: &PathBuf) -> Vec<PathBuf> {
    let dirs_res = fs::read_dir(&path);
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

// input path buf
fn get_scored_directories(path: &PathBuf, query: &str) -> Vec<PathNode> {
    let directories = get_all_directories_in(path);

    // if "-" return all
    if query == "-" {
        return directories.iter().map(|dir| PathNode {
            location: dir.into(),
            score: i64::MAX
        }).collect();
    }

    let mut scored_dirs: Vec<PathNode> = vec![];
    for directory in &directories {
        if !directory.is_dir() {
            continue;
        }
        let Some(dir_name) = directory.file_name() else {
            continue;
        };

        let score = fuzzy_match(
            dir_name.to_str().unwrap_or(""),
            query
        ).unwrap_or(0);

        if score != 0 {
            scored_dirs.push(PathNode {
                location: directory.into(),
                score
            });
        }
    }

    return scored_dirs;
}

// take an array of PathBufs and the query, recursive
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
        all_possible_dirs.append(
            &mut get_scored_directories(
                &possible_path.location,
                current_query_part
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
        .unwrap_or(0 as i64)
        / 2;

    return get_matching_paths(
        all_possible_dirs
            .iter()
            .filter(|scored_path| {
                scored_path.score as f64 >= average_score && scored_path.score >= half_of_highest_score
            })
            .map(|scored_path| PathNode {
                location: scored_path.location.clone(),
                score: scored_path.score
            })
            .collect(),
        query
    );

}
