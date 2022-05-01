use std::convert::TryFrom;
use std::path::PathBuf;
use std::{borrow::Cow, fs};

use crate::git_config::from_paths::escape_backslashes;
use git_config::file::{from_paths, GitConfig};
use git_ref::FullName;
use tempfile::tempdir;

#[test]
fn girdir_and_onbranch() {
    let dir = tempdir().unwrap();

    let a_path = dir.path().join("a");
    let b_path = dir.path().join("b");
    let c_path = dir.path().join("c");
    let c_slash_path = dir.path().join("c_slash");
    let d_path = dir.path().join("d");
    let e_path = dir.path().join("e");
    let i_path = dir.path().join("i");
    let g_path = dir.path().join("g");
    let w_path = dir.path().join("w");
    let x_path = dir.path().join("x");
    let branch_path = dir.path().join("branch");

    fs::write(
        a_path.as_path(),
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
            escape_backslashes(&i_path),
            escape_backslashes(&x_path),
            escape_backslashes(&g_path),
            escape_backslashes(&e_path),
            escape_backslashes(&w_path),
            escape_backslashes(&c_path),
            escape_backslashes(&c_slash_path),
            escape_backslashes(&d_path),
            escape_backslashes(&b_path)
        ),
    )
    .unwrap();

    fs::write(
        branch_path.as_path(),
        "
[core]
  x = 7",
    )
    .unwrap();

    fs::write(
        i_path.as_path(),
        "
[core]
  i = 3",
    )
    .unwrap();

    fs::write(
        x_path.as_path(),
        "
[core]
  c = 5",
    )
    .unwrap();

    fs::write(
        b_path.as_path(),
        "
[core]
  b = 2",
    )
    .unwrap();

    fs::write(
        c_path.as_path(),
        "
[core]
  b = 3",
    )
    .unwrap();

    fs::write(
        d_path.as_path(),
        "
[core]
  b = 4",
    )
    .unwrap();

    fs::write(
        e_path.as_path(),
        "
[core]
  a = 5",
    )
    .unwrap();

    fs::write(
        w_path.as_path(),
        "
[core]
  a = 6",
    )
    .unwrap();

    fs::write(
        c_slash_path.as_path(),
        "
[core]
  b = 7",
    )
    .unwrap();

    fs::write(
        g_path.as_path(),
        "
[core]
  b = 8",
    )
    .unwrap();

    let branch_name = FullName::try_from("refs/heads/repo/br/one").unwrap();
    let branch_name = branch_name.to_ref();
    let options = from_paths::Options {
        branch_name: Some(branch_name),
        ..Default::default()
    };

    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "x").unwrap(),
        Cow::<[u8]>::Borrowed(b"7"),
        "branch name match"
    );

    let a_c_d_path = PathBuf::from("/a/b/c/d/.git");
    let options = from_paths::Options {
        git_dir: Some(a_c_d_path.as_path()),
        ..Default::default()
    };

    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "i").unwrap(),
        Cow::<[u8]>::Borrowed(b"3"),
        "case insensitive patterns match"
    );

    let a_c_d_path = PathBuf::from("/a/c/d/.git");
    let options = from_paths::Options {
        git_dir: Some(a_c_d_path.as_path()),
        ..Default::default()
    };

    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "c").unwrap(),
        Cow::<[u8]>::Borrowed(b"1"),
        "patterns with backslashes do not match"
    );

    let a_p_path = a_path.parent().unwrap().join("p").join("q").join(".git");
    let options = from_paths::Options {
        git_dir: Some(a_p_path.as_path()),
        ..Default::default()
    };

    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "b").unwrap(),
        Cow::<[u8]>::Borrowed(b"8"),
        "relative path pattern is matched correctly"
    );

    let a_z_y_b_path = a_path.join("z").join("y").join("b").join(".git");
    let options = from_paths::Options {
        git_dir: Some(a_z_y_b_path.as_path()),
        ..Default::default()
    };

    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"5"),
        "the pattern is prefixed and suffixed with ** to match GIT_DIR containing it in the middle"
    );

    let cw_path = PathBuf::from("C:\\w\\.git".to_string());
    let options = from_paths::Options {
        git_dir: Some(cw_path.as_path()),
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"6"),
        "backslashes in GIT_DIR are converted to forward slashes"
    );

    let home_git_path = dirs::home_dir().unwrap().join(".git");
    let options = from_paths::Options {
        git_dir: Some(home_git_path.as_path()),
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_multi_value("core", None, "b").unwrap(),
        vec![Cow::<[u8]>::Borrowed(b"1"), Cow::<[u8]>::Borrowed(b"3")],
        "tilde ~ path is resolved to home directory"
    );

    let home_git_path = dirs::home_dir().unwrap().join("c").join("d").join(".git");
    let options = from_paths::Options {
        git_dir: Some(home_git_path.as_path()),
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "b").unwrap(),
        Cow::<[u8]>::Borrowed(b"7"),
        "path with trailing slash is matched"
    );

    let x_a_path = dir.path().join("x").join("a").join(".git");
    let options = from_paths::Options {
        git_dir: Some(x_a_path.as_path()),
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "b").unwrap(),
        Cow::<[u8]>::Borrowed(b"4"),
        "** is prepended so paths ending with the pattern are matched"
    );

    let e_x_y_path = PathBuf::from("/e/x/y/.git");
    let options = from_paths::Options {
        git_dir: Some(e_x_y_path.as_path()),
        ..Default::default()
    };
    let config = GitConfig::from_paths(vec![a_path], &options).unwrap();
    assert_eq!(
        config.raw_value("core", None, "b").unwrap(),
        Cow::<[u8]>::Borrowed(b"2"),
        "absolute path pattern is matched with sub path from GIT_DIR"
    );
}
