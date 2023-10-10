use gix::repository::IndexPersistedOrInMemory;

use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub statistics: bool,
}

pub(crate) mod function {
    use std::borrow::Cow;
    use std::{io, path::Path};

    use anyhow::{anyhow, bail};
    use gix::bstr::BStr;

    use crate::{
        repository::{
            attributes::query::{attributes_cache, Options},
            PathsOrPatterns,
        },
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
                    let is_dir = gix::path::from_bstr(Cow::Borrowed(path.as_ref()))
                        .metadata()
                        .ok()
                        .map(|m| m.is_dir());

                    let entry = cache.at_entry(path.as_slice(), is_dir)?;
                    if !entry.matching_attributes(&mut matches) {
                        continue;
                    }
                    print_match(&matches, path.as_ref(), &mut out)?;
                }
            }
            PathsOrPatterns::Patterns(patterns) => {
                let mut pathspec = repo.pathspec(
                    patterns.iter(),
                    true,
                    &index,
                    gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping
                        .adjust_for_bare(repo.is_bare()),
                )?;
                let mut pathspec_matched_entry = false;
                for (path, _entry) in pathspec
                    .index_entries_with_paths(&index)
                    .ok_or_else(|| anyhow!("Pathspec didn't match a single path in the index"))?
                {
                    pathspec_matched_entry = true;
                    let entry = cache.at_entry(path, Some(false))?;
                    if !entry.matching_attributes(&mut matches) {
                        continue;
                    }
                    print_match(&matches, path, &mut out)?;
                }

                if !pathspec_matched_entry {
                    // TODO(borrowchk): this shouldn't be necessary at all, but `pathspec` stays borrowed mutably for some reason.
                    //                  It's probably due to the strange lifetimes of `index_entries_with_paths()`.
                    let pathspec = repo.pathspec(
                        patterns.iter(),
                        true,
                        &index,
                        gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping
                            .adjust_for_bare(repo.is_bare()),
                    )?;
                    for pattern in pathspec.search().patterns() {
                        let path = pattern.path();
                        let entry = cache.at_entry(
                            path,
                            pattern
                                .signature
                                .contains(gix::pathspec::MagicSignature::MUST_BE_DIR)
                                .into(),
                        )?;
                        if !entry.matching_attributes(&mut matches) {
                            continue;
                        }
                        print_match(&matches, path, &mut out)?;
                    }
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
) -> anyhow::Result<(gix::AttributeStack<'_>, IndexPersistedOrInMemory)> {
    let index = repo.index_or_load_from_head()?;
    let cache = repo.attributes(
        &index,
        if repo.is_bare() {
            gix::worktree::stack::state::attributes::Source::IdMapping
        } else {
            gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping
        },
        gix::worktree::stack::state::ignore::Source::IdMapping,
        None,
    )?;
    Ok((cache, index))
}
