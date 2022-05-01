use std::{borrow::Cow, fs};

use crate::git_config::from_paths::escape_backslashes;
use git_config::file::{from_paths, GitConfig};
use tempfile::tempdir;

#[test]
fn multiple() {
    let dir = tempdir().unwrap();

    let a_path = dir.path().join("a");
    fs::write(
        a_path.as_path(),
        "
[core]
  a = false
  sslVerify = true
  d = 41",
    )
    .unwrap();

    let b_path = dir.path().join("b");
    let relative_b_path: std::path::PathBuf = "b".into();
    fs::write(
        b_path.as_path(),
        "
[diff]
  renames = true",
    )
    .unwrap();
    let ignore_path = dir.path().join("ignore");
    fs::write(
        ignore_path.as_path(),
        "
[diff]
  renames = invalid",
    )
    .unwrap();

    let a_path_string = escape_backslashes(a_path.parent().unwrap());
    let non_canonical_path_a = format!("{}/./a", a_path_string);
    let non_existing_path = "/dfgwfsghfdsfs";
    let c_path = dir.path().join("c");
    fs::write(
        c_path.as_path(),
        format!(
            "
[core]
  c = 12
  d = 42
[include]
  path = {}
  path = {}
  path = {}
[include.ignore]
  path = {}
[http]
  sslVerify = false",
            non_existing_path,
            non_canonical_path_a,
            relative_b_path.as_path().to_str().unwrap(),
            escape_backslashes(&ignore_path)
        ),
    )
    .unwrap();

    let config = GitConfig::from_paths(vec![c_path], &Default::default()).unwrap();

    assert_eq!(
        config.raw_value("core", None, "c").unwrap(),
        Cow::<[u8]>::Borrowed(b"12")
    );
    assert_eq!(
        config.raw_value("core", None, "d").unwrap(),
        Cow::<[u8]>::Borrowed(b"41")
    );
    assert_eq!(
        config.raw_value("http", None, "sslVerify").unwrap(),
        Cow::<[u8]>::Borrowed(b"false")
    );

    assert_eq!(
        config.raw_value("diff", None, "renames").unwrap(),
        Cow::<[u8]>::Borrowed(b"true")
    );

    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"false")
    );
}

#[test]
fn respect_max_depth() {
    let dir = tempdir().unwrap();

    // 0 includes 1 - base level
    // 1 includes 2
    // 2 includes 3
    // 3 includes 4
    // 4 has no includes
    let max_depth = 4u8;
    for (i, next_i) in (0..max_depth).zip(1..=max_depth) {
        let path = dir.path().join(i.to_string());
        let next_path = dir.path().join(next_i.to_string());
        fs::write(
            path.as_path(),
            format!(
                "
        [core]
          i = {i} 
        [include]
          path = {}",
                escape_backslashes(&next_path),
            ),
        )
        .unwrap();
    }

    fs::write(
        dir.path().join(max_depth.to_string()),
        "
        [core]
          i = {}"
            .replace("{}", &max_depth.to_string()),
    )
    .unwrap();

    let options = from_paths::Options::default();
    let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
    assert_eq!(
        config.raw_multi_value("core", None, "i").unwrap(),
        vec![
            Cow::Borrowed(b"0"),
            Cow::Borrowed(b"1"),
            Cow::Borrowed(b"2"),
            Cow::Borrowed(b"3"),
            Cow::Borrowed(b"4")
        ]
    );

    // with max_allowed_depth of 1 and 4 levels of includes and error_on_max_depth_exceeded: false, max_allowed_depth is exceeded and the value of level 1 is returned
    // this is equivalent to running git with --no-includes option
    let options = from_paths::Options {
        max_depth: 1,
        error_on_max_depth_exceeded: false,
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "i").unwrap(),
        Cow::<[u8]>::Borrowed(b"1")
    );

    // with default max_allowed_depth of 10 and 4 levels of includes, last level is read
    let options = from_paths::Options::default();
    let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "i").unwrap(),
        Cow::<[u8]>::Borrowed(b"4")
    );

    // with max_allowed_depth of 5, the base and 4 levels of includes, last level is read
    let options = from_paths::Options {
        max_depth: 5,
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "i").unwrap(),
        Cow::<[u8]>::Borrowed(b"4")
    );

    // with max_allowed_depth of 2 and 4 levels of includes, max_allowed_depth is exceeded and error is returned
    let options = from_paths::Options {
        max_depth: 2,
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![dir.path().join("0")], &options);
    assert!(matches!(
        config.unwrap_err(),
        from_paths::Error::IncludeDepthExceeded { max_depth: 2 }
    ));

    // with max_allowed_depth of 2 and 4 levels of includes and error_on_max_depth_exceeded: false , max_allowed_depth is exceeded and the value of level 2 is returned
    let options = from_paths::Options {
        max_depth: 2,
        error_on_max_depth_exceeded: false,
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "i").unwrap(),
        Cow::<[u8]>::Borrowed(b"2")
    );

    // with max_allowed_depth of 0 and 4 levels of includes, max_allowed_depth is exceeded and error is returned
    let options = from_paths::Options {
        max_depth: 0,
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![dir.path().join("0")], &options);
    assert!(matches!(
        config.unwrap_err(),
        from_paths::Error::IncludeDepthExceeded { max_depth: 0 }
    ));
}

#[test]
fn simple() {
    let dir = tempdir().unwrap();

    let a_path = dir.path().join("a");
    let b_path = dir.path().join("b");

    fs::write(
        a_path.as_path(),
        format!(
            "
[core]
  b = true
[include]
  path = {}
[core]
  b = true
[include]
  path = {}",
            escape_backslashes(&b_path),
            escape_backslashes(&b_path)
        ),
    )
    .unwrap();

    fs::write(
        b_path.as_path(),
        "
[core]
  b = false",
    )
    .unwrap();

    let config = GitConfig::from_paths(vec![a_path], &Default::default()).unwrap();
    assert_eq!(
        config.raw_value("core", None, "b").unwrap(),
        Cow::<[u8]>::Borrowed(b"false")
    );
}

#[test]
fn cycle_detection() {
    let dir = tempdir().unwrap();

    let a_path = dir.path().join("a");
    let b_path = dir.path().join("b");

    fs::write(
        a_path.as_path(),
        format!(
            "
[core]
  b = 0
[include]
  path = {}",
            escape_backslashes(&b_path),
        ),
    )
    .unwrap();

    fs::write(
        b_path.as_path(),
        format!(
            "
[core]
  b = 1
[include]
  path = {}",
            escape_backslashes(&a_path),
        ),
    )
    .unwrap();

    let options = from_paths::Options {
        max_depth: 4,
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![a_path.clone()], &options);
    assert!(matches!(
        config.unwrap_err(),
        from_paths::Error::IncludeDepthExceeded { max_depth: 4 }
    ));

    let options = from_paths::Options {
        max_depth: 4,
        error_on_max_depth_exceeded: false,
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![a_path], &options).unwrap();
    assert_eq!(
        config.raw_multi_value("core", None, "b").unwrap(),
        vec![
            Cow::Borrowed(b"0"),
            Cow::Borrowed(b"1"),
            Cow::Borrowed(b"0"),
            Cow::Borrowed(b"1"),
            Cow::Borrowed(b"0"),
        ]
    );
}

#[test]
fn nested() {
    let dir = tempdir().unwrap();

    let a_path = dir.path().join("a");
    fs::write(
        a_path.as_path(),
        "
[core]
  a = false
  c = 1",
    )
    .unwrap();

    let b_path = dir.path().join("b");
    fs::write(
        b_path.as_path(),
        format!(
            "
[core]
  b = true
[include]
  path = {}",
            escape_backslashes(&a_path)
        ),
    )
    .unwrap();

    let c_path = dir.path().join("c");
    fs::write(
        c_path.as_path(),
        format!(
            "
[core]
  c = 12
[include]
  path = {}",
            escape_backslashes(&b_path)
        ),
    )
    .unwrap();

    let config = GitConfig::from_paths(vec![c_path], &Default::default()).unwrap();

    assert_eq!(
        config.raw_value("core", None, "c").unwrap(),
        Cow::<[u8]>::Borrowed(b"1")
    );

    assert_eq!(
        config.raw_value("core", None, "b").unwrap(),
        Cow::<[u8]>::Borrowed(b"true")
    );

    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"false")
    );
}
