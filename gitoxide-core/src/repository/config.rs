use crate::OutputFormat;
use anyhow::{bail, Result};
use git_repository as git;

pub fn list(
    repo: git::Repository,
    _filters: Vec<String>,
    format: OutputFormat,
    out: impl std::io::Write,
) -> Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human output format is supported at the moment");
    }
    let config = repo.config_snapshot();
    let config = config.plumbing();
    config.write_to(out)?;
    Ok(())
}
