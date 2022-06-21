#[test]
fn relative_path_with_trailing_slash_matches_like_star_star() {
    assert_section_value(Condition::new("gitdir:worktree/"), GitEnv::repo_name("worktree"));
}

#[test]
fn relative_path_without_trailing_slash_does_not_match() {
    assert_section_value(
        Condition::new("gitdir:worktree").expect_original_value(),
        GitEnv::repo_name("worktree"),
    );
}

#[test]
fn relative_path_without_trailing_slash_and_dot_git_suffix_matches() {
    assert_section_value(Condition::new("gitdir:worktree/.git"), GitEnv::repo_name("worktree"));
}

#[test]
fn tilde_slash_expands_the_current_user_home() {
    let env = GitEnv::repo_name("subdir/worktree");
    assert_section_value(Condition::new("gitdir:~/subdir/worktree/"), env);
}

#[test]
fn tilde_alone_does_not_match_even_if_home_is_git_directory() {
    let env = GitEnv::repo_in_home();
    assert_section_value(Condition::new("gitdir:~").expect_original_value(), env);
}

#[test]
fn explicit_star_star_prefix_and_suffix_match_zero_or_more_path_components() {
    assert_section_value(Condition::new("gitdir:**/worktree/**"), GitEnv::repo_name("worktree"));
}

#[test]
fn dot_slash_path_is_replaced_with_directory_containing_the_including_config_file() {
    // TODO: understand this
    assert_section_value(
        Condition::new("gitdir:./").set_user_config_instead_of_repo_config(),
        GitEnv::repo_name("worktree"),
    );
}

#[test]
#[serial_test::serial]
#[ignore]
fn dot_slash_from_environment_causes_error() {
    use git_config::file::from_paths;
    // TODO: figure out how to do this, how do we parse sub-keys? Can git do that even?
    //       If git can't do it, we are also not able to and that's fine (currently unreachable!()).
    let _env = crate::file::from_env::Env::new()
        .set("GIT_CONFIG_COUNT", "1")
        .set("GIT_CONFIG_KEY_0", "includeIf.path")
        .set("GIT_CONFIG_VALUE_0", "some_git_config");

    let res = git_config::File::from_env(from_paths::Options::default());
    assert!(matches!(
        res,
        Err(git_config::file::from_env::Error::FromPathsError(
            from_paths::Error::MissingConfigPath
        ))
    ));
}

#[test]
fn dot_dot_slash_prefixes_are_not_special_and_are_not_what_you_want() {
    assert_section_value(
        Condition::new("gitdir:../")
            .set_user_config_instead_of_repo_config()
            .expect_no_value(),
        GitEnv::repo_name("worktree"),
    );
}

#[test]
#[ignore]
fn leading_dots_are_not_special() {
    // TODO: write this test so that it could fail - right now it's naturally correct
    assert_section_value(
        Condition::new("gitdir:.hidden/").expect_original_value(),
        GitEnv::repo_name(".hidden"),
    );
}

#[test]
fn dot_slash_path_with_dot_git_suffix_matches() {
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
#[cfg(not(windows))]
fn tilde_expansion_with_symlink() {
    let env = git_env_with_symlinked_repo();
    assert_section_value(Condition::new("gitdir:~/symlink-worktree/"), env);
}

#[test]
#[cfg(not(windows))]
fn dot_path_with_symlink() {
    let env = git_env_with_symlinked_repo();
    assert_section_value(
        Condition::new("gitdir:./symlink-worktree/.git").set_user_config_instead_of_repo_config(),
        env,
    );
}

#[test]
#[cfg(not(windows))]
fn relative_path_matching_symlink() {
    let env = git_env_with_symlinked_repo();
    assert_section_value(
        Condition::new("gitdir:symlink-worktree/").set_user_config_instead_of_repo_config(),
        env,
    );
}

#[test]
#[cfg(not(windows))]
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
    use std::io::Write;
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
        value: Option<Value>,
        config_location: ConfigLocation,
    }

    impl Condition {
        pub fn new(condition: impl Into<String>) -> Self {
            Condition {
                condition: condition.into(),
                value: Value::Override.into(),
                config_location: ConfigLocation::Repo,
            }
        }
        pub fn set_user_config_instead_of_repo_config(mut self) -> Self {
            self.config_location = ConfigLocation::User;
            self
        }
        pub fn expect_original_value(mut self) -> Self {
            self.value = Value::Original.into();
            self
        }

        pub fn expect_no_value(mut self) -> Self {
            self.value = None;
            self
        }
    }
    impl GitEnv {
        fn new_in(tempdir: tempfile::TempDir, repo_name: impl AsRef<Path>, home: Option<PathBuf>) -> Self {
            let cwd = std::env::current_dir().unwrap();
            let root_dir = git_path::realpath(tempdir.path(), &cwd).unwrap();
            let git_dir = git_dir(&root_dir, repo_name);
            let home_dir = home
                .map(|home| git_path::realpath(home, cwd).unwrap())
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

        pub fn repo_in_home() -> Self {
            Self::repo_name("")
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

    fn write_config(
        condition: impl AsRef<str>,
        env: GitEnv,
        overwrite_config_location: ConfigLocation,
    ) -> crate::Result<GitEnv> {
        let include_config = write_included_config(&env)?;
        write_main_config(condition, include_config, env, overwrite_config_location)
    }

    fn write_included_config(env: &GitEnv) -> crate::Result<PathBuf> {
        let include_path = env.worktree_dir().join("include.path");
        write_append_config_value(&include_path, "override-value")?;
        Ok(include_path)
    }

    fn write_append_config_value(path: impl AsRef<std::path::Path>, value: &str) -> crate::Result {
        let mut file = std::fs::OpenOptions::new().append(true).create(true).open(path)?;
        file.write_all(
            format!(
                "
[section]
  value = {value}"
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    fn assure_git_agrees(expected: Option<Value>, env: GitEnv) {
        let output = Command::new("git")
            .args(["config", "--get", "section.value"])
            .env("HOME", env.home_dir())
            .env("GIT_DIR", env.git_dir())
            .current_dir(env.worktree_dir())
            .output()
            .unwrap();

        assert_eq!(
            output.status.success(),
            expected.is_some(),
            "{:?}, {:?} for debugging",
            output,
            env.tempdir.into_path()
        );
        let git_output: BString = output.stdout.trim_end().into();
        assert_eq!(
            git_output,
            match expected {
                Some(Value::Original) => "base-value",
                Some(Value::Override) => "override-value",
                None => "",
            },
            "git disagrees with git-config, {:?} for debugging",
            env.tempdir.into_path()
        );
    }

    fn write_main_config(
        condition: impl AsRef<str>,
        include_file_path: PathBuf,
        env: GitEnv,
        overwrite_config_location: ConfigLocation,
    ) -> crate::Result<GitEnv> {
        let output = Command::new("git")
            .args(["init", env.worktree_dir().to_str().unwrap()])
            .output()?;
        assert!(output.status.success(), "git init failed: {:?}", output);

        if overwrite_config_location == ConfigLocation::Repo {
            write_append_config_value(env.git_dir().join("config"), "base-value")?;
        }

        let config_file_path = match overwrite_config_location {
            ConfigLocation::User => env.home_dir().join(".gitconfig"),
            ConfigLocation::Repo => env.git_dir().join("config"),
        };

        let condition = condition.as_ref();
        let include_file_path = escape_backslashes(include_file_path);
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(config_file_path)?;
        file.write_all(
            format!(
                "
[includeIf \"{condition}\"]
  path = {include_file_path}",
            )
            .as_bytes(),
        )?;
        Ok(env)
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
        mut env: GitEnv,
    ) {
        env = write_config(condition, env, config_location).unwrap();

        let mut paths = vec![env.git_dir().join("config")];
        if config_location == ConfigLocation::User {
            paths.push(env.home_dir().join(".gitconfig"));
        }

        let config = git_config::File::from_paths(paths, env.include_options()).unwrap();

        assert_eq!(
            config.string("section", None, "value"),
            match expected {
                Some(Value::Original) => Some(cow_str("base-value")),
                Some(Value::Override) => Some(cow_str("override-value")),
                None => None,
            },
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
