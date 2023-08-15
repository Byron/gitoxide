use crate::OutputFormat;
use gix::repository::IndexPersistedOrInMemory;

pub struct Options {
    pub format: OutputFormat,
    pub statistics: bool,
}

pub(crate) mod function {
    use std::{io, path::Path};

    use anyhow::{anyhow, bail};
    use gix::bstr::BStr;
    use gix::prelude::FindExt;

    use crate::repository::PathsOrPatterns;
    use crate::{
        repository::attributes::query::{attributes_cache, Options},
        OutputFormat,
    };

    pub fn query(
        repo: gix::Repository,
        input: PathsOrPatterns,
        mut out: impl io::Write,
        mut err: impl io::Write,
        Options { format, statistics }: Options,
    ) -> anyhow::Result<()> {
        if format != OutputFormat::Human {
            bail!("JSON output isn't implemented yet");
        }

        let (mut cache, index) = attributes_cache(&repo)?;
        let mut matches = cache.attribute_matches();

        match input {
            PathsOrPatterns::Paths(paths) => {
                for path in paths {
                    let is_dir = gix::path::from_bstr(path.as_ref()).metadata().ok().map(|m| m.is_dir());

                    let entry = cache.at_entry(path.as_slice(), is_dir, |oid, buf| repo.objects.find_blob(oid, buf))?;
                    if !entry.matching_attributes(&mut matches) {
                        continue;
                    }
                    print_match(&matches, path.as_ref(), &mut out)?;
                }
            }
            PathsOrPatterns::Patterns(patterns) => {
                let mut pathspec = repo.pathspec(patterns, true, &index)?;
                for (path, _entry) in pathspec
                    .index_entries_with_paths(&index)
                    .ok_or_else(|| anyhow!("Pathspec didn't match a single path in the index"))?
                {
                    let entry = cache.at_entry(path, Some(false), |oid, buf| repo.objects.find_blob(oid, buf))?;
                    if !entry.matching_attributes(&mut matches) {
                        continue;
                    }
                    print_match(&matches, path, &mut out)?;
                }
            }
        }

        if let Some(stats) = statistics.then(|| cache.take_statistics()) {
            out.flush()?;
            writeln!(err, "{stats:#?}").ok();
        }
        Ok(())
    }

    fn print_match(
        matches: &gix::attrs::search::Outcome,
        path: &BStr,
        mut out: impl std::io::Write,
    ) -> std::io::Result<()> {
        for m in matches.iter() {
            writeln!(
                out,
                "{}:{}:{}\t{}\t{}",
                m.location.source.map(Path::to_string_lossy).unwrap_or_default(),
                m.location.sequence_number,
                m.pattern,
                path,
                m.assignment
            )?;
        }
        Ok(())
    }
}

pub(crate) fn attributes_cache(
    repo: &gix::Repository,
) -> anyhow::Result<(gix::worktree::Cache, IndexPersistedOrInMemory)> {
    let index = repo.index_or_load_from_head()?;
    let cache = repo.attributes(
        &index,
        if repo.is_bare() {
            gix::worktree::cache::state::attributes::Source::IdMapping
        } else {
            gix::worktree::cache::state::attributes::Source::WorktreeThenIdMapping
        },
        gix::worktree::cache::state::ignore::Source::IdMapping,
        None,
    )?;
    Ok((cache, index))
}
