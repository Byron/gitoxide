use anyhow::{Context, Result};
use git_repository as git;
use std::path::PathBuf;

pub fn describe(
    repo: impl Into<PathBuf>,
    rev_spec: Option<&str>,
    mut out: impl std::io::Write,
    describe::Options {
        all_tags,
        all_refs,
        first_parent,
        always,
        long_format,
    }: describe::Options,
) -> Result<()> {
    let repo = git::open(repo)?.apply_environment();
    let commit = match rev_spec {
        Some(spec) => repo.rev_parse(spec)?.object()?.try_into_commit()?,
        None => repo.head_commit()?,
    };
    use git::commit::describe::SelectRef::*;
    let select_ref = if all_refs {
        AllRefs
    } else if all_tags {
        AllTags
    } else {
        Default::default()
    };
    let mut describe_id = commit
        .describe()
        .names(select_ref)
        .traverse_first_parent(first_parent)
        .id_as_fallback(always)
        .try_format()?
        .with_context(|| format!("Did not find a single candidate ref for naming id '{}'", commit.id))?;

    describe_id.long(long_format);

    writeln!(out, "{}", describe_id)?;
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
    }
}
