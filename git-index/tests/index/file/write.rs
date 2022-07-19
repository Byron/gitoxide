#[test]
fn header() {
    let path = crate::fixture_index_path("V2_empty");
    let index = git_index::File::at(&path, git_index::decode::Options::default()).unwrap();
    let data = std::fs::read(&path).unwrap();

    let header = git_index::write::header(git_index::Version::V2, index.entries().len() as i32);

    assert_eq!(header.len(), 12);
    assert_eq!(header, data[..header.len()]);
}
