mod util;

use util::{assert_section_value, git_env_with_symlinked_repo, Condition, GitEnv};

#[test]
fn relative_path_with_trailing_slash_matches_like_star_star() -> crate::Result {
    assert_section_value(Condition::new("gitdir:worktree/"), GitEnv::repo_name("worktree")?)
}

#[test]
fn relative_path_without_trailing_slash_does_not_match() -> crate::Result {
    assert_section_value(
        Condition::new("gitdir:worktree").expect_original_value(),
        GitEnv::repo_name("worktree")?,
    )
}

#[test]
fn relative_path_without_trailing_slash_and_dot_git_suffix_matches() -> crate::Result {
    assert_section_value(Condition::new("gitdir:worktree/.git"), GitEnv::repo_name("worktree")?)
}

#[test]
fn tilde_slash_expands_the_current_user_home() -> crate::Result {
    let env = GitEnv::repo_name("subdir/worktree")?;
    assert_section_value(Condition::new("gitdir:~/subdir/worktree/"), env)
}

#[test]
fn tilde_alone_does_not_match_even_if_home_is_git_directory() -> crate::Result {
    let env = GitEnv::repo_in_home()?;
    assert_section_value(Condition::new("gitdir:~").expect_original_value(), env)
}

#[test]
fn explicit_star_star_prefix_and_suffix_match_zero_or_more_path_components() -> crate::Result {
    assert_section_value(Condition::new("gitdir:**/worktree/**"), GitEnv::repo_name("worktree")?)
}

#[test]
fn dot_slash_path_is_replaced_with_directory_containing_the_including_config_file() -> crate::Result {
    // TODO: understand this
    assert_section_value(
        Condition::new("gitdir:./").set_user_config_instead_of_repo_config(),
        GitEnv::repo_name("worktree")?,
    )
}

#[test]
#[serial_test::serial]
#[ignore]
fn dot_slash_from_environment_causes_error() {
    use git_config::file::from_paths;
    // TODO: figure out how to do this, how do we parse sub-keys? Can git do that even? YES, git can actually!
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
fn dot_dot_slash_prefixes_are_not_special_and_are_not_what_you_want() -> crate::Result {
    assert_section_value(
        Condition::new("gitdir:../")
            .set_user_config_instead_of_repo_config()
            .expect_no_value(),
        GitEnv::repo_name("worktree")?,
    )
}

#[test]
#[ignore]
fn leading_dots_are_not_special() -> crate::Result {
    // TODO: write this test so that it could fail - right now it's naturally correct
    assert_section_value(
        Condition::new("gitdir:.hidden/").expect_original_value(),
        GitEnv::repo_name(".hidden")?,
    )
}

#[test]
fn dot_slash_path_with_dot_git_suffix_matches() -> crate::Result {
    assert_section_value(
        Condition::new("gitdir:./worktree/.git").set_user_config_instead_of_repo_config(),
        GitEnv::repo_name("worktree")?,
    )
}

#[test]
fn case_insensitive_matches_any_case() -> crate::Result {
    assert_section_value(Condition::new("gitdir/i:WORKTREE/"), GitEnv::repo_name("worktree")?)?;
    assert_section_value(
        Condition::new("gitdir:WORKTREE/").expect_original_value(),
        GitEnv::repo_name("worktree")?,
    )
}

#[test]
#[ignore]
fn pattern_with_escaped_backslash() -> crate::Result {
    assert_section_value(
        Condition::new(r#"gitdir:\\work\\tree\\/"#),
        GitEnv::repo_name("worktree")?,
    )
}

#[test]
fn pattern_with_backslash() -> crate::Result {
    assert_section_value(Condition::new(r#"gitdir:work\tree/"#), GitEnv::repo_name("worktree")?)
}

#[test]
fn star_star_in_the_middle() -> crate::Result {
    assert_section_value(
        Condition::new("gitdir:**/dir/**/worktree/**"),
        GitEnv::repo_name("dir/worktree")?,
    )
}

#[test]
#[cfg(not(windows))]
fn tilde_expansion_with_symlink() -> crate::Result {
    let env = git_env_with_symlinked_repo()?;
    assert_section_value(Condition::new("gitdir:~/symlink-worktree/"), env)
}

#[test]
#[cfg(not(windows))]
fn dot_path_with_symlink() -> crate::Result {
    let env = git_env_with_symlinked_repo()?;
    assert_section_value(
        Condition::new("gitdir:./symlink-worktree/.git").set_user_config_instead_of_repo_config(),
        env,
    )
}

#[test]
#[cfg(not(windows))]
fn relative_path_matching_symlink() -> crate::Result {
    let env = git_env_with_symlinked_repo()?;
    assert_section_value(
        Condition::new("gitdir:symlink-worktree/").set_user_config_instead_of_repo_config(),
        env,
    )
}

#[test]
#[cfg(not(windows))]
fn dot_path_matching_symlink_with_icase() -> crate::Result {
    let env = git_env_with_symlinked_repo()?;
    assert_section_value(
        Condition::new("gitdir/i:SYMLINK-WORKTREE/").set_user_config_instead_of_repo_config(),
        env,
    )
}
