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
        pathspecs: impl Iterator<Item = gix::path::Spec>,
        mut out: impl io::Write,
        mut err: impl io::Write,
        Options { format, statistics }: Options,
    ) -> anyhow::Result<()> {
        if format != OutputFormat::Human {
            bail!("JSON output isn't implemented yet");
        }

        let mut cache = attributes_cache(&repo)?;
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
                        m.location.source.map(Path::to_string_lossy).unwrap_or_default(),
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
