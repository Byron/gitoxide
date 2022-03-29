use std::borrow::Cow;

use git_object::bstr::ByteSlice;
use git_repository::{odb::FindExt, Repository};
use git_revision::describe;
use git_testtools::hex_to_id;

mod format;

#[test]
fn option_none_if_no_tag_found() {
    let repo = repo();
    let commit = repo.head().unwrap().peel_to_commit_in_place().unwrap();
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf),
        &Default::default(),
    )
    .unwrap();
    assert_eq!(res, None, "cannot find anything if there's no candidate");
}
#[test]
fn typical_usecases() {
    let repo = repo();
    let commit = repo.head().unwrap().peel_to_commit_in_place().unwrap();
    let name = Cow::Borrowed(b"main".as_bstr());
    let res = git_revision::describe(
        &commit.id,
        |_, _| Err(std::io::Error::new(std::io::ErrorKind::Other, "shouldn't be called")),
        &vec![(commit.id, name.clone())].into_iter().collect(),
    )
    .unwrap();

    assert_eq!(
        res,
        Some(describe::Outcome {
            name,
            id: commit.id,
            depth: 0,
        }),
        "this is an exact match"
    );

    let name = Cow::Borrowed(b"at-c5".as_bstr());
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf),
        &vec![
            (hex_to_id("efd9a841189668f1bab5b8ebade9cd0a1b139a37"), name.clone()),
            (
                hex_to_id("9152eeee2328073cf23dcf8e90c949170b711659"),
                b"at-b1c1".as_bstr().into(),
            ),
        ]
        .into_iter()
        .collect(),
    )
    .unwrap();

    assert_eq!(
        res,
        Some(describe::Outcome {
            name,
            id: commit.id,
            depth: 3,
        }),
        "a match to a tag 1 commit away with 2 commits on the other side of the merge/head"
    );
}

fn repo() -> Repository {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_repo_with_branches.sh").unwrap();
    git_repository::open(dir).unwrap()
}
