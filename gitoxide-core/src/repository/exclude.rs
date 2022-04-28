use anyhow::bail;
use std::io;
use std::path::PathBuf;

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
    repository: PathBuf,
    mut out: impl io::Write,
    query::Options { format, pathspecs }: query::Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("JSON output isn't implemented yet");
    }

    let repo = git::open(repository)?.apply_environment();
    todo!("impl");
}
