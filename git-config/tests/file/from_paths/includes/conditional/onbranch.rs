use std::convert::TryInto;
use std::{convert::TryFrom, fs};

use bstr::{BString, ByteSlice};
use git_config::file::from_paths;
use git_ref::transaction::{Change, PreviousValue, RefEdit};
use git_ref::{FullName, Target};
use git_repository as git;
use tempfile::tempdir;

use crate::file::cow_str;

type Result = crate::Result;

#[test]
fn literal_branch_names_match() -> Result {
    assert_section_value(
        Options {
            condition: "literal-match",
            branch_name: "refs/heads/literal-match",
            expect: Value::OverrideByInclude,
        },
        GitEnv::new()?,
    )?;
    Ok(())
}

#[test]
fn full_ref_names_do_not_match() -> Result {
    assert_section_value(
        Options {
            condition: "refs/heads/simple",
            branch_name: "refs/heads/simple",
            expect: Value::Base,
        },
        GitEnv::new()?,
    )?;
    Ok(())
}

#[test]
fn non_branches_never_match() -> Result {
    assert_section_value(
        Options {
            condition: "good",
            branch_name: "refs/bisect/good",
            expect: Value::Base,
        },
        GitEnv::new()?,
    )?;
    Ok(())
}

#[test]
fn patterns_ending_with_slash_match_subdirectories_recursively() -> Result {
    let mut env = GitEnv::new()?;
    env = assert_section_value(
        Options {
            condition: "feature/b/",
            branch_name: "refs/heads/feature/b/start",
            expect: Value::OverrideByInclude,
        },
        env,
    )?;
    env = assert_section_value(
        Options {
            condition: "feature/",
            branch_name: "refs/heads/feature/b/start",
            expect: Value::OverrideByInclude,
        },
        env,
    )?;
    assert_section_value_msg(
        Options {
            condition: "feature/b/start",
            branch_name: "refs/heads/feature/b/start",
            expect: Value::OverrideByInclude,
        },
        env,
        "just for good measure, we would expect branch paths to work as well".into(),
    )?;
    Ok(())
}

#[test]
fn simple_glob_patterns() -> Result {
    let mut env = GitEnv::new()?;
    env = assert_section_value(
        Options {
            condition: "prefix*",
            branch_name: "refs/heads/prefixsuffix",
            expect: Value::OverrideByInclude,
        },
        env,
    )?;
    env = assert_section_value_msg(
        Options {
            condition: "prefix*",
            branch_name: "refs/heads/prefix/suffix",
            expect: Value::Base,
        },
        env,
        "single-stars do not cross component boundaries".into(),
    )?;
    env = assert_section_value(
        Options {
            condition: "*suffix",
            branch_name: "refs/heads/prefixsuffix",
            expect: Value::OverrideByInclude,
        },
        env,
    )?;
    env = assert_section_value(
        Options {
            condition: "*/suffix",
            branch_name: "refs/heads/prefix/suffix",
            expect: Value::OverrideByInclude,
        },
        env,
    )?;
    assert_section_value_msg(
        Options {
            condition: "*suffix",
            branch_name: "refs/heads/prefix/suffix",
            expect: Value::Base,
        },
        env,
        "single-stars do not cross component boundaries".into(),
    )?;
    Ok(())
}

#[test]
fn simple_globs_do_not_cross_component_boundary() -> Result {
    let mut env = GitEnv::new()?;
    env = assert_section_value(
        Options {
            condition: "feature/*/start",
            branch_name: "refs/heads/feature/a/start",
            expect: Value::OverrideByInclude,
        },
        env,
    )?;
    assert_section_value_msg(
        Options {
            condition: "feature/*/start",
            branch_name: "refs/heads/feature/a/b/start",
            expect: Value::Base,
        },
        env,
        "path matching would never match 'a/b' as it cannot cross /".into(),
    )?;
    Ok(())
}

#[test]
fn double_star_globs_cross_component_boundaries() -> Result {
    assert_section_value(
        Options {
            condition: "feature/**/start",
            branch_name: "refs/heads/feature/a/b/start",
            expect: Value::OverrideByInclude,
        },
        GitEnv::new()?,
    )?;
    Ok(())
}

enum Value {
    Base,
    OverrideByInclude,
}

#[derive(Debug)]
struct GitEnv {
    repo: git::Repository,
    dir: tempfile::TempDir,
}

impl GitEnv {
    fn new() -> crate::Result<Self> {
        let dir = tempdir()?;
        let repo = git_repository::init_bare(dir.path())?;
        Ok(GitEnv { repo, dir })
    }
}

struct Options<'a> {
    condition: &'a str,
    branch_name: &'a str,
    expect: Value,
}

fn assert_section_value(opts: Options, env: GitEnv) -> crate::Result<GitEnv> {
    assert_section_value_msg(opts, env, None)
}

fn assert_section_value_msg(
    Options {
        condition,
        branch_name,
        expect,
    }: Options,
    GitEnv { repo, dir }: GitEnv,
    message: Option<&str>,
) -> crate::Result<GitEnv> {
    let root_config = dir.path().join("config");
    let included_config = dir.path().join("include.config");

    fs::write(
        &root_config,
        format!(
            r#"
[section]
value = base-value

[includeIf "onbranch:{}"]
path = ./include.config"#,
            condition,
        ),
    )?;

    fs::write(
        included_config,
        r#"
[section]
value = branch-override-by-include
"#,
    )?;

    let branch_name = FullName::try_from(BString::from(branch_name))?;
    let options = from_paths::Options {
        branch_name: Some(branch_name.as_ref()),
        ..Default::default()
    };

    let config = git_config::File::from_paths(Some(&root_config), options)?;
    assert_eq!(
        config.string("section", None, "value"),
        Some(cow_str(match expect {
            Value::OverrideByInclude => "branch-override-by-include",
            Value::Base => "base-value",
        })),
        "{}, info: {:?}, debug at {:?}",
        match expect {
            Value::Base => "the base value should not be overridden as the branch does not match",
            Value::OverrideByInclude =>
                "the base value is overridden by an included file because the condition matches",
        },
        message,
        dir.into_path()
    );

    repo.refs
        .transaction()
        .prepare(
            Some(RefEdit {
                name: "HEAD".try_into()?,
                change: Change::Update {
                    log: Default::default(),
                    expected: PreviousValue::Any,
                    new: Target::Symbolic(branch_name),
                },
                deref: false,
            }),
            git_repository::lock::acquire::Fail::Immediately,
        )?
        .commit(repo.committer().to_ref())?;

    let dir = assure_git_agrees(expect, dir)?;
    Ok(GitEnv { repo, dir })
}

fn assure_git_agrees(expected: Value, dir: tempfile::TempDir) -> crate::Result<tempfile::TempDir> {
    let git_dir = dir.path();
    let output = std::process::Command::new("git")
        .args(["config", "--get", "section.value"])
        .env("GIT_DIR", git_dir)
        .env("HOME", git_dir)
        .env_remove("GIT_CONFIG_COUNT")
        .env_remove("XDG_CONFIG_HOME")
        .current_dir(git_dir)
        .output()?;

    assert!(
        output.status.success(),
        "{:?}, {:?} for debugging",
        output,
        dir.into_path()
    );
    let git_output: BString = output.stdout.trim_end().into();
    assert_eq!(
        git_output,
        match expected {
            Value::Base => "base-value",
            Value::OverrideByInclude => "branch-override-by-include",
        },
        "git disagrees with git-config, {:?} for debugging",
        dir.into_path()
    );
    Ok(dir)
}
