mod from_state {
    use crate::index::Fixture::*;
    use git_index::Version::{V2, V3};

    #[test]
    fn writes_data_to_disk_and_is_a_valid_index() -> git_testtools::Result {
        let fixtures = [
            (Loose("extended-flags"), V3),
            (Generated("v2.sh"), V2),
            (Generated("v2_empty.sh"), V2),
            (Generated("v2_more_files.sh"), V2),
            (Generated("v2_all_file_kinds.sh"), V2),
            (Generated("v4_more_files_IEOT.sh"), V2),
        ];

        for (fixture, expected_version) in fixtures {
            let tmp = git_testtools::tempfile::TempDir::new()?;
            let index_path = tmp.path().join(fixture.to_name());
            assert!(!index_path.exists());

            let index = git_index::File::at(fixture.to_path(), Default::default())?;
            let mut index = git_index::File::from_state(index.state, index_path.clone());
            assert!(index.checksum.is_null());
            assert_eq!(index.path, index_path);

            index.write(git_index::write::Options {
                hash_kind: git_hash::Kind::Sha1,
                extensions: Default::default(),
            })?;
            assert!(!index.checksum.is_null(), "checksum is adjusted after writing");
            assert!(index.path.is_file());
            assert_eq!(index.state.version(), expected_version,);

            index.verify_integrity()?;
        }
        Ok(())
    }
}
