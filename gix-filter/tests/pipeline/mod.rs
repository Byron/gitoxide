use gix_attributes::glob::pattern::Case;

mod convert_to_git;
mod convert_to_worktree;

fn attribute_cache(name: &str) -> gix_testtools::Result<gix_worktree::Cache> {
    let dir = gix_testtools::scripted_fixture_read_only("pipeline_repos.sh")?.join(name);
    Ok(gix_worktree::Cache::new(
        dir,
        gix_worktree::cache::State::for_add(
            gix_worktree::cache::state::Attributes::new(
                Default::default(),
                None,
                gix_worktree::cache::state::attributes::Source::WorktreeThenIdMapping,
                Default::default(),
            ),
            gix_worktree::cache::state::Ignore::new(
                Default::default(),
                Default::default(),
                None,
                gix_worktree::cache::state::ignore::Source::WorktreeThenIdMappingIfNotSkipped,
            ),
        ),
        Case::Sensitive,
        Vec::new(),
        Default::default(),
    ))
}

fn pipeline(
    name: &str,
    init: impl FnOnce() -> (
        Vec<gix_filter::Driver>,
        Vec<&'static encoding_rs::Encoding>,
        gix_filter::pipeline::CrlfRoundTripCheck,
    ),
) -> gix_testtools::Result<(gix_worktree::Cache, gix_filter::Pipeline)> {
    let cache = attribute_cache(name)?;
    let (drivers, encodings_with_roundtrip, crlf_roundtrip) = init();
    let pipe = gix_filter::Pipeline::new(
        drivers,
        cache.attributes_collection(),
        Default::default(),
        encodings_with_roundtrip,
        crlf_roundtrip,
        gix_hash::Kind::Sha1,
    );
    Ok((cache, pipe))
}
