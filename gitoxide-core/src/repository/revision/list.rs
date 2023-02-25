use std::ffi::OsString;

use anyhow::{bail, Context};
use gix::prelude::ObjectIdExt;

use crate::OutputFormat;

pub fn list(
    mut repo: gix::Repository,
    spec: OsString,
    mut out: impl std::io::Write,
    format: OutputFormat,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human output is currently supported");
    }
    repo.object_cache_size_if_unset(4 * 1024 * 1024);

    let spec = gix::path::os_str_into_bstr(&spec)?;
    let id = repo
        .rev_parse_single(spec)
        .context("Only single revisions are currently supported")?;
    let commit_id = id
        .object()?
        .peel_to_kind(gix::object::Kind::Commit)
        .context("Need commitish as starting point")?
        .id
        .attach(&repo);
    for commit in commit_id.ancestors().all()? {
        writeln!(out, "{}", commit?.to_hex())?;
    }
    Ok(())
}
