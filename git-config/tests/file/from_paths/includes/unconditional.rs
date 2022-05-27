use std::fs;

use crate::file::cow_str;
use crate::file::from_paths::escape_backslashes;
use git_config::file::from_paths;
use git_config::File;
use tempfile::tempdir;

#[test]
fn multiple() -> crate::Result {
    let dir = tempdir()?;

    let a_path = dir.path().join("a");
    fs::write(
        a_path.as_path(),
        "
[core]
  a = false
  sslVerify = true
  d = 41",
    )?;

    let b_path = dir.path().join("b");
    let relative_b_path: std::path::PathBuf = "b".into();
    fs::write(
        b_path.as_path(),
        "
[diff]
  renames = true",
    )?;
    let ignore_path = dir.path().join("ignore");
    fs::write(
        ignore_path.as_path(),
        "
[diff]
  renames = invalid",
    )?;

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
    )?;

    let config = File::from_paths(vec![c_path], Default::default())?;

    assert_eq!(config.string("core", None, "c"), Some(cow_str("12")));
    assert_eq!(config.integer("core", None, "d"), Some(Ok(41)));
    assert_eq!(config.boolean("http", None, "sslVerify"), Some(Ok(false)));
    assert_eq!(config.boolean("diff", None, "renames"), Some(Ok(true)));
    assert_eq!(config.boolean("core", None, "a"), Some(Ok(false)));

    Ok(())
}

#[test]
fn respect_max_depth() -> crate::Result {
    let dir = tempdir()?;

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
        )?;
    }

    fs::write(
        dir.path().join(max_depth.to_string()),
        "
        [core]
          i = {}"
            .replace("{}", &max_depth.to_string()),
    )?;

    let config = File::from_paths(vec![dir.path().join("0")], Default::default())?;
    assert_eq!(config.integers("core", None, "i"), Some(Ok(vec![0, 1, 2, 3, 4])));

    // with max_allowed_depth of 1 and 4 levels of includes and error_on_max_depth_exceeded: false, max_allowed_depth is exceeded and the value of level 1 is returned
    // this is equivalent to running git with --no-includes option
    let options = from_paths::Options {
        max_depth: 1,
        error_on_max_depth_exceeded: false,
        ..Default::default()
    };
    let config = File::from_paths(vec![dir.path().join("0")], options)?;
    assert_eq!(config.integer("core", None, "i"), Some(Ok(1)));

    // with default max_allowed_depth of 10 and 4 levels of includes, last level is read
    let options = from_paths::Options::default();
    let config = File::from_paths(vec![dir.path().join("0")], options)?;
    assert_eq!(config.integer("core", None, "i"), Some(Ok(4)));

    // with max_allowed_depth of 5, the base and 4 levels of includes, last level is read
    let options = from_paths::Options {
        max_depth: 5,
        ..Default::default()
    };
    let config = File::from_paths(vec![dir.path().join("0")], options)?;
    assert_eq!(config.integer("core", None, "i"), Some(Ok(4)));

    // with max_allowed_depth of 2 and 4 levels of includes, max_allowed_depth is exceeded and error is returned
    let options = from_paths::Options {
        max_depth: 2,
        ..Default::default()
    };
    let config = File::from_paths(vec![dir.path().join("0")], options);
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
    let config = File::from_paths(vec![dir.path().join("0")], options)?;
    assert_eq!(config.integer("core", None, "i"), Some(Ok(2)));

    // with max_allowed_depth of 0 and 4 levels of includes, max_allowed_depth is exceeded and error is returned
    let options = from_paths::Options {
        max_depth: 0,
        ..Default::default()
    };
    let config = File::from_paths(vec![dir.path().join("0")], options);
    assert!(matches!(
        config.unwrap_err(),
        from_paths::Error::IncludeDepthExceeded { max_depth: 0 }
    ));
    Ok(())
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

    let config = File::from_paths(vec![a_path], Default::default()).unwrap();
    assert_eq!(config.boolean("core", None, "b"), Some(Ok(false)));
}

#[test]
fn cycle_detection() -> crate::Result {
    let dir = tempdir()?;

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
    )?;

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
    )?;

    let options = from_paths::Options {
        max_depth: 4,
        ..Default::default()
    };
    let config = File::from_paths(vec![a_path.clone()], options);
    assert!(matches!(
        config.unwrap_err(),
        from_paths::Error::IncludeDepthExceeded { max_depth: 4 }
    ));

    let options = from_paths::Options {
        max_depth: 4,
        error_on_max_depth_exceeded: false,
        ..Default::default()
    };
    let config = File::from_paths(vec![a_path], options)?;
    assert_eq!(config.integers("core", None, "b"), Some(Ok(vec![0, 1, 0, 1, 0])));
    Ok(())
}

#[test]
fn nested() -> crate::Result {
    let dir = tempdir()?;

    let a_path = dir.path().join("a");
    fs::write(
        a_path.as_path(),
        "
[core]
  a = false
  c = 1",
    )?;

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
    )?;

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
    )?;

    let config = File::from_paths(vec![c_path], Default::default())?;

    assert_eq!(config.integer("core", None, "c"), Some(Ok(1)));
    assert_eq!(config.boolean("core", None, "b"), Some(Ok(true)));
    assert_eq!(config.boolean("core", None, "a"), Some(Ok(false)));
    Ok(())
}
