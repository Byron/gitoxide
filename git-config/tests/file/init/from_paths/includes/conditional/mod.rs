use std::{fs, path::Path};

use git_config::file::includes;
use git_config::file::init;
use git_config::{path, File};
use tempfile::tempdir;

use crate::file::{cow_str, init::from_paths::escape_backslashes};

mod gitdir;
mod onbranch;

#[test]
fn include_and_includeif_correct_inclusion_order() -> crate::Result {
    let dir = tempdir()?;
    let config_path = dir.path().join("root");
    let first_include_path = dir.path().join("first-incl");
    let second_include_path = dir.path().join("second-incl");
    let include_if_path = dir.path().join("incl-if");
    fs::write(
        first_include_path.as_path(),
        "
[section]
  value = first-incl-path",
    )?;

    fs::write(
        second_include_path.as_path(),
        "
[section]
  value = second-incl-path",
    )?;

    fs::write(
        include_if_path.as_path(),
        "
[section]
  value = incl-if-path",
    )?;

    let root_config = format!(
        r#"
[section]
    value = base
[include]
  path = {}
[section]
  value = base-past-first-include
[includeIf "gitdir:root/"]
  path = {}
[section]
  value = base-past-includeIf
[include]
  path = {}
[section]
  value = base-past-second-include "#,
        escape_backslashes(&first_include_path),
        escape_backslashes(&include_if_path),
        escape_backslashes(&second_include_path),
    );
    fs::write(config_path.as_path(), &root_config)?;

    let dir = config_path.join(".git");
    for delayed_resolve in [false, true] {
        let meta = git_config::file::Metadata::try_from_path(&config_path, git_config::Source::Api)?;
        let options = options_with_git_dir(&dir);
        let config = if delayed_resolve {
            let mut config = File::from_bytes_owned(&mut root_config.as_bytes().into(), meta, Default::default())?;
            config.resolve_includes(options)?;
            config
        } else {
            File::from_paths_metadata(Some(meta), options)?.expect("non-empty")
        };

        assert_eq!(
            config.strings("section", None, "value"),
            Some(vec![
                cow_str("base"),
                cow_str("first-incl-path"),
                cow_str("base-past-first-include"),
                cow_str("incl-if-path"),
                cow_str("base-past-includeIf"),
                cow_str("second-incl-path"),
                cow_str("base-past-second-include"),
            ]),
            "include order isn't changed also in relation to the root configuratino, delayed_resolve = {}",
            delayed_resolve,
        );

        // TODO: also validate serialization here, with front/post-matter.
    }
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
