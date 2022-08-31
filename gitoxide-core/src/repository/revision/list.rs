use anyhow::{bail, Context};
use git_repository as git;
use std::ffi::OsString;

use crate::OutputFormat;

pub fn list(
    mut repo: git::Repository,
    spec: OsString,
    mut out: impl std::io::Write,
    format: OutputFormat,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human output is currently supported");
    }
    repo.object_cache_size_if_unset(4 * 1024 * 1024);

    let spec = git::path::os_str_into_bstr(&spec)?;
    let id = repo
        .rev_parse(spec)?
        .single()
        .context("Only single revisions are currently supported")?;
    for commit in id.ancestors().all()? {
        writeln!(out, "{}", commit?.to_hex())?;
    }
    Ok(())
}
