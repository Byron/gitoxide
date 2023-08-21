use std::{io::Write, process::Stdio};

use anyhow::{anyhow, bail, Context, Result};

/// Note that this is a quick implementation of commit signature verification that ignores a lot of what
/// git does and can do, while focussing on the gist of it.
/// For this to go into `gix`, one will have to implement many more options and various validation programs.
pub fn verify(repo: gix::Repository, rev_spec: Option<&str>) -> Result<()> {
    let rev_spec = rev_spec.unwrap_or("HEAD");
    let commit = repo
        .rev_parse_single(format!("{rev_spec}^{{commit}}").as_str())?
        .object()?
        .into_commit();
    let (signature, signed_data) = commit
        .signature()
        .context("Could not parse commit to obtain signature")?
        .ok_or_else(|| anyhow!("Commit at {rev_spec} is not signed"))?;

    let mut signature_storage = tempfile::NamedTempFile::new()?;
    signature_storage.write_all(signature.as_ref())?;
    let signed_storage = signature_storage.into_temp_path();

    let mut cmd = std::process::Command::new("gpg");
    cmd.args(["--keyid-format=long", "--status-fd=1", "--verify"])
        .arg(&signed_storage)
        .arg("-")
        .stdin(Stdio::piped());
    gix::trace::debug!("About to execute {cmd:?}");
    let mut child = cmd.spawn()?;
    child
        .stdin
        .take()
        .expect("configured")
        .write_all(signed_data.to_bstring().as_ref())?;

    if !child.wait()?.success() {
        bail!("Command {cmd:?} failed");
    }
    Ok(())
}

pub fn describe(
    mut repo: gix::Repository,
    rev_spec: Option<&str>,
    mut out: impl std::io::Write,
    mut err: impl std::io::Write,
    describe::Options {
        all_tags,
        all_refs,
        first_parent,
        always,
        statistics,
        max_candidates,
        long_format,
    }: describe::Options,
) -> Result<()> {
    repo.object_cache_size_if_unset(4 * 1024 * 1024);
    let commit = match rev_spec {
        Some(spec) => repo.rev_parse_single(spec)?.object()?.try_into_commit()?,
        None => repo.head_commit()?,
    };
    use gix::commit::describe::SelectRef::*;
    let select_ref = if all_refs {
        AllRefs
    } else if all_tags {
        AllTags
    } else {
        Default::default()
    };
    let resolution = commit
        .describe()
        .names(select_ref)
        .traverse_first_parent(first_parent)
        .id_as_fallback(always)
        .max_candidates(max_candidates)
        .try_resolve()?
        .with_context(|| format!("Did not find a single candidate ref for naming id '{}'", commit.id))?;

    if statistics {
        writeln!(err, "traversed {} commits", resolution.outcome.commits_seen)?;
    }

    let mut describe_id = resolution.format()?;
    describe_id.long(long_format);

    writeln!(out, "{describe_id}")?;
    Ok(())
}

pub mod describe {
    #[derive(Debug, Clone)]
    pub struct Options {
        pub all_tags: bool,
        pub all_refs: bool,
        pub first_parent: bool,
        pub always: bool,
        pub long_format: bool,
        pub statistics: bool,
        pub max_candidates: usize,
    }
}
