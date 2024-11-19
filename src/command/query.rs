use std::{fs, path::PathBuf};
use std::env;
use fuzzy_matcher::clangd::fuzzy_match;


#[derive(Debug)]
pub struct PathNode {
    location: PathBuf,
    score: i64,
}

pub fn resolve_query(query: &str) -> Vec<PathBuf> {
    let prepared_query = query.replace(" ", "/").replace("//", "/");
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
        .iter()
        .map(|path_node| path_node.location.clone())
        .collect()
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


#[cfg(test)]
mod tests {
    use super::*;

    fn current_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn abs_path(path: &str) -> PathBuf {
        let mut path = PathBuf::from(path);
        if !path.is_absolute() {
            path = current_path().join(path);
        }
        path
    }

    fn args_to_string(args: Vec<&str>) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            resolve_query(""),
            vec![abs_path("")]
        );
        assert_eq!(
            resolve_query("test"),
            vec![abs_path("test")]
        );
        assert_eq!(
            resolve_query("tes"),
            vec![abs_path("test")]
        );
        assert_eq!(
            resolve_query("es"),
            vec![abs_path("test")]
        );
        assert_eq!(
            resolve_query("/"),
            vec![abs_path("/")]
        );
        assert_eq!(
            resolve_query(".. lacy"),
            vec![abs_path("")]
        );
    }

    #[test]
    fn test_with_slash_at_end() {
        assert_eq!(
            resolve_query("test/"),
            vec![abs_path("test")]
        );
    }

    #[test]
    fn test_nonexisting() {
        assert_eq!(
            resolve_query("test zzzzzzzzz zzzzzzzzz"),
            vec![String::new()]
        );
    }

    #[test]
    fn test_alpha() {
        assert_eq!(
            resolve_query("test alp alp"),
            vec![abs_path("test/alpha/alpha")]
        );
        assert_eq!(
            resolve_query("tst eps bta om9 0"),
            vec![abs_path("test/epsilon/beta/omega9/alpha0")]
        );
        assert_eq!(
            resolve_query("test delta gamma"),
            vec![abs_path("test/delta/gamma7")]
        );
    }

    #[test]
    fn test_multiple_matches() {
        assert_eq!(
            resolve_query("test alpha beta a"),
            vec![
                abs_path("test/alpha/beta/delta6"),
                abs_path("test/alpha/beta/gamma3"),
            ]
        );
    }

    #[test]
    fn test_dir_skip() {
        assert_eq!(
            resolve_query("test gamma - u"),
            vec!(
                abs_path("test/gamma/sigm#a/upsilon3"),
                abs_path("test/gamma/sigma9/t@u0"),
            )
        );
        assert_eq!(
            resolve_query("test alpha - epsil#on et4"),
            vec![abs_path("test/alpha/betabeta/epsil#on/eta4")]
        );
        assert_eq!(
            resolve_query("test alpha - - et4"),
            vec![abs_path("test/alpha/betabeta/epsil#on/eta4")]
        );
        assert_eq!(
            resolve_query("- alpha"),
            vec![abs_path("test/alpha")]
        );
    }

    #[test]
    fn test_numeric_suffixes() {
        assert_eq!(
            resolve_query("test epsilon beta beta2 3"),
            vec![abs_path("test/epsilon/beta/beta2/gamma3")]
        );
        assert_eq!(
            resolve_query("test beta 9"),
            vec![abs_path("test/beta/mu9")]
        );
    }

    #[test]
    fn test_real_paths() {
        assert_eq!(
            resolve_query("test alpha/beta del6"),
            vec![abs_path("test/alpha/beta/delta6")]
        );
        assert_eq!(
            resolve_query("test /alpha/beta del6"),
            vec![abs_path("test/alpha/beta/delta6")]
        );
        assert_eq!(
            resolve_query("test /alpha/beta/ del6"),
            vec![abs_path("test/alpha/beta/delta6")]
        );
    }
}
