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

    let mut buffer = Vec::new();

    let head_id = reference.peel_to_id_in_place(&store, &odb).unwrap();
    let (head, _) = odb.find_commit(&head_id, &mut buffer).unwrap();

    let mut buffer = Vec::new();
    let head_tree_iter = odb
        .find(&head.tree(), &mut buffer)
        .unwrap()
        .0
        .try_into_tree_iter()
        .unwrap();

    let mut traverse = gix_traverse::commit::Simple::new(Some(head_id), &odb);

    traverse.next();

    let iter = traverse.commit_iter();
    let parent_ids = iter.parent_ids().collect::<Vec<_>>();

    let last_parent_id = parent_ids.last().unwrap();

    let mut buffer = Vec::new();

    let (last_parent, _) = odb.find_commit(&last_parent_id, &mut buffer).unwrap();

    let mut buffer = Vec::new();
    let last_parent_tree_iter = odb
        .find(&last_parent.tree(), &mut buffer)
        .unwrap()
        .0
        .try_into_tree_iter()
        .unwrap();

    let mut recorder = gix_diff::tree::Recorder::default();
    let _result = gix_diff::tree::Changes::from(head_tree_iter)
        .needed_to_obtain(
            last_parent_tree_iter,
            gix_diff::tree::State::default(),
            odb,
            &mut recorder,
        )
        .unwrap();
}

fn odb_at(name: &str) -> gix_odb::Handle {
    gix_odb::at(fixture_path().join(name).join(".git/objects")).unwrap()
}

fn fixture_path() -> PathBuf {
    gix_testtools::scripted_fixture_read_only("make_blame_repo.sh").unwrap()
}
