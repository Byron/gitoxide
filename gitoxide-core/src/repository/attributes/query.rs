use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub statistics: bool,
}

pub(crate) mod function {
    use std::{io, path::Path};

    use anyhow::bail;
    use gix::prelude::FindExt;

    use crate::{
        repository::attributes::query::{attributes_cache, Options},
        OutputFormat,
    };

    pub fn query(
        repo: gix::Repository,
        pathspecs: impl Iterator<Item = gix::pathspec::Pattern>,
        mut out: impl io::Write,
        mut err: impl io::Write,
        Options { format, statistics }: Options,
    ) -> anyhow::Result<()> {
        if format != OutputFormat::Human {
            bail!("JSON output isn't implemented yet");
        }

        let mut cache = attributes_cache(&repo)?;
        let mut matches = cache.attribute_matches();
        // TODO(pathspec): The search is just used as a shortcut to normalization, but one day should be used for an actual search.
        let search = gix::pathspec::Search::from_specs(
            pathspecs,
            repo.prefix()?.as_deref(),
            repo.work_dir().unwrap_or_else(|| repo.git_dir()),
        )?;

        for spec in search.into_patterns() {
            let is_dir = gix::path::from_bstr(spec.path()).metadata().ok().map(|m| m.is_dir());
            let entry = cache.at_entry(spec.path(), is_dir, |oid, buf| repo.objects.find_blob(oid, buf))?;

            if !entry.matching_attributes(&mut matches) {
                continue;
            }
            for m in matches.iter() {
                writeln!(
                    out,
                    "{}:{}:{}\t{}\t{}",
                    m.location.source.map(Path::to_string_lossy).unwrap_or_default(),
                    m.location.sequence_number,
                    m.pattern,
                    spec.path(),
                    m.assignment
                )?;
            }
        }

        if let Some(stats) = statistics.then(|| cache.take_statistics()) {
            out.flush()?;
            writeln!(err, "{stats:#?}").ok();
        }
        Ok(())
    }
}

pub(crate) fn attributes_cache(repo: &gix::Repository) -> anyhow::Result<gix::worktree::Cache> {
    let index = repo.index_or_load_from_head()?;
    Ok(repo.attributes(
        &index,
        if repo.is_bare() {
            gix::worktree::cache::state::attributes::Source::IdMapping
        } else {
            gix::worktree::cache::state::attributes::Source::WorktreeThenIdMapping
        },
        gix::worktree::cache::state::ignore::Source::IdMapping,
        None,
    )?)
}
