use git_index::verify::extensions::no_find;
use git_index::State;
use git_repository as git;
use git_repository::prelude::FindExt;
use git_testtools::scripted_fixture_repo_read_only;

#[test]
fn tree_to_state() -> crate::Result {
    let fixtures = [
        "make_index/v2.sh",
        "make_index/v2_more_files.sh",
        "make_index/v4_more_files_IEOT.sh",
    ];

    for fixture in fixtures {
        let repo_dir = scripted_fixture_repo_read_only(fixture)?;
        let repo = git::open(&repo_dir)?;

        let tree_id = repo.head_commit()?.tree_id()?;

        let expected_state = repo.index()?;
        let actual_state = State::from_tree(&tree_id, |oid, buf| repo.objects.find_tree_iter(oid, buf).ok())?;

        compare_states(&actual_state, &expected_state, fixture)
    }
    Ok(())
}

fn compare_states(actual: &State, expected: &State, fixture: &str) {
    actual.verify_entries().expect("valid");
    actual.verify_extensions(false, no_find).expect("valid");

    assert_eq!(
        actual.entries().len(),
        expected.entries().len(),
        "entry count mismatch in {}",
        fixture
    );

    assert_eq!(
        actual
            .entries()
            .iter()
            .map(|e| (e.id, e.flags, e.mode))
            .collect::<Vec<_>>(),
        expected
            .entries()
            .iter()
            .map(|e| (e.id, e.flags, e.mode))
            .collect::<Vec<_>>()
    );

    // TODO: check if path_backing needs to be sorted like entries are
    // assert_eq!(
    //     actual.path_backing(),
    //     expected.path_backing(),
    //     "path_backing mismatch in {}",
    //     fixture
    // );
}
