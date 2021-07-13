pub mod iter {
    use git_ref::packed;

    #[test]
    fn empty() {
        assert_eq!(
            packed::Iter::new(&[]).unwrap().count(),
            0,
            "empty buffers are fine and lead to no line returned"
        )
    }

    #[test]
    fn packed_refs_with_header() {
        let dir = git_testtools::scripted_fixture_repo_read_only("make_packed_ref_repository.sh").unwrap();
        let buf = std::fs::read(dir.join(".git").join("packed-refs")).unwrap();
        let iter = packed::Iter::new(&buf).unwrap();
        assert_eq!(iter.count(), 8, "it finds the right amount of items");
    }

    #[test]
    #[ignore]
    fn packed_refs_without_header() {}

    #[test]
    #[ignore]
    fn broken_ref_doesnt_end_the_iteration() {}
}
