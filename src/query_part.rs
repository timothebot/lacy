use std::{env::home_dir, path::PathBuf};

use crate::directory::{scored_directories, sub_directories, Directory};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryPart {
    /// ~
    Tilde,

    /// .. (two or more dots)
    Back(u32),

    /// /
    Root,

    /// - (one or more dashes)
    Skip(u32),

    /// Anything else
    Text(String),
}

impl From<&str> for QueryPart {
    fn from(part: &str) -> Self {
        match part {
            "" => QueryPart::Root,
            "~" => QueryPart::Tilde,
            _ if part.starts_with("-") && part.replace("-", "").is_empty() => {
                QueryPart::Skip(part.len() as u32 - 1)
            }
            _ if part.starts_with("..") && part.replace(".", "").is_empty() => {
                QueryPart::Back(part.len() as u32 - 1)
            }
            _ => QueryPart::Text(part.to_string()),
        }
    }
}

impl QueryPart {
    pub fn matching_directories(&self, dirs: &[Directory]) -> Vec<Directory> {
        match &self {
            QueryPart::Tilde => {
                let Ok(dir) =
                    Directory::try_from(home_dir().unwrap_or(PathBuf::from("/")).as_path())
                else {
                    return vec![];
                };
                vec![dir]
            }
            QueryPart::Root => {
                let Ok(dir) = Directory::try_from(PathBuf::from("/").as_path()) else {
                    eprintln!("Couldn't create Directory from root!");
                    return vec![];
                };
                vec![dir]
            }
            QueryPart::Skip(depth) => dirs
                .iter()
                .flat_map(|dir| sub_directories(dir.location().as_path(), *depth))
                .collect(),
            QueryPart::Back(amount) => {
                let Some(target_dir) = dirs.first() else {
                    return vec![];
                };
                let mut target_location = target_dir.location().clone();
                for _ in 0..*amount {
                    target_location.push("..");
                }
                let Ok(dir) = Directory::try_from(target_location.as_path()) else {
                    return vec![];
                };
                vec![dir]
            }
            QueryPart::Text(text) => {
                let mut scored_dirs = scored_directories(
                    dirs.iter()
                        .flat_map(|dir| sub_directories(dir.location().as_path(), 0))
                        .collect(),
                    text.as_str(),
                );

                let average_score: f64 = scored_dirs
                    .iter()
                    .map(|scored_dir| scored_dir.score() as f64)
                    .sum::<f64>()
                    / scored_dirs.len() as f64;

                let half_of_highest_score = scored_dirs
                    .iter()
                    .map(|scored_dir| scored_dir.score())
                    .max()
                    .unwrap_or(0_i32)
                    / 2;

                // sort by alphabetical order, then by score
                scored_dirs.sort_by(|a, b| a.directory().location().cmp(b.directory().location()));
                scored_dirs.sort_by_key(|a| a.score());

                scored_dirs
                    .iter()
                    // remove dirs with low score
                    .filter(|scored_dir| {
                        scored_dir.score() as f64 > 0.0
                            && scored_dir.score() as f64 >= average_score
                            && scored_dir.score() >= half_of_highest_score
                    })
                    .map(|scored_dir| scored_dir.directory().clone())
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(QueryPart::Tilde, QueryPart::from("~"));
        assert_eq!(QueryPart::Back(1), QueryPart::from(".."));
        assert_eq!(QueryPart::Back(2), QueryPart::from("..."));
        assert_eq!(QueryPart::Root, QueryPart::from(""));
        assert_eq!(QueryPart::Skip(0), QueryPart::from("-"));
        assert_eq!(QueryPart::Skip(1), QueryPart::from("--"));
        assert_eq!(
            QueryPart::Text(String::from("hello")),
            QueryPart::from("hello")
        );
    }
}
