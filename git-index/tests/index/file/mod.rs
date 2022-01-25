mod init {
    use bstr::ByteSlice;
    use std::path::{Path, PathBuf};

    use git_index::{entry, Version};
    use git_testtools::hex_to_id;

    fn loose_file(name: &str) -> git_index::File {
        let path = git_testtools::fixture_path(Path::new("loose_index").join(name).with_extension("git-index"));
        git_index::File::at(path, git_index::decode::Options::default()).unwrap()
    }
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
            assert!(entry.flags.is_empty());
            assert_eq!(entry.mode, entry::Mode::FILE);
            assert_eq!(entry.path(&file.state), "a");

            let tree = file.tree().unwrap();
            assert_eq!(tree.num_entries, 1);
            assert_eq!(tree.id, hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"));
            assert!(tree.name.is_empty());
            assert!(tree.children.is_empty());
        }
    }
    #[test]
    fn read_v2_empty() {
        let file = file("v2_empty");
        assert_eq!(file.version(), Version::V2);
        assert_eq!(file.entries().len(), 0);
        let tree = file.tree().unwrap();
        assert_eq!(tree.num_entries, 0);
        assert!(tree.name.is_empty());
        assert!(tree.children.is_empty());
        assert_eq!(tree.id, hex_to_id("4b825dc642cb6eb9a060e54bf8d69288fbee4904"));
    }

    #[test]
    fn read_v2_with_multiple_entries_without_eoie_ext() {
        let file = file("v2_more_files");
        assert_eq!(file.version(), Version::V2);

        assert_eq!(file.entries().len(), 6);
        for (idx, path) in ["a", "b", "c", "d/a", "d/b", "d/c"].iter().enumerate() {
            let e = &file.entries()[idx];
            assert_eq!(e.path(&file), path);
            assert!(e.flags.is_empty());
            assert_eq!(e.mode, entry::Mode::FILE);
            assert_eq!(e.id, hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"))
        }

        let tree = file.tree().unwrap();
        assert_eq!(tree.id, hex_to_id("c9b29c3168d8e677450cc650238b23d9390801fb"));
        assert_eq!(tree.num_entries, 6);
        assert!(tree.name.is_empty());
        assert_eq!(tree.children.len(), 1);

        let tree = &tree.children[0];
        assert_eq!(tree.id, hex_to_id("765b32c65d38f04c4f287abda055818ec0f26912"));
        assert_eq!(tree.num_entries, 3);
        assert_eq!(tree.name.as_bstr(), "d");
    }

    fn find_shared_index_for(index: impl AsRef<Path>) -> PathBuf {
        let mut matches = std::fs::read_dir(index.as_ref().parent().unwrap())
            .unwrap()
            .map(Result::unwrap)
            .filter(|e: &std::fs::DirEntry| e.file_name().into_string().unwrap().starts_with("sharedindex."));
        let res = matches.next().unwrap();
        assert!(matches.next().is_none(), "found more than one shared indices");
        res.path()
    }

    #[test]
    fn read_split_index_without_any_extension() {
        let file = git_index::File::at(
            find_shared_index_for(crate::fixture_path("v2_split_index")),
            git_index::decode::Options::default(),
        )
        .unwrap();
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    fn read_v2_split_index() {
        let file = file("v2_split_index");
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    fn read_v3_extended_flags() {
        let file = loose_file("extended-flags");
        assert_eq!(file.version(), Version::V3);
    }

    #[test]
    fn read_v2_very_long_path() {
        let file = loose_file("very-long-path");
        assert_eq!(file.version(), Version::V2);

        assert_eq!(file.state.entries().len(), 9);
        assert_eq!(
            file.state.entries()[0].path(&file.state),
            std::iter::repeat('a')
                .take(4096)
                .chain(std::iter::once('q'))
                .collect::<String>()
        );
    }

    #[test]
    fn read_reuc_extension() {
        let file = loose_file("REUC");
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    fn read_untr_extension() {
        let file = loose_file("UNTR");
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    fn read_untr_extension_with_oids() {
        let file = loose_file("UNTR-with-oids");
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    fn read_fsmn_v1() {
        let file = loose_file("FSMN");
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    fn read_file_with_conflicts() {
        let file = loose_file("conflicting-file");
        assert_eq!(file.version(), Version::V2);
        assert_eq!(file.entries().len(), 3);
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
        .iter()
        .enumerate()
        {
            let e = &file.entries()[idx];
            assert_eq!(e.path(&file), path);
            assert!(e.flags.is_empty());
            assert_eq!(e.mode, entry::Mode::FILE);
            assert_eq!(e.id, hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"))
        }
    }
}
