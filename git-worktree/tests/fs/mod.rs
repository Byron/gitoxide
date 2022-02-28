#[test]
fn from_probing_cwd() {
    let dir = tempfile::tempdir().unwrap();
    let _ctx = git_worktree::fs::Context::probe(dir.path());
    let entries: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path().to_owned())
        .collect();
    assert_eq!(
        entries.len(),
        0,
        "there should be no left-over files after probing, found {:?}",
        entries
    );
}
