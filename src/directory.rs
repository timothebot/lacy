use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn get_current_directory() -> PathBuf {
    env::current_dir().unwrap_or(PathBuf::from("/"))
}

/// Returns all directories for the given path
pub fn get_all_directories_in(path: &Path) -> Vec<Directory> {
    let dirs_res = fs::read_dir(path);
    let Ok(dirs) = dirs_res else {
        return vec![];
    };
    dirs.filter_map(|entry| Directory::try_from(entry.ok()?.path().as_path()).ok())
        .collect()
}

/// Get the sub directories at a specified depth relative to the given cwd.
///
/// ### Example
///
/// With this dir structure:
///
/// ```text
/// User/ <-- CWD
///   Desktop/
///     Wallpapers/
///   Documents/
///     FirstProject/
///     SecondProject/
/// ```
///
/// Depth 0 returns `Desktop` and `Documents`.
///
/// Depth 1 returns `Wallpapers`, `FirstProject` and `SecondProject`.
pub fn sub_directories(cwd: &Path, depth: u32) -> Vec<Directory> {
    let mut directories = get_all_directories_in(cwd);
    for _ in 0..depth {
        let mut next_directories: Vec<Directory> = vec![];
        for dir in directories {
            next_directories.append(&mut get_all_directories_in(dir.location.as_path()));
        }
        directories = next_directories;
    }
    directories
}

#[derive(Debug)]
pub struct ScoredDirectory {
    directory: Directory,
    score: i32,
}

impl ScoredDirectory {
    pub fn new(directory: Directory, score: i32) -> Self {
        Self { directory, score }
    }

    pub fn directory(&self) -> &Directory {
        &self.directory
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

pub fn scored_directories(directories: Vec<Directory>, query: &str) -> Vec<ScoredDirectory> {
    directories
        .iter()
        .map(|directory| {
            let score = fuzzy_dir::score_dir(directory.name(), query);
            ScoredDirectory::new(directory.clone(), score)
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Directory {
    /// The name of the directory
    name: String,

    /// The actual path of the directory, with symlinks resolved
    location: PathBuf,
}

impl Directory {
    pub fn new(name: String, location: PathBuf) -> Self {
        Self { name, location }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &PathBuf {
        &self.location
    }
}

impl TryFrom<&Path> for Directory {
    type Error = ();

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.is_symlink() {
            return Ok(Directory::new(
                path.file_name().ok_or(())?.display().to_string(),
                path.to_path_buf(),
            ));
        }
        if path.is_dir() {
            let Some(file_name) = path.file_name() else {
                return Ok(Directory::new(String::new(), path.to_path_buf()));
            };
            return Ok(Directory::new(
                file_name.display().to_string(),
                path.to_path_buf(),
            ));
        }
        Err(())
    }
}
