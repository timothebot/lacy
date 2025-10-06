use crate::directory::Directory;
use crate::query_part::QueryPart;
use std::path::{Path, PathBuf};

// _ => space
// query: /user_tiimo_
// prompt => cd user/tiimo
// complete => all dirs in user/tiimo/

// query: /user_tiimo_-
// prompt => show all dirs in tiimo/**/*
// complete => no completion

// query: /user_tiimo_-_
// prompt => show all dirs in tiimo/**/*
// complete => complete all dirs in tiimo/**/*

// query: /user_tiimo_abc
// prompt => return matching
// complete => complete all dirs at tiimo/* that have a score > 0 (except if its only one result)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    query: String,
    parts: Vec<QueryPart>,
}

impl From<String> for Query {
    fn from(query: String) -> Self {
        let mut enhanced_query = query.clone();
        if enhanced_query.starts_with("/") {
            enhanced_query = format!("##ROOT##{}", enhanced_query.strip_prefix("/").unwrap());
        }
        if enhanced_query.ends_with("/") {
            enhanced_query = enhanced_query.strip_suffix("/").unwrap().to_string();
        }
        enhanced_query = enhanced_query
            .replace("  ", " ")
            .replace(" / ", " ##ROOT## ")
            .replace(" ", "/")
            .replace("//", "/");

        let query_parts: Vec<QueryPart> = enhanced_query
            .split("/")
            .map(|part| {
                if part == "##ROOT##" {
                    return QueryPart::Root;
                }
                QueryPart::from(part)
            })
            .collect();

        Self {
            query,
            parts: query_parts,
        }
    }
}

impl Query {
    pub fn results(&self, cwd: &Path) -> Vec<PathBuf> {
        let Some(start_dir) = Directory::try_from(cwd).ok() else {
            eprintln!("Couldn't get the current dir!");
            return vec![];
        };
        let mut directories = vec![start_dir];
        for part in self.parts() {
            directories = part.matching_directories(&directories);
        }

        directories
            .iter()
            .map(|dir| dir.location().clone())
            .collect()
    }

    #[allow(dead_code)]
    pub fn completions(&self) -> Vec<String> {
        // Different completion algorithm for query:
        // 1. If it ends with space, complete all dirs in current path
        // 2.
        vec![]
    }

    pub fn parts(&self) -> &Vec<QueryPart> {
        &self.parts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: optimize this by smart removing parts before
    // parts like .., / or ~
    #[test]
    #[should_panic]
    fn test_smart_query_creation_tilde() {
        assert_eq!(
            &vec![QueryPart::Tilde],
            Query::from(String::from("hello world ~")).parts()
        );
        assert_eq!(
            &vec![QueryPart::Tilde],
            Query::from(String::from("hello world ~ ~")).parts()
        );
        assert_eq!(
            &vec![QueryPart::Tilde, QueryPart::Text(String::from("hello"))],
            Query::from(String::from("hello world ~ hello")).parts()
        );
    }

    #[test]
    #[should_panic]
    fn test_smart_query_creation_root() {
        assert_eq!(
            &vec![QueryPart::Root],
            Query::from(String::from("hello /")).parts()
        );
        assert_eq!(
            &vec![QueryPart::Root],
            Query::from(String::from("hello / /")).parts()
        );
        assert_eq!(
            &vec![QueryPart::Root],
            Query::from(String::from("/ hello /")).parts()
        );
        assert_eq!(
            &vec![QueryPart::Root],
            Query::from(String::from("/")).parts()
        );
    }

    #[test]
    #[should_panic]
    fn test_smart_query_creation_back() {
        assert_eq!(
            &vec![QueryPart::Root],
            Query::from(String::from("/ hello world ...")).parts()
        );
        assert_eq!(
            &vec![QueryPart::Root],
            Query::from(String::from("/ ...")).parts()
        );
        assert_eq!(
            &vec![QueryPart::Tilde],
            Query::from(String::from("~ ..")).parts()
        );
    }
}
