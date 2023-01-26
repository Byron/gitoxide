use std::{path::PathBuf, sync::Arc};

use git_pack::data::output;

#[test]
fn size_of_entry() {
    assert_eq!(
        std::mem::size_of::<output::Entry>(),
        80,
        "The size of the structure shouldn't change unexpectedly"
    )
}

#[test]
fn size_of_count() {
    assert_eq!(
        std::mem::size_of::<output::Count>(),
        56,
        "The size of the structure shouldn't change unexpectedly"
    )
}

enum DbKind {
    DeterministicGeneratedContent,
    DeterministicGeneratedContentMultiIndex,
}

fn db(kind: DbKind) -> crate::Result<git_odb::HandleArc> {
    use DbKind::*;
    let name = match kind {
        DeterministicGeneratedContent => "make_pack_gen_repo.sh",
        DeterministicGeneratedContentMultiIndex => "make_pack_gen_repo_multi_index.sh",
    };
    let path: PathBuf = git_testtools::scripted_fixture_read_only(name)?
        .join(".git")
        .join("objects");
    git_odb::Store::at_opts(path, Vec::new(), git_odb::store::init::Options::default())
        .map_err(Into::into)
        .map(|store| {
            let mut cache = Arc::new(store).to_cache_arc();
            cache.prevent_pack_unload();
            cache
        })
}

mod count_and_entries;
