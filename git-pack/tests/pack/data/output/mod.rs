use std::{path::PathBuf, sync::Arc};

use git_odb::linked;
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
        48,
        "The size of the structure shouldn't change unexpectedly"
    )
}

enum DbKind {
    DeterministicGeneratedContent,
}

fn db(kind: DbKind) -> crate::Result<Arc<linked::Store>> {
    use DbKind::*;
    let path: PathBuf = match kind {
        DeterministicGeneratedContent => git_testtools::scripted_fixture_repo_read_only("make_pack_gen_repo.sh")?
            .join(".git")
            .join("objects"),
    };
    linked::Store::at(path).map_err(Into::into).map(Into::into)
}

mod count_and_entries;
mod in_order_iter;
