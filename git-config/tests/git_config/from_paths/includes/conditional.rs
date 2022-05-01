use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};

use crate::git_config::cow_str;
use crate::git_config::from_paths::escape_backslashes;
use git_config::file::{from_paths, GitConfig};
use git_ref::FullName;
use tempfile::tempdir;

#[test]
fn girdir_and_onbranch() {
    let dir = tempdir().unwrap();

    let config_path = dir.path().join("a");
    let absolute_path = dir.path().join("b");
    let home_dot_git_path = dir.path().join("c");
    let home_trailing_slash_path = dir.path().join("c_slash");
    let relative_dot_git_path2 = dir.path().join("d");
    let relative_path = dir.path().join("e");
    let casei_path = dir.path().join("i");
    let relative_dot_slash_path = dir.path().join("g");
    let relative_dot_git_path = dir.path().join("w");
    let relative_with_backslash_path = dir.path().join("x");
    let branch_path = dir.path().join("branch");

    fs::write(
        config_path.as_path(),
        format!(
            r#"
[core]
  x = 1
  a = 1
  b = 1
  c = 1
  i = 1
[includeIf "onbranch:/br/"]
  path = {}
[includeIf "gitdir/i:a/B/c/D/"]
  path = {}
[includeIf "gitdir:c\\d"]
  path = {}
[includeIf "gitdir:./p/"]
  path = {}
[includeIf "gitdir:z/y/"]
  path = {}
[includeIf "gitdir:w/.git"]
  path = {}
[includeIf "gitdir:~/.git"]
  path = {}
[includeIf "gitdir:~/c/"]
  path = {}
[includeIf "gitdir:a/.git"]
  path = {}
[includeIf "gitdir:/e/x/"]
  path = {}"#,
            escape_backslashes(&branch_path),
            escape_backslashes(&casei_path),
            escape_backslashes(&relative_with_backslash_path),
            escape_backslashes(&relative_dot_slash_path),
            escape_backslashes(&relative_path),
            escape_backslashes(&relative_dot_git_path),
            escape_backslashes(&home_dot_git_path),
            escape_backslashes(&home_trailing_slash_path),
            escape_backslashes(&relative_dot_git_path2),
            escape_backslashes(&absolute_path)
        ),
    )
    .unwrap();

    fs::write(
        branch_path.as_path(),
        "
[core]
  x = branch-override",
    )
    .unwrap();

    fs::write(
        casei_path.as_path(),
        "
[core]
  i = case-i-match",
    )
    .unwrap();

    fs::write(
        relative_with_backslash_path.as_path(),
        "
[core]
  c = relative with backslash do not match",
    )
    .unwrap();

    fs::write(
        absolute_path.as_path(),
        "
[core]
  b = absolute-path",
    )
    .unwrap();

    fs::write(
        home_dot_git_path.as_path(),
        "
[core]
  b = home-dot-git",
    )
    .unwrap();

    fs::write(
        relative_dot_git_path2.as_path(),
        "
[core]
  b = relative-dot-git-2",
    )
    .unwrap();

    fs::write(
        relative_path.as_path(),
        "
[core]
  a = relative-path",
    )
    .unwrap();

    fs::write(
        relative_dot_git_path.as_path(),
        "
[core]
  a = relative-dot-git",
    )
    .unwrap();

    fs::write(
        home_trailing_slash_path.as_path(),
        "
[core]
  b = home-trailing-slash",
    )
    .unwrap();

    fs::write(
        relative_dot_slash_path.as_path(),
        "
[core]
  b = relative-dot-slash-path",
    )
    .unwrap();

    {
        let branch_name = FullName::try_from("refs/heads/repo/br/one").unwrap();
        let branch_name = branch_name.to_ref();
        let options = from_paths::Options {
            branch_name: Some(branch_name),
            ..Default::default()
        };

        let config = GitConfig::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "x"),
            Some(cow_str("branch-override")),
            "branch name match"
        );
    }

    {
        let options = from_paths::Options {
            git_dir: Some(Path::new("/a/b/c/d/.git")),
            ..Default::default()
        };

        let config = GitConfig::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "i"),
            Some(cow_str("case-i-match")),
            "case insensitive patterns match"
        );
    }

    {
        let options = from_paths::Options {
            git_dir: Some(Path::new("/a/b/c/d/.git")),
            ..Default::default()
        };

        let config = GitConfig::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.integer("core", None, "c"),
            Some(Ok(1)),
            "patterns with backslashes do not match"
        );
    }

    {
        let a_p_path = config_path.parent().unwrap().join("p").join("q").join(".git");
        let options = from_paths::Options {
            git_dir: Some(a_p_path.as_path()),
            ..Default::default()
        };

        let config = GitConfig::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "b"),
            Some(cow_str("relative-dot-slash-path")),
            "relative path pattern is matched correctly"
        );
    }

    {
        let a_z_y_b_path = config_path.join("z").join("y").join("b").join(".git");
        let options = from_paths::Options {
            git_dir: Some(a_z_y_b_path.as_path()),
            ..Default::default()
        };

        let config = GitConfig::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "a"),
            Some(cow_str("relative-path")),
            "the pattern is prefixed and suffixed with ** to match GIT_DIR containing it in the middle"
        );
    }

    {
        let dir = PathBuf::from("C:\\w\\.git".to_string());
        let config = GitConfig::from_paths(Some(&config_path), options_with_git_dir(&dir)).unwrap();
        assert_eq!(
            config.string("core", None, "a"),
            Some(cow_str("relative-dot-git")),
            "backslashes in GIT_DIR are converted to forward slashes"
        );
    }

    {
        let dir = dirs::home_dir().unwrap().join(".git");
        let config = GitConfig::from_paths(Some(&config_path), options_with_git_dir(&dir)).unwrap();
        assert_eq!(
            config.strings("core", None, "b"),
            Some(vec![cow_str("1"), cow_str("home-dot-git")]),
            "tilde ~ path is resolved to home directory"
        );
    }

    {
        let home_git_path = dirs::home_dir().unwrap().join("c").join("d").join(".git");
        let options = from_paths::Options {
            git_dir: Some(home_git_path.as_path()),
            ..Default::default()
        };
        let config = GitConfig::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "b"),
            Some(cow_str("home-trailing-slash")),
            "path with trailing slash is matched"
        );
    }

    {
        let x_a_path = dir.path().join("x").join("a").join(".git");
        let options = from_paths::Options {
            git_dir: Some(x_a_path.as_path()),
            ..Default::default()
        };
        let config = GitConfig::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "b"),
            Some(cow_str("relative-dot-git-2")), // TODO: figure out what's the difference to the non -2 version
            "** is prepended so paths ending with the pattern are matched"
        );
    }

    {
        let e_x_y_path = PathBuf::from("/e/x/y/.git");
        let options = from_paths::Options {
            git_dir: Some(e_x_y_path.as_path()),
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![config_path], options).unwrap();
        assert_eq!(
            config.string("core", None, "b"),
            Some(cow_str("absolute-path")),
            "absolute path pattern is matched with sub path from GIT_DIR"
        );
    }
}

fn options_with_git_dir(git_dir: &Path) -> from_paths::Options<'_> {
    from_paths::Options {
        git_dir: Some(git_dir),
        ..Default::default()
    }
}
