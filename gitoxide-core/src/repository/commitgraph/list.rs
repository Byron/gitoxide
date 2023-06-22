pub(crate) mod function {
    use std::{borrow::Cow, ffi::OsString};

    use anyhow::{bail, Context};
    use gix::{prelude::ObjectIdExt, traverse::commit::Sorting};

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
        let graph = repo
            .commit_graph()
            .context("a commitgraph is required, but none was found")?;
        repo.object_cache_size_if_unset(4 * 1024 * 1024);

        let spec = gix::path::os_str_into_bstr(&spec)?;
        let id = repo
            .rev_parse_single(spec)
            .context("Only single revisions are currently supported")?;
        let commits = id
            .object()?
            .peel_to_kind(gix::object::Kind::Commit)
            .context("Need commitish as starting point")?
            .id()
            .ancestors()
            .sorting(Sorting::ByCommitTimeNewestFirst)
            .all()?;
        for commit in commits {
            let commit = commit?;
            writeln!(
                out,
                "{} {} {} {}",
                commit.id().shorten_or_id(),
                commit.commit_time.expect("traversal with date"),
                commit.parent_ids.len(),
                graph.commit_by_id(commit.id).map_or_else(
                    || Cow::Borrowed("<NOT IN GRAPH-CACHE>"),
                    |c| Cow::Owned(format!(
                        "{} {}",
                        c.root_tree_id().to_owned().attach(&repo).shorten_or_id(),
                        c.generation()
                    ))
                )
            )?;
        }
        Ok(())
    }
}
