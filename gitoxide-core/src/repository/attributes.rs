use std::io;

use anyhow::bail;
use gix::prelude::FindExt;

use crate::OutputFormat;

pub mod query {
    use crate::OutputFormat;

    pub struct Options {
        pub format: OutputFormat,
        pub statistics: bool,
    }
}

pub fn query(
    repo: gix::Repository,
    pathspecs: impl Iterator<Item = gix::path::Spec>,
    mut out: impl io::Write,
    mut err: impl io::Write,
    query::Options { format, statistics }: query::Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("JSON output isn't implemented yet");
    }

    let index = repo.index()?;
    let mut cache = repo.attributes(
        &index,
        gix::worktree::cache::state::attributes::Source::WorktreeThenIdMapping,
        gix::worktree::cache::state::ignore::Source::IdMapping,
        None,
    )?;

    let prefix = repo.prefix().expect("worktree - we have an index by now")?;
    let mut matches = cache.attribute_matches();

    for mut spec in pathspecs {
        for path in spec.apply_prefix(&prefix).items() {
            let is_dir = gix::path::from_bstr(path).metadata().ok().map(|m| m.is_dir());
            let entry = cache.at_entry(path, is_dir, |oid, buf| repo.objects.find_blob(oid, buf))?;

            if !entry.matching_attributes(&mut matches) {
                continue;
            }
            for m in matches.iter() {
                writeln!(
                    out,
                    "{}:{}:{}\t{}\t{}",
                    m.location.source.map(|p| p.to_string_lossy()).unwrap_or_default(),
                    m.location.sequence_number,
                    m.pattern,
                    path,
                    m.assignment
                )?;
            }
        }
    }

    if let Some(stats) = statistics.then(|| cache.take_statistics()) {
        out.flush()?;
        writeln!(err, "{:#?}", stats).ok();
    }
    Ok(())
}
