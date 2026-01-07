#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
fn symlink<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(src: P, dst: Q) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(src, dst)
}
use std::{fs, path::PathBuf};

use lacy::query::Query;
use tempfile::TempDir;

struct TempEnv {
    dir: TempDir,
}

impl TempEnv {
    fn new() -> Self {
        let tmpdir = tempfile::tempdir().unwrap();
        let dir_list = vec![
            "alpha/beta/gamma3",
            "alpha/beta/delta6",
            "alpha/beta/epsil@on8",
            "alpha/betabeta/epsil0n/zeta1",
            "alpha/betabeta/epsil#on/eta4",
            "alpha/betabetaa/thet@a/iota7",
            "alpha/alpha/thet#a/iota0/kappa1",
            "alpha/alpha/beta/theta3/lambda4",
            "beta/mu6/n@u7",
            "beta/mu9/xi0",
            "beta/mu2/omicron3/pi4",
            "beta/mu6/rho7",
            "gamma/sigma9/t@u0",
            "gamma/sigm#a/upsilon3",
            "gamma/ph!i/chi6",
            "gamma/ph!i/psi9/omega0",
            "gamma/alpha2",
            "gamma/alpha4/beta5",
            "delta/gamma7",
            "delta/delta9/epsilon0",
            "delta/del@ta/zeta2/eta3",
            "delta/thet#a/iota5",
            "delta/kapp@a/lambda7",
            "delta/mu9",
            "epsilon/nu1/x2",
            "epsilon/xi4",
            "epsilon/omicron6/pi7",
            "epsilon/omicron9/rh0",
            "epsilon/sigm#a/tau2",
            "epsilon/upsil@on/phi4",
            "epsilon/beta/chi6/ps!i7",
            "epsilon/beta/omega9/alpha0",
            "epsilon/beta/beta2/gamma3",
            "epsilon/beta/del@ta/epsilon5",
            "epsilon/beta/eta7/theta8",
            "epsilon/beta/iot@a/kappa0",
        ];

        for dir in dir_list {
            let path = tmpdir.path().join("test").join(dir);
            if !path.exists() {
                let result = fs::create_dir_all(path);
                if result.is_err() {
                    panic!("Error, couldn't create test folder in tempdir!");
                }
            }
        }

        symlink(
            tmpdir.path().join("test/alpha"),
            tmpdir.path().join("test/link"),
        )
        .unwrap();

        Self { dir: tmpdir }
    }

    fn resolve_query(&self, query: &str) -> Vec<PathBuf> {
        let query = Query::from(query.to_string());
        query.results(self.dir.path())
    }

    fn abs_path(&self, path: &str) -> PathBuf {
        let abs_path = PathBuf::from(path);
        if abs_path.is_absolute() {
            return abs_path;
        }
        self.dir.path().to_path_buf().join(path)
    }
}

#[test]
fn test_absolute() {
    let env = TempEnv::new();
    assert_eq!(env.resolve_query("/"), vec![env.abs_path("/")]);
}

#[test]
fn test_nonexisting() {
    let env = TempEnv::new();
    assert!(env.resolve_query("test zzzzzzzzz zzzzzzzzz").is_empty());
}

#[test]
fn test_alpha() {
    let env = TempEnv::new();

    assert_eq!(
        env.resolve_query("test alph alp"),
        vec![env.abs_path("test/alpha/alpha")]
    );
    assert_eq!(
        env.resolve_query("tst eps bta om9 0"),
        vec![env.abs_path("test/epsilon/beta/omega9/alpha0")]
    );
    assert_eq!(
        env.resolve_query("test delta gamma"),
        vec![env.abs_path("test/delta/gamma7")]
    );
    assert_eq!(
        env.resolve_query("test delta gamma "),
        vec![env.abs_path("test/delta/gamma7")]
    );
}

#[test]
fn test_multiple_matches() {
    let env = TempEnv::new();

    assert_eq!(
        env.resolve_query("test alpha beta a"),
        vec![
            env.abs_path("test/alpha/beta/delta6"),
            env.abs_path("test/alpha/beta/gamma3"),
        ]
    );
}

#[test]
fn test_alpha_with_slashes() {
    let env = TempEnv::new();

    assert_eq!(
        env.resolve_query("test alph/alp"),
        vec![env.abs_path("test/alpha/alpha")]
    );
    assert_eq!(
        env.resolve_query("tst/eps/bta/om9/0"),
        vec![env.abs_path("test/epsilon/beta/omega9/alpha0")]
    );
    assert_eq!(
        env.resolve_query("test/delta gamma"),
        vec![env.abs_path("test/delta/gamma7")]
    );
    assert_eq!(
        env.resolve_query("test delta gamma"),
        vec![env.abs_path("test/delta/gamma7")]
    );
}

#[test]
fn test_multiple_spaces_or_slashes() {
    let env = TempEnv::new();

    assert_eq!(
        env.resolve_query("test  alph alp"),
        vec![env.abs_path("test/alpha/alpha")]
    );
    assert_eq!(
        env.resolve_query("tst eps bta  om9 0"),
        vec![env.abs_path("test/epsilon/beta/omega9/alpha0")]
    );
    assert_eq!(
        env.resolve_query("test /delta   gamma"),
        vec![env.abs_path("test/delta/gamma7")]
    );
}

#[test]
fn test_dir_skip() {
    let env = TempEnv::new();

    assert_eq!(
        env.resolve_query("test gamma - u"),
        vec!(
            env.abs_path("test/gamma/sigm#a/upsilon3"),
            env.abs_path("test/gamma/sigma9/t@u0"),
        )
    );
    assert_eq!(
        env.resolve_query("test alpha - epsil#on et4"),
        vec![env.abs_path("test/alpha/betabeta/epsil#on/eta4")]
    );
    assert_eq!(
        env.resolve_query("test alpha - - et4"),
        vec![env.abs_path("test/alpha/betabeta/epsil#on/eta4")]
    );
    assert_eq!(
        env.resolve_query("- alpha"),
        vec![env.abs_path("test/alpha")]
    );
}

#[test]
fn test_numeric_suffixes() {
    let env = TempEnv::new();

    assert_eq!(
        env.resolve_query("test epsilon beta beta2 3"),
        vec![env.abs_path("test/epsilon/beta/beta2/gamma3")]
    );
    assert_eq!(
        env.resolve_query("test beta 9"),
        vec![env.abs_path("test/beta/mu9")]
    );
}

#[test]
fn test_real_paths() {
    let env = TempEnv::new();

    assert_eq!(
        env.resolve_query("test alpha/beta del6"),
        vec![env.abs_path("test/alpha/beta/delta6")]
    );
    assert_eq!(
        env.resolve_query("test /alpha/beta del6"),
        vec![env.abs_path("test/alpha/beta/delta6")]
    );
    assert_eq!(
        env.resolve_query("test /alpha/beta/ del6"),
        vec![env.abs_path("test/alpha/beta/delta6")]
    );
}

#[test]
fn test_symlinks() {
    let env = TempEnv::new();

    assert_eq!(
        env.resolve_query("test link beta"),
        vec![env.abs_path("test/link/beta")]
    );
    assert_eq!(
        env.resolve_query("test link - gamma3"),
        vec![env.abs_path("test/link/beta/gamma3")]
    );
    assert_eq!(
        env.resolve_query("test - beta gamma3"),
        vec![
            env.abs_path("test/alpha/beta/gamma3"),
            env.abs_path("test/link/beta/gamma3")
        ]
    );
}
