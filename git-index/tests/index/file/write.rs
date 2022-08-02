use git_index::write::Options;

#[test]
fn v2_empty() {
    let path = crate::fixture_index_path("V2_empty");
    let index = git_index::File::at(&path, git_index::decode::Options::default()).unwrap();
    let expected = std::fs::read(&path).unwrap();
    let expected_without_hash = &expected[..expected.len() - 20];

    let output = index.write_to(Options::default());

    assert_eq!(output, expected_without_hash);
}

#[test]
fn v2() {
    let path = crate::fixture_index_path("v2");
    let index = git_index::File::at(&path, git_index::decode::Options::default()).unwrap();
    let expected = std::fs::read(&path).unwrap();
    let expected_without_hash = &expected[..expected.len() - 20];

    let output = index.write_to(Options::default());

    assert_eq!(output, expected_without_hash);
}

#[test]
#[ignore]
fn v2_more_files() {
    let path = crate::fixture_index_path("v2_more_files");
    let index = git_index::File::at(&path, git_index::decode::Options::default()).unwrap();
    let expected = std::fs::read(&path).unwrap();
    let expected_without_hash = &expected[..expected.len() - 20];

    let output = index.write_to(Options::default());

    assert_eq!(output, expected_without_hash);
}
