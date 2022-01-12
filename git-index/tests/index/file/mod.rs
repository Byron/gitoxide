mod init {
    use git_index::Version;
    use git_testtools::hex_to_id;

    fn file(name: &str) -> git_index::File {
        git_index::File::at(crate::fixture_path(name), git_index::decode::Options::default()).unwrap()
    }
    fn file_opt(name: &str, opts: git_index::decode::Options) -> git_index::File {
        git_index::File::at(crate::fixture_path(name), opts).unwrap()
    }

    #[test]
    fn read_v2_with_single_entry_tree_and_eoie_ext() {
        let file_disallow_threaded_loading = file_opt(
            "v2",
            git_index::decode::Options {
                min_extension_block_in_bytes_for_threading: 100000,
                ..Default::default()
            },
        );
        for file in [file("v2"), file_disallow_threaded_loading] {
            assert_eq!(file.version(), Version::V2);

            assert_eq!(file.entries().len(), 1);

            let entry = &file.entries()[0];
            assert_eq!(entry.id, hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"));
            assert_eq!(entry.path(&file.state), "a");
        }
    }

    #[test]
    fn read_v2_with_multiple_entries_without_eoie_ext() {
        let file = file("v2_more_files");
        assert_eq!(file.version(), Version::V2);

        assert_eq!(file.entries().len(), 6);
        for (idx, path) in ["a", "b", "c", "d/a", "d/b", "d/c"].into_iter().enumerate() {
            let e = &file.entries()[idx];
            assert_eq!(e.path(&file), path);
            assert_eq!(e.id, hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"))
        }
    }

    #[test]
    fn read_split_index_without_any_extension() {
        let file = git_index::File::at(
            crate::fixture_path("v2_split_index")
                .parent()
                .unwrap()
                .join("sharedindex.f1d614440cb10c1b0ba226b0fe73ce5e9d2552dc"),
            git_index::decode::Options::default(),
        )
        .unwrap();
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    #[ignore]
    fn read_v2_split_index() {
        let file = file("v2_split_index");
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    fn read_v4_with_delta_paths_and_ieot_ext() {
        let file = file("v4_more_files_IEOT");
        assert_eq!(file.version(), Version::V4);

        assert_eq!(file.entries().len(), 10);
        for (idx, path) in [
            "a",
            "b",
            "c",
            "d/a",
            "d/b",
            "d/c",
            "d/last/123",
            "d/last/34",
            "d/last/6",
            "x",
        ]
        .into_iter()
        .enumerate()
        {
            let e = &file.entries()[idx];
            assert_eq!(e.path(&file), path);
            assert_eq!(e.id, hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"))
        }
    }
}
