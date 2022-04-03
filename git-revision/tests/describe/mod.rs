use std::borrow::Cow;

use git_object::bstr::ByteSlice;
use git_repository::{odb::FindExt, Repository};
use git_revision::describe;
use git_testtools::hex_to_id;

mod format;

#[test]
fn option_none_if_no_tag_found() {
    let repo = repo();
    let commit = repo.head_commit().unwrap();
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf),
        Default::default(),
    )
    .unwrap();
    assert!(res.is_none(), "cannot find anything if there's no candidate");
}

#[test]
#[ignore]
fn not_enough_candidates() {
    let repo = repo();
    let commit = repo.head_commit().unwrap();

    let name = Cow::Borrowed(b"at-c5".as_bstr());
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf),
        describe::Options {
            name_by_oid: vec![
                (hex_to_id("efd9a841189668f1bab5b8ebade9cd0a1b139a37"), name.clone()),
                (
                    hex_to_id("9152eeee2328073cf23dcf8e90c949170b711659"),
                    b"at-b1c1".as_bstr().into(),
                ),
            ]
            .into_iter()
            .collect(),
            max_candidates: 1,
        },
    )
    .unwrap()
    .expect("candidate found");

    assert_eq!(res.name, name, "it finds the youngest/most-recent name");
    assert_eq!(res.id, commit.id);
    assert_eq!(
        res.depth, 3,
        "it calculates the final number of commits even though it aborted early"
    );
}

#[test]
fn typical_usecases() {
    let repo = repo();
    let commit = repo.head_commit().unwrap();
    let name = Cow::Borrowed(b"main".as_bstr());
    let res = git_revision::describe(
        &commit.id,
        |_, _| Err(std::io::Error::new(std::io::ErrorKind::Other, "shouldn't be called")),
        describe::Options {
            name_by_oid: vec![(commit.id, name.clone())].into_iter().collect(),
            ..Default::default()
        },
    )
    .unwrap()
    .expect("found a candidate");

    assert_eq!(res.name, name, "this is an exact match");
    assert_eq!(res.id, commit.id);
    assert_eq!(res.depth, 0);

    let name = Cow::Borrowed(b"at-c5".as_bstr());
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf),
        describe::Options {
            name_by_oid: vec![
                (hex_to_id("efd9a841189668f1bab5b8ebade9cd0a1b139a37"), name.clone()),
                (
                    hex_to_id("9152eeee2328073cf23dcf8e90c949170b711659"),
                    b"at-b1c1".as_bstr().into(),
                ),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        },
    )
    .unwrap()
    .expect("found a candidate");

    assert_eq!(
        res.name, name,
        "a match to a tag 1 commit away with 2 commits on the other side of the merge/head"
    );
    assert_eq!(res.id, commit.id);
    assert_eq!(res.depth, 3);
}

fn repo() -> Repository {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_repo_with_branches.sh").unwrap();
    git_repository::open(dir).unwrap()
}
