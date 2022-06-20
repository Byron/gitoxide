use serial_test::serial;

#[test]
#[serial]
fn relative_path_with_trailing_slash() {
    assert_section_value(GitEnv::repo_name("foo"), Options::new("gitdir:foo/"));
}

#[test]
#[serial]
fn tilde_expansion() {
    let (env, basename) = GitEnv::repo_in_home_named("foo");
    assert_section_value(env, Options::new(format!("gitdir:~/{}/foo/", basename)));
}

#[test]
#[serial]
fn star_star_prefix_and_suffix() {
    assert_section_value(GitEnv::repo_name("foo"), Options::new("gitdir:**/foo/**"));
}

#[test]
#[serial]
fn dot_path_slash() {
    assert_section_value(
        GitEnv::repo_name_with_root_as_home("foo"),
        Options::new("gitdir:./").set_user_config_instead_of_repo_config(),
    );
}

#[test]
#[serial]
fn dot_path() {
    assert_section_value(
        GitEnv::repo_name_with_root_as_home("foo"),
        Options::new("gitdir:./foo/.git").set_user_config_instead_of_repo_config(),
    );
}

#[test]
#[serial]
fn case_insensitive() {
    assert_section_value(GitEnv::repo_name("foo"), Options::new("gitdir/i:FOO/"));
}

#[test]
#[serial]
#[ignore]
fn pattern_with_backslash() {
    assert_section_value(
        GitEnv::repo_name("foo"),
        Options::new(r#"gitdir:\foo/"#).expect_original_value(),
    );
}

#[test]
#[serial]
fn star_star_in_the_middle() {
    assert_section_value(GitEnv::repo_name("foo/bar"), Options::new("gitdir:**/foo/**/bar/**"));
}

#[test]
#[serial]
#[cfg(not(target_os = "windows"))]
fn tilde_expansion_with_symlink() {
    let (env, basename) = git_env_with_symlinked_repo();
    assert_section_value(env, Options::new(format!("gitdir:~/{}/symlink-foo/", basename)));
}

#[test]
#[serial]
#[cfg(not(target_os = "windows"))]
#[ignore]
fn dot_path_with_symlink() {
    let mut env = GitEnv::repo_name_with_root_as_home("foo");
    let link_destination = env.root_dir().join("symlink-foo");
    crate::file::from_paths::includes::conditional::create_symlink(&link_destination, env.worktree_dir());

    let git_dir_through_symlink = link_destination.join(".git");
    env.set_git_dir(git_dir_through_symlink);

    assert_section_value(
        env,
        Options::new("gitdir:./symlink-foo/.git").set_user_config_instead_of_repo_config(),
    );
}

#[test]
#[serial]
#[cfg(not(target_os = "windows"))]
fn relative_path_matching_symlink() {
    let (env, _) = git_env_with_symlinked_repo();
    assert_section_value(
        env,
        Options::new("gitdir:symlink-foo/").set_user_config_instead_of_repo_config(),
    );
}

#[test]
#[serial]
#[cfg(not(target_os = "windows"))]
fn dot_path_matching_symlink_with_icase() {
    let (env, _) = git_env_with_symlinked_repo();
    assert_section_value(
        env,
        Options::new("gitdir/i:SYMLINK-FOO/").set_user_config_instead_of_repo_config(),
    );
}

mod util {
    use crate::file::cow_str;
    use crate::file::from_paths::escape_backslashes;
    use crate::file::from_paths::includes::conditional::{create_symlink, options_with_git_dir};
    use bstr::{BString, ByteSlice};
    use dirs::home_dir;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use tempfile::tempdir_in;

    pub struct GitEnv {
        tempdir: tempfile::TempDir,
        git_dir: PathBuf,
        worktree_dir: PathBuf,
        home_dir: PathBuf,
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum ConfigLocation {
        Repo,
        User,
    }

    #[derive(Copy, Clone)]
    pub enum Value {
        Original,
        Override,
    }

    pub struct Options {
        pub condition: String,
        pub expected: Value,
        pub config_location: ConfigLocation,
    }

    impl Options {
        pub fn new(condition: impl Into<String>) -> Self {
            Options {
                condition: condition.into(),
                expected: Value::Override,
                config_location: ConfigLocation::Repo,
            }
        }
        pub fn set_user_config_instead_of_repo_config(mut self) -> Self {
            self.config_location = ConfigLocation::User;
            self
        }
        pub fn expect_original_value(mut self) -> Self {
            self.expected = Value::Original;
            self
        }
    }

    impl GitEnv {
        pub fn repo_in_home_named(repo_name: impl AsRef<Path>) -> (Self, String) {
            let tempdir = tempdir_in(home_dir().unwrap()).unwrap();
            let basename = tempdir.path().file_name().unwrap().to_str().unwrap().into();
            (Self::new_in(tempdir, repo_name, None), basename)
        }
        pub fn repo_name(repo_name: impl AsRef<Path>) -> Self {
            Self::new_in(tempfile::tempdir().unwrap(), repo_name, None)
        }

        pub fn repo_name_with_root_as_home(repo_name: impl AsRef<Path>) -> Self {
            let tempdir = tempfile::tempdir().unwrap();
            let home = tempdir.path().to_owned();
            Self::new_in(tempdir, repo_name, Some(home))
        }

        fn new_in(tempdir: tempfile::TempDir, repo_name: impl AsRef<Path>, home: Option<PathBuf>) -> Self {
            let git_dir = git_dir(tempdir.path(), repo_name);
            let worktree_dir = git_dir.parent().unwrap().into();
            Self {
                tempdir,
                git_dir,
                worktree_dir,
                home_dir: match home {
                    Some(home) => home,
                    None => home_dir().unwrap(),
                },
            }
        }

        pub fn git_dir(&self) -> &Path {
            &self.git_dir
        }
        pub fn set_git_dir(&mut self, git_dir: PathBuf) {
            self.git_dir = git_dir;
        }
        pub fn worktree_dir(&self) -> &Path {
            &self.worktree_dir
        }
        pub fn home_dir(&self) -> &Path {
            &self.home_dir
        }
        pub fn root_dir(&self) -> &Path {
            self.tempdir.path()
        }
    }

    fn write_config(condition: impl AsRef<str>, env: &GitEnv, overwrite_config_location: ConfigLocation) {
        let override_config_dir_file = write_override_config(env.worktree_dir());
        write_main_config(condition, override_config_dir_file, env, overwrite_config_location);
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

    fn git_assert_eq(expected: Value, env: &GitEnv) {
        let output = Command::new("git")
            .args(["config", "--get", "section.value"])
            .env("HOME", env.home_dir())
            .env("GIT_DIR", env.git_dir())
            .current_dir(env.git_dir())
            .output()
            .unwrap();

        assert!(output.status.success(), "{:?}", output);
        let git_output: BString = output.stdout.trim_end().into();
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
        condition: impl AsRef<str>,
        override_config_dir_file: PathBuf,
        env: &GitEnv,
        overwrite_config_location: ConfigLocation,
    ) {
        let output = Command::new("git")
            .args(["init", env.worktree_dir().to_str().unwrap()])
            .output()
            .unwrap();
        assert!(output.status.success(), "git init failed: {:?}", output);

        if overwrite_config_location == ConfigLocation::Repo {
            let output = Command::new("git")
                .args(["config", "section.value", "base-value"])
                .env("GIT_DIR", env.git_dir())
                .output()
                .unwrap();
            assert!(output.status.success(), "git config set value failed: {:?}", output);
        }

        let output = Command::new("git")
            .args([
                "config",
                match overwrite_config_location {
                    ConfigLocation::User => "--global",
                    ConfigLocation::Repo => "--local",
                },
                &format!("includeIf.{}.path", condition.as_ref()),
                &escape_backslashes(override_config_dir_file.as_path()),
            ])
            .current_dir(env.git_dir())
            .env("HOME", env.home_dir())
            .output()
            .unwrap();
        assert!(output.status.success(), "git config set value failed: {:?}", output);
    }

    fn git_dir(root_dir: &Path, subdir_name: impl AsRef<Path>) -> PathBuf {
        let git_dir = root_dir.join(subdir_name).join(".git");
        std::fs::create_dir_all(&git_dir).unwrap();
        git_dir
    }

    pub fn assert_section_value(
        env: GitEnv,
        Options {
            condition,
            expected,
            config_location,
        }: Options,
    ) {
        write_config(condition, &env, config_location);
        git_assert_eq(expected, &env);

        let mut paths = vec![env.git_dir().join("config")];
        if config_location == ConfigLocation::User {
            paths.push(env.home_dir().join(".gitconfig"));
        }

        let config = git_config::File::from_paths(paths, options_with_git_dir(&env.git_dir())).unwrap();

        assert_eq!(
            config.string("section", None, "value"),
            Some(cow_str(match expected {
                Value::Original => "base-value",
                Value::Override => "override-value",
            })),
            "git-config disagrees with the expected value",
        );
    }

    pub fn git_env_with_symlinked_repo() -> (GitEnv, String) {
        let (mut env, basename) = GitEnv::repo_in_home_named("foo");
        let link_destination = env.root_dir().join("symlink-foo");
        create_symlink(&link_destination, env.worktree_dir());

        let git_dir_through_symlink = link_destination.join(".git");
        env.set_git_dir(git_dir_through_symlink);
        (env, basename)
    }
}
use util::{assert_section_value, git_env_with_symlinked_repo, GitEnv, Options};
