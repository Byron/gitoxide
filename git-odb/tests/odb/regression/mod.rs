mod repo_with_small_packs {
    use crate::odb::{fixture_path, hex_to_id};
    use git_odb::pack;

    #[test]
    fn all_packed_objects_can_be_found() {
        let store = git_odb::linked::Store::at(fixture_path("repos/small-packs.git/objects")).unwrap();
        assert_eq!(store.dbs.len(), 1, "a simple repo");
        let db = &store.dbs[0];
        assert_eq!(db.bundles.len(), 2, "small packs");
        let mut buf = Vec::new();
        assert!(
            db.try_find(
                hex_to_id("ecc68100297fff843a7eef8df0d0fb80c1c8bac5"),
                &mut buf,
                &mut pack::cache::Never
            )
            .unwrap()
            .is_some(),
            "object is present and available"
        );
    }
}
