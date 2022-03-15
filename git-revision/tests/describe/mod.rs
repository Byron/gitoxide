use git_object::bstr::ByteSlice;
use std::borrow::Cow;

#[test]
#[ignore]
fn it_uses_the_ref_the_described_commit_is_on() {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_repo_with_branches.sh").unwrap();
    let repo = git_repository::open(dir).unwrap();

    let commit = repo.head().unwrap().peel_to_commit_in_place().unwrap();
    git_revision::describe(
        &commit.id,
        &vec![(commit.id, Cow::Borrowed(b"main".as_bstr()))]
            .into_iter()
            .collect(),
    );
}
