use std::{env::home_dir, path::Path};

use crate::directory::{scored_directories, sub_directories, Directory, ScoredDirectory};

#[derive(Debug, PartialEq)]
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
            _ if part.starts_with('-') && part.replace('-', "").is_empty() => {
                QueryPart::Skip(part.len() as u32 - 1)
            }
            _ if part.starts_with("..") && part.replace('.', "").is_empty() => {
                QueryPart::Back(part.len() as u32 - 1)
            }
            _ => QueryPart::Text(part.to_string()),
        }
    }
}

impl QueryPart {
    pub fn matching_directories(&self, dirs: &[ScoredDirectory]) -> Vec<ScoredDirectory> {
        match &self {
            QueryPart::Tilde => {
                let Ok(dir) = Directory::try_from(home_dir().as_deref().unwrap_or(Path::new("/")))
                else {
                    return vec![];
                };
                vec![ScoredDirectory::new(dir, 0)]
            }
            QueryPart::Root => {
                let Ok(dir) = Directory::try_from(Path::new("/")) else {
                    eprintln!("Couldn't create Directory from root!");
                    return vec![];
                };
                vec![ScoredDirectory::new(dir, 0)]
            }
            QueryPart::Skip(depth) => dirs
                .iter()
                .flat_map(|dir| {
                    sub_directories(dir.directory().location().as_path(), *depth)
                        .into_iter()
                        .map(|subdir| ScoredDirectory::new(subdir, dir.score()))
                })
                .collect(),
            QueryPart::Back(amount) => dirs
                .iter()
                .filter_map(|dir| {
                    Some(ScoredDirectory::new(
                        Directory::try_from(
                            dir.directory()
                                .location()
                                .join("../".repeat(*amount as usize))
                                .as_ref(),
                        )
                        .ok()?,
                        dir.score(),
                    ))
                })
                .collect(),
            QueryPart::Text(text) => {
                let scored_dirs = scored_directories(
                    &dirs
                        .iter()
                        .flat_map(|dir| sub_directories(dir.directory().location().as_path(), 0))
                        .collect::<Vec<_>>(),
                    text.as_str(),
                );

                let average_score: f64 = scored_dirs
                    .iter()
                    .map(|scored_dir| f64::from(scored_dir.score()))
                    .sum::<f64>()
                    / scored_dirs.len() as f64;

                let half_of_highest_score = scored_dirs
                    .iter()
                    .map(ScoredDirectory::score)
                    .max()
                    .unwrap_or(0_i32)
                    / 2;

                scored_dirs
                    .iter()
                    // remove dirs with low score
                    .filter(|scored_dir| {
                        f64::from(scored_dir.score()) > 0.0
                            && f64::from(scored_dir.score()) >= average_score
                            && scored_dir.score() >= half_of_highest_score
                    })
                    .cloned()
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
