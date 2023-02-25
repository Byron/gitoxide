use std::io;

use anyhow::{bail, Context};
use gix::prelude::FindExt;

use crate::OutputFormat;

pub mod query {
    use std::ffi::OsString;

    use crate::OutputFormat;

    pub struct Options {
        pub format: OutputFormat,
        pub overrides: Vec<OsString>,
        pub show_ignore_patterns: bool,
    }
}

pub fn query(
    repo: gix::Repository,
    pathspecs: impl Iterator<Item = gix::path::Spec>,
    mut out: impl io::Write,
    query::Options {
        overrides,
        format,
        show_ignore_patterns,
    }: query::Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("JSON output isn't implemented yet");
    }

    let worktree = repo
        .worktree()
        .with_context(|| "Cannot check excludes without a current worktree")?;
    let index = worktree.index()?;
    let mut cache = worktree.excludes(
        &index,
        Some(gix::attrs::MatchGroup::<gix::attrs::Ignore>::from_overrides(overrides)),
    )?;

    let prefix = repo.prefix().expect("worktree - we have an index by now")?;

    for mut spec in pathspecs {
        for path in spec.apply_prefix(&prefix).items() {
            // TODO: what about paths that end in /? Pathspec might handle it, it's definitely something git considers
            //       even if the directory doesn't exist. Seems to work as long as these are kept in the spec.
            let is_dir = gix::path::from_bstr(path).metadata().ok().map(|m| m.is_dir());
            let entry = cache.at_entry(path, is_dir, |oid, buf| repo.objects.find_blob(oid, buf))?;
            let match_ = entry
                .matching_exclude_pattern()
                .and_then(|m| (show_ignore_patterns || !m.pattern.is_negative()).then_some(m));
            match match_ {
                Some(m) => writeln!(
                    out,
                    "{}:{}:{}\t{}",
                    m.source.map(|p| p.to_string_lossy()).unwrap_or_default(),
                    m.sequence_number,
                    m.pattern,
                    path
                )?,
                None => writeln!(out, "::\t{}", path)?,
            }
        }
    }
    Ok(())
}
