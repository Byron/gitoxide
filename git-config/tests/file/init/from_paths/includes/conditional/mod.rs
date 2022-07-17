use std::{fs, path::Path};

use git_config::file::init;
use git_config::file::init::includes;
use git_config::{path, File};
use tempfile::tempdir;

use crate::file::{cow_str, init::from_paths::escape_backslashes};

mod gitdir;
mod onbranch;

#[test]
fn include_and_includeif_correct_inclusion_order() -> crate::Result {
    let dir = tempdir()?;
    let config_path = dir.path().join("p");
    let first_include_path = dir.path().join("first-incl");
    let second_include_path = dir.path().join("second-incl");
    let include_if_path = dir.path().join("incl-if");
    fs::write(
        first_include_path.as_path(),
        "
[core]
  b = first-incl-path",
    )?;

    fs::write(
        second_include_path.as_path(),
        "
[core]
  b = second-incl-path",
    )?;

    fs::write(
        include_if_path.as_path(),
        "
[core]
  b = incl-if-path",
    )?;

    fs::write(
        config_path.as_path(),
        format!(
            r#"
[core]
[include]
  path = {}
[includeIf "gitdir:p/"]
  path = {}
[include]
  path = {}"#,
            escape_backslashes(&first_include_path),
            escape_backslashes(&include_if_path),
            escape_backslashes(&second_include_path),
        ),
    )?;

    let dir = config_path.join(".git");
    let config = File::from_paths_metadata(
        Some(git_config::file::Metadata::try_from_path(
            &config_path,
            git_config::Source::Api,
        )?),
        options_with_git_dir(&dir),
    )?;

    assert_eq!(
        config.strings("core", None, "b"),
        Some(vec![
            cow_str("first-incl-path"),
            cow_str("incl-if-path"),
            cow_str("second-incl-path")
        ]),
        "first include is matched correctly",
    );
    assert_eq!(
        config.string("core", None, "b"),
        Some(cow_str("second-incl-path")),
        "second include is matched after incl-if",
    );
    Ok(())
}

fn options_with_git_dir(git_dir: &Path) -> init::Options<'_> {
    init::Options {
        includes: includes::Options::follow(
            path::interpolate::Context {
                home_dir: Some(git_dir.parent().unwrap()),
                ..Default::default()
            },
            includes::conditional::Context {
                git_dir: Some(git_dir),
                ..Default::default()
            },
        ),
        ..Default::default()
    }
}

fn create_symlink(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    std::fs::create_dir_all(from.as_ref().parent().unwrap()).unwrap();
    #[cfg(not(windows))]
    std::os::unix::fs::symlink(to, from).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(to, from).unwrap();
}
