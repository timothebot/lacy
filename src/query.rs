use crate::directory::{sub_directories, Directory};
use crate::query_part::QueryPart;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    query: String,
    parts: Vec<QueryPart>,
}

impl From<String> for Query {
    fn from(query: String) -> Self {
        let mut enhanced_query = query.clone().trim().replace("  ", " ");
        if enhanced_query.trim().is_empty() {
            return Query {
                query,
                parts: vec![],
            };
        }
        if enhanced_query.starts_with("/") {
            enhanced_query = format!("##ROOT## {}", enhanced_query.strip_prefix("/").unwrap());
        }
        if enhanced_query.ends_with("/") {
            enhanced_query = enhanced_query.strip_suffix("/").unwrap().to_string();
        }
        enhanced_query = enhanced_query
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

    pub fn completions(&self, cwd: &Path) -> Vec<PathBuf> {
        let query = self.query.clone();
        if query.trim().is_empty() {
            return sub_directories(cwd, 0)
                .iter()
                .map(|dir| dir.location().clone())
                .collect();
        }
        if query.ends_with(" ") {
            return self
                .results(cwd)
                .iter()
                .flat_map(|dir| sub_directories(dir, 0))
                .map(|dir| dir.location().clone())
                .collect();
        }
        if let QueryPart::Text(_) = &self.parts.last().unwrap_or(&QueryPart::Root) {
            return self.results(cwd);
        }
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
