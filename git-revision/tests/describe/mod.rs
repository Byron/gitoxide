use git_object::bstr::ByteSlice;
use git_repository::odb::FindExt;
use git_revision::describe;
use git_testtools::hex_to_id;
use std::borrow::Cow;

mod outcome_format {
    use git_object::bstr::ByteSlice;
    use git_revision::describe;
    use git_testtools::hex_to_id;
    use std::borrow::Cow;

    #[test]
    fn exact_match_with_dirty_and_long() {
        let mut outcome = describe::Outcome {
            name: Cow::Borrowed(b"main".as_bstr()),
            id: hex_to_id("b920bbb055e1efb9080592a409d3975738b6efb3"),
            hex_len: 7,
            depth: 0,
            dirty_suffix: None,
            long: false,
        };
        assert!(outcome.is_exact_match());
        assert_eq!(outcome.to_string(), "main");
        assert_eq!(outcome.long().to_string(), "main-0-gb920bbb");

        outcome.dirty_suffix = Some("dirty".into());
        assert_eq!(outcome.short().to_string(), "main-dirty");
        assert_eq!(outcome.long().to_string(), "main-0-gb920bbb-dirty");

        outcome.dirty_suffix = None;
        outcome.depth = 42;
        assert!(!outcome.is_exact_match());
        assert_eq!(outcome.short().to_string(), "main-42-gb920bbb");

        outcome.dirty_suffix = Some("dirty".into());
        assert_eq!(outcome.to_string(), "main-42-gb920bbb-dirty");
        assert_eq!(outcome.long().to_string(), "main-42-gb920bbb-dirty");
    }
}

#[test]
#[ignore]
fn typical_usecases() {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_repo_with_branches.sh").unwrap();
    let repo = git_repository::open(dir).unwrap();

    let commit = repo.head().unwrap().peel_to_commit_in_place().unwrap();
    let name = Cow::Borrowed(b"main".as_bstr());
    let hex_len = 7;
    let res = git_revision::describe(
        &commit.id,
        |_, _| Err(std::io::Error::new(std::io::ErrorKind::Other, "shouldn't be called")),
        hex_len,
        &vec![(commit.id, name.clone())].into_iter().collect(),
    )
    .unwrap();

    assert_eq!(
        res,
        Some(describe::Outcome {
            name,
            id: commit.id,
            depth: 0,
            hex_len,
            long: false,
            dirty_suffix: None
        }),
        "this is an exact match"
    );

    let name = Cow::Borrowed(b"at-c5".as_bstr());
    let res = git_revision::describe(
        &commit.id,
        |id, buf| repo.objects.find_commit_iter(id, buf),
        hex_len,
        &vec![(hex_to_id("efd9a841189668f1bab5b8ebade9cd0a1b139a37"), name.clone())]
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
            hex_len,
            long: false,
            dirty_suffix: None
        }),
        "a match to a tag 1 commit away with 2 commits on the other side of the merge/head"
    );
}
