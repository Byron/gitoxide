use std::{borrow::Cow, fs, io};

use git_config::{file::from_paths::Error, parse, File};
use tempfile::tempdir;

use crate::file::cow_str;

/// Escapes backslash when writing a path as string so that it is a valid windows path
fn escape_backslashes(path: impl AsRef<std::path::Path>) -> String {
    path.as_ref().to_str().unwrap().replace('\\', "\\\\")
}

#[test]
fn file_not_found() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config");

    let paths = vec![config_path];
    let error = File::from_paths(paths, Default::default()).unwrap_err();
    assert!(
        matches!(error,  Error::Parse(parse::state::from_path::Error::Io(io_error)) if io_error.kind() == io::ErrorKind::NotFound)
    );
}

#[test]
fn single_path() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config");
    fs::write(config_path.as_path(), b"[core]\nboolean = true").unwrap();

    let paths = vec![config_path];
    let config = File::from_paths(paths, Default::default()).unwrap();

    assert_eq!(
        config.raw_value("core", None, "boolean").unwrap(),
        Cow::<[u8]>::Borrowed(b"true")
    );

    assert_eq!(config.num_values(), 1);
}

#[test]
fn multiple_paths_single_value() -> crate::Result {
    let dir = tempdir()?;

    let a_path = dir.path().join("a");
    fs::write(a_path.as_path(), b"[core]\na = true")?;

    let b_path = dir.path().join("b");
    fs::write(b_path.as_path(), b"[core]\nb = true")?;

    let c_path = dir.path().join("c");
    fs::write(c_path.as_path(), b"[core]\nc = true")?;

    let d_path = dir.path().join("d");
    fs::write(d_path.as_path(), b"[core]\na = false")?;

    let paths = vec![a_path, b_path, c_path, d_path];
    let config = File::from_paths(paths, Default::default())?;

    assert_eq!(config.boolean("core", None, "a"), Some(Ok(false)));
    assert_eq!(config.boolean("core", None, "b"), Some(Ok(true)));
    assert_eq!(config.boolean("core", None, "c"), Some(Ok(true)));
    assert_eq!(config.num_values(), 4);

    Ok(())
}

#[test]
fn multiple_paths_multi_value() -> crate::Result {
    let dir = tempdir()?;

    let a_path = dir.path().join("a");
    fs::write(a_path.as_path(), b"[core]\nkey = a")?;

    let b_path = dir.path().join("b");
    fs::write(b_path.as_path(), b"[core]\nkey = b")?;

    let c_path = dir.path().join("c");
    fs::write(c_path.as_path(), b"[core]\nkey = c")?;

    let d_path = dir.path().join("d");
    fs::write(d_path.as_path(), b"[include]\npath = d_path")?;

    let e_path = dir.path().join("e");
    fs::write(e_path.as_path(), b"[include]\npath = e_path")?;

    let paths = vec![a_path, b_path, c_path, d_path, e_path];
    let config = File::from_paths(paths, Default::default())?;

    assert_eq!(
        config.strings("core", None, "key"),
        Some(vec![cow_str("a"), cow_str("b"), cow_str("c"),])
    );

    assert_eq!(
        config.strings("include", None, "path"),
        Some(vec![cow_str("d_path"), cow_str("e_path")])
    );

    assert_eq!(config.num_values(), 5);
    Ok(())
}

mod includes {
    mod conditional;
    mod unconditional;
}
