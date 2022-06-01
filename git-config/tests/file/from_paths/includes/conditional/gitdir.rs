use crate::file::cow_str;
use crate::file::from_paths::escape_backslashes;
use crate::file::from_paths::includes::conditional::options_with_git_dir;
use bstr::BString;
use git_config::File;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

enum Value {
    Original,
    Override,
}

struct Options<'a> {
    condition: &'a str,
    git_dir_suffix: &'a str,
    expected: Value,
}

fn assert_section_value(
    Options {
        condition,
        git_dir_suffix,
        expected,
    }: Options,
) {
    let dir = tempdir().unwrap();
    let dir = dir.path().join(git_dir_suffix);
    fs::create_dir_all(dir.as_path()).unwrap();
    dbg!(&dir);

    let real_git_dir = dir.join(".git");
    let git_config = real_git_dir.join("config");

    write_config(condition, dir.as_path(), real_git_dir.as_path());

    git_assert_eq(&expected, real_git_dir.as_path());

    let config = File::from_paths(Some(git_config), options_with_git_dir(real_git_dir.as_path())).unwrap();

    assert_eq!(
        config.string("section", None, "value"),
        Some(cow_str(match expected {
            Value::Original => "base-value",
            Value::Override => "override-value",
        })),
        "relative path pattern is matched correctly"
    );
}

fn write_config(condition: &str, dir: &Path, real_git_dir: &Path) {
    let include_path = write_override_config(dir);
    write_main_config(condition, dir, include_path.as_path(), real_git_dir);
}

fn write_override_config(dir: &Path) -> PathBuf {
    let include_path = dir.join("include.path");
    fs::write(
        include_path.as_path(),
        "
[section]
  value = override-value",
    )
    .unwrap();
    include_path
}

fn git_assert_eq(expected: &Value, git_dirrr: &Path) {
    let output = Command::new("git")
        .args(["config", "--get", "section.value"])
        .env_clear()
        .env("GIT_DIR", git_dirrr)
        .output()
        .unwrap();

    assert_eq!(output.stderr, Vec::new());
    assert_eq!(
        BString::from(output.stdout.strip_suffix(b"\n").unwrap()),
        match expected {
            Value::Original => "base-value",
            Value::Override => "override-value",
        }
    );
}

fn write_main_config(condition: &str, dir: &Path, include_path: &Path, git_dirrr: &Path) {
    Command::new("git")
        .args(["init", dir.to_str().unwrap()])
        .output()
        .unwrap();

    Command::new("git")
        .args(["config", "section.value", "base-value"])
        .env_clear()
        .env("GIT_DIR", git_dirrr)
        .output()
        .unwrap();

    Command::new("git")
        .args([
            "config",
            &format!("includeIf.gitdir:{}.path", condition),
            &escape_backslashes(include_path),
        ])
        .env_clear()
        .env("GIT_DIR", git_dirrr)
        .output()
        .unwrap();
}

#[test]
fn dot_slash_match() {
    assert_section_value(Options {
        condition: "./",
        git_dir_suffix: ".git",
        expected: Value::Override,
    });
}

#[test]
fn relative_path_with_backslash_do_not_match() {
    assert_section_value(Options {
        condition: "c\\\\d/",
        git_dir_suffix: "c//d/.git",
        expected: Value::Original,
    });
}

#[test]
fn relative_path_match() {
    assert_section_value(Options {
        condition: "foo/bar/.git",
        git_dir_suffix: "foo/bar",
        expected: Value::Override,
    });
}

#[test]
fn dot_dot_path_match() {
    assert_section_value(Options {
        condition: "..",
        git_dir_suffix: ".",
        expected: Value::Override,
    });
}
