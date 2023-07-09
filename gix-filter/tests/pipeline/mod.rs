use bstr::ByteSlice;
use gix_attributes::glob::pattern::Case;
use gix_filter::eol;

mod convert_to_git;
mod convert_to_worktree;

#[test]
fn default() -> crate::Result {
    let mut filters = gix_filter::Pipeline::default();
    let out = filters.convert_to_worktree(b"hi", "file".into(), |_, _| {}, gix_filter::driver::apply::Delay::Allow)?;
    assert_eq!(
        out.as_bytes().expect("unchanged").as_bstr(),
        "hi",
        "default-pipelines can be used like normal, they have not effect"
    );
    Ok(())
}

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
        eol::Configuration,
    ),
) -> gix_testtools::Result<(gix_worktree::Cache, gix_filter::Pipeline)> {
    let cache = attribute_cache(name)?;
    let (drivers, encodings_with_roundtrip_check, crlf_roundtrip_check, eol_config) = init();
    let pipe = gix_filter::Pipeline::new(
        cache.attributes_collection(),
        gix_filter::pipeline::Options {
            drivers,
            eol_config,
            encodings_with_roundtrip_check,
            crlf_roundtrip_check,
            object_hash: gix_hash::Kind::Sha1,
        },
    );
    Ok((cache, pipe))
}
