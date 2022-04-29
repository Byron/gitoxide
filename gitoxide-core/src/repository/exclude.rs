use anyhow::{bail, Context};
use std::io;

use crate::OutputFormat;
use git_repository as git;

pub mod query {
    use crate::OutputFormat;
    use git_repository as git;
    use std::ffi::OsString;

    pub struct Options {
        pub format: OutputFormat,
        pub pathspecs: Vec<git::path::Spec>,
        pub overrides: Vec<OsString>,
    }
}

pub fn query(
    repo: git::Repository,
    _out: impl io::Write,
    query::Options {
        overrides,
        format,
        pathspecs: _,
    }: query::Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("JSON output isn't implemented yet");
    }

    let worktree = repo
        .worktree()
        .current()
        .with_context(|| "Cannot check excludes without a current worktree")?;
    let index = worktree.open_index()?;
    worktree.excludes(
        &index.state,
        Some(git::attrs::MatchGroup::<git::attrs::Ignore>::from_overrides(overrides)),
    )?;

    todo!("impl");
}
