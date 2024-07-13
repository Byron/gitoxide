use std::path::PathBuf;

use gix_odb::pack::FindExt;
use gix_ref::{file::ReferenceExt, store::WriteReflog};

#[test]
fn it_works() {
    // TODO
    // At a high level, what we want to do is the following:
    //
    // - get the commit that belongs to a commit id
    // - walk through parents
    //   - for each parent, do a diff and mark lines that don’t have a suspect (this is the term
    //     used in `libgit2`) yet, but that have been changed in this commit
    //
    // The algorithm in `libgit2` works by going through parents and keeping a linked list of blame
    // suspects. It can be visualized as follows:
    //
    // <---------------------------------------->
    // <---------------><----------------------->
    // <---><----------><----------------------->
    // <---><----------><-------><-----><------->
    // <---><---><-----><-------><-----><------->
    // <---><---><-----><-------><-----><-><-><->

    let worktree = fixture_path();

    let store = gix_ref::file::Store::at(
        worktree.join(".git"),
        gix_ref::store::init::Options {
            write_reflog: WriteReflog::Disable,
            ..Default::default()
        },
    );
    let odb = odb_at("");

    let mut reference = gix_ref::file::Store::find(&store, "HEAD").unwrap();

    let head = reference.peel_to_id_in_place(&store, &odb).unwrap();

    let mut buffer = Vec::new();

    let (commit, _) = odb.find_commit(&head, &mut buffer).unwrap();

    let last_parent = commit.parents().last().unwrap();

    let mut buffer = Vec::new();

    let (last_parent, _) = odb.find_commit(&last_parent, &mut buffer).unwrap();
}

fn odb_at(name: &str) -> gix_odb::Handle {
    gix_odb::at(fixture_path().join(name).join(".git/objects")).unwrap()
}

fn fixture_path() -> PathBuf {
    gix_testtools::scripted_fixture_read_only("make_blame_repo.sh").unwrap()
}
