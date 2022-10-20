use crate::fixture_index_path;
use git_index::decode;

// These functions don't actually test anything, they just exist to experiment
// and discover a little bit about how sparse indexes work internally.
// Will be deleted eventually

#[test]
#[ignore]
fn sparse_index() {
    let path = fixture_index_path("v3_sparse_index");

    let expected = git_index::File::at(&path, git_hash::Kind::Sha1, decode::Options::default()).unwrap();

    println!("Version {:?}", expected.version());
    println!("Sparse: {}", expected.is_sparse());
    println!("Entries:");
    expected.entries().into_iter().for_each(|e| {
        println!(
            "  path: {:?} - mode: {:?} - flags: {:?}",
            e.path(&expected),
            e.mode,
            e.flags,
        )
    });
}

#[test]
#[ignore]
fn sparse_index_non_cone() {
    let path = fixture_index_path("v3_sparse_index_non_cone");

    let expected = git_index::File::at(&path, git_hash::Kind::Sha1, decode::Options::default()).unwrap();

    println!("Version {:?}", expected.version());
    println!("Sparse: {}", expected.is_sparse());
    println!("Entries:");
    expected.entries().into_iter().for_each(|e| {
        println!(
            "  path: {:?} - mode: {:?} - flags: {:?}",
            e.path(&expected),
            e.mode,
            e.flags,
        )
    });
}

#[test]
#[ignore]
fn skip_worktree() {
    let path = fixture_index_path("v3_skip_worktree");

    let expected = git_index::File::at(&path, git_hash::Kind::Sha1, decode::Options::default()).unwrap();

    println!("Is Sparse: {}", expected.is_sparse());
    println!("Version {:?}", expected.version());
    expected.entries().into_iter().for_each(|e| {
        println!(
            "  path: {:?} - mode: {:?} - flags: {:?}",
            e.path(&expected),
            e.mode,
            e.flags,
        )
    });

    // println!("{:#?}", expected.tree().expect("tree extension"));
}
