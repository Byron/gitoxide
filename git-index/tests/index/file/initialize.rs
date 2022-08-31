// use git_index::verify::extensions::no_find;
use git_index::{decode, State};
use git_repository::prelude::FindExt;
use git_testtools::scripted_fixture_repo_read_only;

#[test]
fn tree_to_state() {
    let fixture = "make_index/v2_more_files.sh";
    let repo_dir = scripted_fixture_repo_read_only(fixture).unwrap();
    let repo = git_repository::open(&repo_dir).unwrap();

    let tree_id = repo.head_commit().unwrap().tree_id().unwrap();

    let expected_state = git_index::File::at(repo_dir.join(".git").join("index"), decode::Options::default()).unwrap();
    let actual_state = State::from_tree(&tree_id, |oid, buf| repo.objects.find_tree_iter(oid, buf).ok()).unwrap();

    println!("\n");
    actual_state.entries().iter().for_each(|e| println!("{}", e.id));
    println!("\n");
    expected_state.entries().iter().for_each(|e| println!("{}", e.id));

    compare_states(&actual_state, &expected_state, fixture)
}

fn compare_states(actual: &State, expected: &State, fixture: &str) {
    actual.verify_entries().expect("valid");
    // actual.verify_extensions(false, no_find).expect("valid");

    assert_eq!(actual.version(), expected.version(), "version mismatch in {}", fixture);
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
        actual.path_backing(),
        expected.path_backing(),
        "path_backing mismatch in {}",
        fixture
    );
}
