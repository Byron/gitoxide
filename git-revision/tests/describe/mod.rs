use std::borrow::Cow;

use git_object::bstr::ByteSlice;
use git_repository::{
    odb::{Find, FindExt},
    Repository,
};
use git_revision::describe;
use git_testtools::hex_to_id;

mod format;

#[test]
fn option_none_if_no_tag_found() -> crate::Result {
    let repo = repo();
    let commit = repo.head_commit()?;
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf).map(Some),
        Default::default(),
    )?;
    assert!(res.is_none(), "cannot find anything if there's no candidate");
    Ok(())
}

#[test]
fn fallback_if_configured_in_options_but_no_candidate_or_names() -> crate::Result {
    let repo = repo();
    let commit = repo.head_commit()?;
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf).map(Some),
        describe::Options {
            fallback_to_oid: true,
            ..Default::default()
        },
    )?
    .expect("fallback activated");
    assert!(res.name.is_none(), "no name can be found");
    assert_eq!(res.depth, 0, "just a default, not relevant as there is no name");
    assert_eq!(
        res.commits_seen, 0,
        "a traversal is isn't performed as name map is empty, and that's the whole point"
    );
    assert_eq!(res.into_format(7).to_string(), "01ec18a");
    Ok(())
}

#[test]
fn fallback_if_configured_in_options_and_max_candidates_zero() -> crate::Result {
    let repo = repo();
    let commit = repo.head_commit()?;
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf).map(Some),
        describe::Options {
            fallback_to_oid: true,
            max_candidates: 0,
            ..Default::default()
        },
    )?
    .expect("fallback activated");
    assert!(res.name.is_none(), "no name can be found");
    assert_eq!(res.depth, 0, "just a default, not relevant as there is no name");
    assert_eq!(res.commits_seen, 0, "we don't do any traversal");
    assert_eq!(res.into_format(7).to_string(), "01ec18a");
    Ok(())
}

#[test]
fn not_enough_candidates() -> crate::Result {
    let repo = repo();
    let commit = repo.head_commit()?;

    let name = Cow::Borrowed(b"at-c5".as_bstr());
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf).map(Some),
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
            ..Default::default()
        },
    )?
    .expect("candidate found");

    assert_eq!(res.name, Some(name), "it finds the youngest/most-recent name");
    assert_eq!(res.id, commit.id);
    assert_eq!(res.commits_seen, 6, "it has to traverse commits");
    assert_eq!(
        res.depth, 3,
        "it calculates the final number of commits even though it aborted early"
    );

    Ok(())
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
            max_candidates: 0,
            ..Default::default()
        },
    )
    .unwrap()
    .expect("found a candidate");

    assert_eq!(
        res.name,
        Some(name),
        "this is an exact match, and it's found despite max-candidates being 0 (one lookup is always performed)"
    );
    assert_eq!(res.id, commit.id);
    assert_eq!(res.depth, 0);

    let name = Cow::Borrowed(b"at-c5".as_bstr());
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf).map(Some),
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
        res.name,
        Some(name.clone()),
        "a match to a tag 1 commit away with 2 commits on the other side of the merge/head"
    );
    assert_eq!(res.id, commit.id);
    assert_eq!(res.depth, 3);
    assert_eq!(res.commits_seen, 6);

    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf).map(Some),
        describe::Options {
            name_by_oid: res.name_by_oid,
            first_parent: true,
            ..Default::default()
        },
    )
    .unwrap()
    .expect("found a candidate");

    assert_eq!(res.name, Some(name),);
    assert_eq!(res.id, commit.id);
    assert_eq!(res.depth, 1);

    let shallow_repo = git_repository::open(repo.work_dir().expect("non-bare").join("shallow-clone")).unwrap();

    let res = git_revision::describe(
        &commit.id,
        |id, buf| {
            shallow_repo
                .objects
                .try_find(id, buf)
                .map(|r| r.and_then(|d| d.try_into_commit_iter()))
        },
        describe::Options {
            name_by_oid: res.name_by_oid,
            first_parent: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(res.is_none(), "no candidate found on truncated history");
}

fn repo() -> Repository {
    let dir = git_testtools::scripted_fixture_read_only("make_repo_with_branches.sh").unwrap();
    git_repository::open(dir).unwrap()
}
