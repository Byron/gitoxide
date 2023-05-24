use crate::OutputFormat;
use gix::odb::FindExt;

pub struct Options {
    pub format: OutputFormat,
    pub statistics: bool,
}

pub(crate) mod function {
    use crate::repository::attributes::query::{attributes_cache, Options};
    use crate::OutputFormat;
    use std::io;

    use anyhow::bail;
    use gix::prelude::FindExt;

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
}

pub(crate) enum Index {
    Shared(gix::worktree::Index),
    Owned(gix::index::File),
}

impl std::ops::Deref for Index {
    type Target = gix::index::File;

    fn deref(&self) -> &Self::Target {
        match self {
            Index::Shared(i) => i,
            Index::Owned(i) => i,
        }
    }
}

impl Index {
    pub fn into_owned(self) -> gix::index::File {
        match self {
            Index::Shared(i) => gix::index::File::clone(&i),
            Index::Owned(i) => i,
        }
    }
}

pub(crate) fn index_on_demand(repo: &gix::Repository) -> anyhow::Result<Index> {
    Ok(match repo.index() {
        Ok(index) => Index::Shared(index),
        Err(gix::worktree::open_index::Error::IndexFile(_)) => {
            let tree = repo.head_commit()?.tree_id()?;
            Index::Owned(gix::index::File::from_state(
                gix::index::State::from_tree(&tree, |oid, buf| repo.objects.find_tree_iter(oid, buf).ok())?,
                repo.git_dir().join("index"),
            ))
        }
        Err(err) => return Err(err.into()),
    })
}

pub(crate) fn attributes_cache(repo: &gix::Repository) -> anyhow::Result<gix::worktree::Cache> {
    let index = index_on_demand(repo)?;
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
