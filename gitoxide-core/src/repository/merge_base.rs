use crate::OutputFormat;
use anyhow::bail;

pub fn merge_base(
    mut repo: gix::Repository,
    first: String,
    others: Vec<String>,
    mut out: impl std::io::Write,
    format: OutputFormat,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only 'human' format is currently supported");
    }
    repo.object_cache_size_if_unset(50 * 1024 * 1024);
    let first_id = repo.rev_parse_single(first.as_str())?;
    let other_ids: Vec<_> = others
        .iter()
        .cloned()
        .map(|other| repo.rev_parse_single(other.as_str()).map(gix::Id::detach))
        .collect::<Result<_, _>>()?;

    let cache = repo.commit_graph_if_enabled()?;
    let mut graph = repo.revision_graph(cache.as_ref());
    let bases = repo.merge_bases_many_with_graph(first_id, &other_ids, &mut graph)?;
    if bases.is_empty() {
        bail!("No base found for {first} and {others}", others = others.join(", "))
    }
    for id in bases {
        writeln!(&mut out, "{id}")?;
    }
    Ok(())
}
