use std::path::PathBuf;

use lacy::query::resolve_query;

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

fn setup() {
    use std::fs;

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
        let path = current_path().join("test").join(dir);
        if !path.exists() {
            let result = fs::create_dir_all(path);
            if result.is_err() {
                panic!("error");
            }
        }
    }
}

#[test]
fn test_basic() {
    setup();

    assert_eq!(resolve_query("/"), vec![abs_path("/")]);
}

#[test]
fn test_nonexisting() {
    assert!(resolve_query("test zzzzzzzzz zzzzzzzzz").is_empty());
}

#[test]
fn test_alpha() {
    setup();

    assert_eq!(
        resolve_query("test alph alp"),
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
    setup();

    assert_eq!(
        resolve_query("test alpha beta a"),
        vec![
            abs_path("test/alpha/beta/delta6"),
            abs_path("test/alpha/beta/gamma3"),
        ]
    );
}

#[test]
fn test_alpha_with_slashes() {
    setup();

    assert_eq!(
        resolve_query("test alph/alp"),
        vec![abs_path("test/alpha/alpha")]
    );
    assert_eq!(
        resolve_query("tst/eps/bta/om9/0"),
        vec![abs_path("test/epsilon/beta/omega9/alpha0")]
    );
    assert_eq!(
        resolve_query("test/delta gamma"),
        vec![abs_path("test/delta/gamma7")]
    );
    assert_eq!(
        resolve_query("test delta gamma"),
        vec![abs_path("test/delta/gamma7")]
    );
}

#[test]
fn test_multiple_spaces_or_slashes() {
    setup();

    assert_eq!(
        resolve_query("test  alph alp"),
        vec![abs_path("test/alpha/alpha")]
    );
    assert_eq!(
        resolve_query("tst eps bta  om9 0"),
        vec![abs_path("test/epsilon/beta/omega9/alpha0")]
    );
    assert_eq!(
        resolve_query("test /delta   gamma"),
        vec![abs_path("test/delta/gamma7")]
    );
}

#[test]
fn test_dir_skip() {
    setup();

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
    assert_eq!(resolve_query("- alpha"), vec![abs_path("test/alpha")]);
}

#[test]
fn test_numeric_suffixes() {
    setup();

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
    setup();

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
