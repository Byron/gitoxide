use git_index::State;
use git_repository as git;
use git_repository::prelude::FindExt;
use git_testtools::scripted_fixture_repo_read_only;

#[test]
fn tree_to_state() -> crate::Result {
    let fixtures = [
        "make_index/v2.sh",
        "make_index/v2_more_files.sh",
        // "make_index/V2_split_index.sh",
        "make_index/v4_more_files_IEOT.sh",
    ];

    for fixture in fixtures {
        let repo_dir = scripted_fixture_repo_read_only(fixture)?;
        let repo = git::open(&repo_dir)?;

        let tree_id = repo.head_commit()?.tree_id()?;

        let expected_state = repo.index()?;
        let actual_state = State::from_tree(&tree_id, |oid, buf| repo.objects.find_tree_iter(oid, buf).ok())?;

        println!("{}\n", fixture);
        actual_state
            .entries()
            .iter()
            .for_each(|e| println!("{}\t{}", e.id, e.path(&actual_state)));
        println!("");
        expected_state
            .entries()
            .iter()
            .for_each(|e| println!("{}\t{}", e.id, e.path(&expected_state)));
        println!("");

        compare_states(&actual_state, &expected_state, fixture)
    }
    Ok(())
}

fn compare_states(actual: &State, expected: &State, fixture: &str) {
    actual.verify_entries().expect("valid");
    // actual.verify_extensions(false, no_find).expect("valid");

    // assert_eq!(actual.version(), expected.version(), "version mismatch in {}", fixture);
    // assert_eq!(
    //     actual.tree(),
    //     options
    //         .extensions
    //         .should_write(extension::tree::SIGNATURE)
    //         .and_then(|_| expected.tree()),
    //     "tree extension mismatch in {}",
    //     fixture
    // );
    assert_eq!(
        actual.entries().len(),
        expected.entries().len(),
        "entry count mismatch in {}",
        fixture
    );
    // assert_eq!(actual.entries(), expected.entries(), "entries mismatch in {}", fixture);
    assert_eq!(
        actual.entries().iter().map(|e| e.mode).collect::<Vec<_>>(),
        expected.entries().iter().map(|e| e.mode).collect::<Vec<_>>()
    );
    assert_eq!(
        actual.path_backing(),
        expected.path_backing(),
        "path_backing mismatch in {}",
        fixture
    );
}
