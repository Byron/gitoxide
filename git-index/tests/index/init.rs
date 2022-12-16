use git_index::{verify::extensions::no_find, State};
use git_repository as git;
use git_repository::prelude::FindExt;
use git_testtools::scripted_fixture_read_only;

#[test]
fn from_tree() -> crate::Result {
    let fixtures = [
        "make_index/v2.sh",
        "make_index/v2_more_files.sh",
        "make_index/v2_all_file_kinds.sh",
        "make_index/v4_more_files_IEOT.sh",
    ];

    for fixture in fixtures {
        let repo_dir = scripted_fixture_read_only(fixture)?;
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
        "entry count mismatch in {:?}",
        fixture
    );

    for (a, e) in actual.entries().iter().zip(expected.entries()) {
        assert_eq!(a.id, e.id, "entry id mismatch in {:?}", fixture);
        assert_eq!(a.flags, e.flags, "entry flags mismatch in {:?}", fixture);
        assert_eq!(a.mode, e.mode, "entry mode mismatch in {:?}", fixture);
        assert_eq!(a.path(actual), e.path(expected), "entry path mismatch in {:?}", fixture);
    }
}
