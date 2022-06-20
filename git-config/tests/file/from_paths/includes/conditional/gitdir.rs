#[test]
fn relative_path_with_trailing_slash() {
    assert_section_value(Condition::new("gitdir:worktree/"), GitEnv::repo_name("worktree"));
}

#[test]
fn tilde_expansion() {
    let env = GitEnv::repo_name("subdir/worktree");
    assert_section_value(Condition::new("gitdir:~/subdir/worktree/"), env);
}

#[test]
fn star_star_prefix_and_suffix() {
    assert_section_value(Condition::new("gitdir:**/worktree/**"), GitEnv::repo_name("worktree"));
}

#[test]
fn dot_path_slash() {
    assert_section_value(
        Condition::new("gitdir:./").set_user_config_instead_of_repo_config(),
        GitEnv::repo_name("worktree"),
    );
}

#[test]
fn dot_path() {
    assert_section_value(
        Condition::new("gitdir:./worktree/.git").set_user_config_instead_of_repo_config(),
        GitEnv::repo_name("worktree"),
    );
}

#[test]
fn case_insensitive() {
    assert_section_value(Condition::new("gitdir/i:WORKTREE/"), GitEnv::repo_name("worktree"));
}

#[test]
#[ignore]
fn pattern_with_backslash() {
    assert_section_value(
        Condition::new(r#"gitdir:\worktree/"#).expect_original_value(),
        GitEnv::repo_name("worktree"),
    );
}

#[test]
fn star_star_in_the_middle() {
    assert_section_value(
        Condition::new("gitdir:**/dir/**/worktree/**"),
        GitEnv::repo_name("dir/worktree"),
    );
}

#[test]
#[cfg(not(target_os = "windows"))]
fn tilde_expansion_with_symlink() {
    let env = git_env_with_symlinked_repo();
    assert_section_value(Condition::new("gitdir:~/symlink-worktree/"), env);
}

#[test]
#[cfg(not(target_os = "windows"))]
fn dot_path_with_symlink() {
    let env = git_env_with_symlinked_repo();
    assert_section_value(
        Condition::new("gitdir:./symlink-worktree/.git").set_user_config_instead_of_repo_config(),
        env,
    );
}

#[test]
#[cfg(not(target_os = "windows"))]
fn relative_path_matching_symlink() {
    let env = git_env_with_symlinked_repo();
    assert_section_value(
        Condition::new("gitdir:symlink-worktree/").set_user_config_instead_of_repo_config(),
        env,
    );
}

#[test]
#[cfg(not(target_os = "windows"))]
fn dot_path_matching_symlink_with_icase() {
    let env = git_env_with_symlinked_repo();
    assert_section_value(
        Condition::new("gitdir/i:SYMLINK-WORKTREE/").set_user_config_instead_of_repo_config(),
        env,
    );
}

mod util {
    use crate::file::cow_str;
    use crate::file::from_paths::escape_backslashes;
    use crate::file::from_paths::includes::conditional::{create_symlink, options_with_git_dir};
    use bstr::{BString, ByteSlice};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    #[derive(Debug)]
    pub struct GitEnv {
        tempdir: tempfile::TempDir,
        root_dir: PathBuf,
        git_dir: PathBuf,
        home_dir: PathBuf,
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    enum ConfigLocation {
        Repo,
        User,
    }

    #[derive(Copy, Clone)]
    enum Value {
        Original,
        Override,
    }

    pub struct Condition {
        condition: String,
        value: Value,
        config_location: ConfigLocation,
    }

    impl Condition {
        pub fn new(condition: impl Into<String>) -> Self {
            Condition {
                condition: condition.into(),
                value: Value::Override,
                config_location: ConfigLocation::Repo,
            }
        }
        pub fn set_user_config_instead_of_repo_config(mut self) -> Self {
            self.config_location = ConfigLocation::User;
            self
        }
        pub fn expect_original_value(mut self) -> Self {
            self.value = Value::Original;
            self
        }
    }
    impl GitEnv {
        fn new_in(tempdir: tempfile::TempDir, repo_name: impl AsRef<Path>, home: Option<PathBuf>) -> Self {
            let root_dir = tempdir.path().canonicalize().unwrap();
            let git_dir = git_dir(&root_dir, repo_name);
            let home_dir = home
                .map(|home| home.canonicalize().unwrap())
                .unwrap_or_else(|| root_dir.clone());
            Self {
                tempdir,
                root_dir,
                git_dir,
                home_dir,
            }
        }

        fn include_options(&self) -> git_config::file::from_paths::Options {
            let mut opts = options_with_git_dir(self.git_dir());
            opts.home_dir = Some(self.home_dir());
            opts
        }
    }

    impl GitEnv {
        pub fn repo_name(repo_name: impl AsRef<Path>) -> Self {
            let tempdir = tempfile::tempdir().unwrap();
            let home = tempdir.path().to_owned();
            Self::new_in(tempdir, repo_name, Some(home))
        }

        pub fn git_dir(&self) -> &Path {
            &self.git_dir
        }
        pub fn set_git_dir(&mut self, git_dir: PathBuf) {
            self.git_dir = git_dir;
        }
        pub fn worktree_dir(&self) -> &Path {
            self.git_dir.parent().unwrap()
        }
        pub fn home_dir(&self) -> &Path {
            &self.home_dir
        }
        pub fn root_dir(&self) -> &Path {
            &self.root_dir
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

    fn assure_git_agrees(expected: Value, env: GitEnv) {
        let output = Command::new("git")
            .args(["config", "--get", "section.value"])
            .env("HOME", env.home_dir())
            .env("GIT_DIR", env.git_dir())
            .current_dir(env.worktree_dir())
            .output()
            .unwrap();

        assert!(
            output.status.success(),
            "{:?}, {:?} for debugging",
            output,
            env.tempdir.into_path()
        );
        let git_output: BString = output.stdout.trim_end().into();
        assert_eq!(
            git_output,
            match expected {
                Value::Original => "base-value",
                Value::Override => "override-value",
            },
            "git disagrees with git-config, {:?} for debugging",
            env.tempdir.into_path()
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
            .current_dir(env.worktree_dir())
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
        Condition {
            condition,
            value: expected,
            config_location,
        }: Condition,
        env: GitEnv,
    ) {
        write_config(condition, &env, config_location);

        let mut paths = vec![env.git_dir().join("config")];
        if config_location == ConfigLocation::User {
            paths.push(env.home_dir().join(".gitconfig"));
        }

        let config = git_config::File::from_paths(paths, env.include_options()).unwrap();

        assert_eq!(
            config.string("section", None, "value"),
            Some(cow_str(match expected {
                Value::Original => "base-value",
                Value::Override => "override-value",
            })),
            "git-config disagrees with the expected value, {:?} for debugging",
            env.tempdir.into_path()
        );
        assure_git_agrees(expected, env);
    }

    pub fn git_env_with_symlinked_repo() -> GitEnv {
        let mut env = GitEnv::repo_name("worktree");
        let link_destination = env.root_dir().join("symlink-worktree");
        create_symlink(&link_destination, env.worktree_dir());

        let git_dir_through_symlink = link_destination.join(".git");
        env.set_git_dir(git_dir_through_symlink);
        env
    }
}
use util::{assert_section_value, git_env_with_symlinked_repo, Condition, GitEnv};
