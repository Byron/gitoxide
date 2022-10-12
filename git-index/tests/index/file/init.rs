mod from_state {
    #[test]
    fn writes_data_to_disk_and_is_a_valid_index() -> git_testtools::Result {
        let fixtures = [
            "v2.sh",
            "v2_empty.sh",
            "v2_more_files.sh",
            "v2_all_file_kinds.sh",
            "v4_more_files_IEOT.sh",
        ];

        for fixture in fixtures {
            let tmp = git_testtools::tempfile::TempDir::new()?;
            let index_path = tmp.path().join(fixture);

            let fixture = format!("make_index/{}", fixture);
            let repo_dir = git_testtools::scripted_fixture_repo_read_only(&fixture)?;
            let index = git_index::File::at(repo_dir.join(".git").join("index"), Default::default())?;

            assert!(!index_path.exists());
            let mut index = git_index::File::from_state(index.state, index_path.clone());
            assert!(index.checksum.is_null());
            assert_eq!(index.path, index_path);

            index.write(git_index::write::Options {
                hash_kind: git_hash::Kind::Sha1,
                ..Default::default()
            })?;
            assert!(!index.checksum.is_null(), "checksum is adjusted after writing");
            assert!(index.path.is_file());
            // TODO: make it easier to test on loose indices.
            assert_eq!(
                index.state.version(),
                git_index::Version::V2,
                "V2 is enough as we don't have extended attributes"
            );

            index.verify_integrity()?;
        }
        Ok(())
    }
}
