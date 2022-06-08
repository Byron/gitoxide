use crate::file::cow_str;
use crate::file::from_paths::escape_backslashes;
use crate::file::from_paths::includes::conditional::options_with_git_dir;
use bstr::BString;
use git_config::File;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::{tempdir, tempdir_in, TempDir};

#[derive(Eq, PartialEq)]
enum ConfigLocation {
    Repo,
    User,
}

enum Value {
    Original,
    Override,
}

struct Options<'a> {
    condition: &'a str,
    git_dir: &'a Path,
    expected: Value,
    config_location: ConfigLocation,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            condition: "",
            git_dir: Path::new(""),
            expected: Value::Override,
            config_location: ConfigLocation::Repo,
        }
    }
}

fn assert_section_value(
    Options {
        condition,
        git_dir,
        expected,
        config_location,
    }: Options,
) {
    dbg!(git_dir);

    write_config(condition, git_dir, &config_location);

    git_assert_eq(&expected, git_dir, &config_location);

    let mut paths = vec![git_dir.join("config")];
    if config_location == ConfigLocation::User {
        paths.push(git_dir.parent().unwrap().parent().unwrap().join(".gitconfig"));
    }

    let config = File::from_paths(paths, options_with_git_dir(git_dir)).unwrap();

    assert_eq!(
        config.string("section", None, "value"),
        Some(cow_str(match expected {
            Value::Original => "base-value",
            Value::Override => "override-value",
        })),
        "gitoxide assert equals",
    );
}

fn git_dir(root_dir: &TempDir, git_dir_suffix: &str) -> Box<Path> {
    let git_dir = root_dir.path().join(git_dir_suffix).join(".git");
    let git_dir = git_dir.as_path();
    fs::create_dir_all(git_dir).unwrap();
    git_dir.into()
}

fn write_config(condition: &str, git_dir: &Path, overwrite_config_location: &ConfigLocation) {
    let override_config_dir_file = write_override_config(git_dir.parent().unwrap());
    write_main_config(condition, override_config_dir_file, git_dir, overwrite_config_location);
}

fn write_override_config(root_path: &Path) -> PathBuf {
    let include_path = root_path.join("include.path");
    fs::create_dir_all(root_path).unwrap();
    fs::write(
        include_path.as_path(),
        "
[section]
  value = override-value",
    )
    .unwrap();
    include_path
}

fn git_assert_eq(expected: &Value, git_dir: &Path, overwrite_config_location: &ConfigLocation) {
    let output = Command::new("git")
        .args(["config", "--get", "section.value"])
        .env(
            "HOME",
            match overwrite_config_location {
                ConfigLocation::User => git_dir
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                ConfigLocation::Repo => std::env::var("HOME").unwrap(),
            },
        )
        .current_dir(git_dir)
        .output()
        .unwrap();

    dbg!(&git_dir);
    assert_eq!(output.stderr.len(), 0);
    let git_output = BString::from(output.stdout.strip_suffix(b"\n").unwrap());
    assert_eq!(
        git_output,
        match expected {
            Value::Original => "base-value",
            Value::Override => "override-value",
        },
        "git assert equals"
    );
}

fn write_main_config(
    condition: &str,
    override_config_dir_file: PathBuf,
    git_dir: &Path,
    overwrite_config_location: &ConfigLocation,
) {
    let repo_dir = git_dir.parent().unwrap();
    Command::new("git")
        .args(["init", repo_dir.to_str().unwrap()])
        .output()
        .unwrap();

    if overwrite_config_location == &ConfigLocation::Repo {
        dbg!(Command::new("git")
            .args(["config", "section.value", "base-value"])
            .output()
            .unwrap());
    }

    dbg!(Command::new("git")
        .args([
            "config",
            match overwrite_config_location {
                ConfigLocation::User => "--global",
                ConfigLocation::Repo => "--local",
            },
            &format!("includeIf.{}.path", condition),
            &escape_backslashes(override_config_dir_file.as_path()),
        ])
        .current_dir(git_dir)
        .env(
            "HOME",
            match overwrite_config_location {
                ConfigLocation::User => git_dir
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                ConfigLocation::Repo => std::env::var("HOME").unwrap(),
            }
        )
        .output()
        .unwrap());

    dbg!(&git_dir);
}

#[test]
#[serial]
fn relative_path_with_trailing_slash() {
    assert_section_value(Options {
        condition: "gitdir:foo/",
        git_dir: &git_dir(&tempdir().unwrap(), "foo"),
        ..Default::default()
    });
}

#[test]
#[serial]
fn tilde_expansion() {
    let tmp_dir = tempdir_in(std::env::var("HOME").unwrap()).unwrap();
    let root = tmp_dir
        .path()
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();

    assert_section_value(Options {
        condition: &format!("gitdir:~/{}/foo/", root),
        git_dir: &git_dir(&tmp_dir, "foo"),
        ..Default::default()
    });
}

#[test]
#[serial]
fn star_star_prefix_and_suffix() {
    assert_section_value(Options {
        condition: "gitdir:**/foo/**",
        git_dir: &git_dir(&tempdir().unwrap(), "foo"),
        ..Default::default()
    });
}

#[test]
#[serial]
fn dot_path() {
    assert_section_value(Options {
        condition: "gitdir:./foo/.git",
        git_dir: &git_dir(&tempdir().unwrap(), "foo"),
        config_location: ConfigLocation::User,
        ..Default::default()
    });
}

#[test]
#[serial]
fn case_insensitive() {
    assert_section_value(Options {
        condition: "gitdir/i:FOO/",
        git_dir: &git_dir(&tempdir().unwrap(), "foo"),
        ..Default::default()
    });
}

#[test]
#[serial]
fn star_star_in_the_middle() {
    assert_section_value(Options {
        condition: "gitdir:**/foo/**/bar/**",
        git_dir: &git_dir(&tempdir().unwrap(), "foo/bar"),
        ..Default::default()
    });
}
