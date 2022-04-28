use anyhow::{bail, Context};
use std::io;

use crate::OutputFormat;
use git_repository as git;

pub mod query {
    use crate::OutputFormat;
    use git_repository as git;

    pub struct Options {
        pub format: OutputFormat,
        pub pathspecs: Vec<git::path::Spec>,
    }
}

pub fn query(
    repo: git::Repository,
    _out: impl io::Write,
    query::Options { format, pathspecs: _ }: query::Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("JSON output isn't implemented yet");
    }

    repo.worktree()
        .current()
        .with_context(|| "Cannot check excludes without a current worktree")?
        .open_index()?;

    todo!("impl");
}
