mod repo_with_small_packs {
    use git_odb::Find;

    use crate::odb::{fixture_path, hex_to_id};

    #[test]
    fn all_packed_objects_can_be_found() {
        let store = git_odb::at(fixture_path("repos/small-packs.git/objects")).unwrap();
        let mut buf = Vec::new();
        assert!(
            store
                .try_find(hex_to_id("ecc68100297fff843a7eef8df0d0fb80c1c8bac5"), &mut buf)
                .unwrap()
                .is_some(),
            "object is present and available"
        );
    }
}
