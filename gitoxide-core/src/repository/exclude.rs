use std::io;

use anyhow::bail;
use gix::prelude::FindExt;

use crate::OutputFormat;

pub mod query {
    use std::ffi::OsString;

    use crate::OutputFormat;

    pub struct Options {
        pub format: OutputFormat,
        pub overrides: Vec<OsString>,
        pub show_ignore_patterns: bool,
        pub statistics: bool,
    }
}

pub fn query(
    repo: gix::Repository,
    pathspecs: impl Iterator<Item = gix::pathspec::Pattern>,
    mut out: impl io::Write,
    mut err: impl io::Write,
    query::Options {
        overrides,
        format,
        show_ignore_patterns,
        statistics,
    }: query::Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("JSON output isn't implemented yet");
    }

    let index = repo.index()?;
    let mut cache = repo.excludes(
        &index,
        Some(gix::ignore::Search::from_overrides(overrides)),
        Default::default(),
    )?;

    // TODO(pathspec): actually use the search to find items. This looks like `gix` capabilities to put it all together.
    let search = gix::pathspec::Search::from_specs(
        pathspecs,
        repo.prefix()?.as_deref(),
        repo.work_dir().unwrap_or_else(|| repo.git_dir()),
    )?;

    for spec in search.into_patterns() {
        let path = spec.path();
        let is_dir = gix::path::from_bstr(path).metadata().ok().map(|m| m.is_dir());
        let entry = cache.at_entry(path, is_dir, |oid, buf| repo.objects.find_blob(oid, buf))?;
        let match_ = entry
            .matching_exclude_pattern()
            .and_then(|m| (show_ignore_patterns || !m.pattern.is_negative()).then_some(m));
        match match_ {
            Some(m) => writeln!(
                out,
                "{}:{}:{}\t{}",
                m.source.map(std::path::Path::to_string_lossy).unwrap_or_default(),
                m.sequence_number,
                m.pattern,
                path
            )?,
            None => writeln!(out, "::\t{path}")?,
        }
    }

    if let Some(stats) = statistics.then(|| cache.take_statistics()) {
        out.flush()?;
        writeln!(err, "{stats:#?}").ok();
    }
    Ok(())
}
