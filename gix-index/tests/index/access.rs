use crate::index::Fixture;

fn icase_fixture() -> gix_index::File {
    Fixture::Generated("v2_icase_name_clashes").open()
}

mod directory_by_path {
    use crate::index::Fixture;
    use gix_index::DirectoryKind;

    #[test]
    fn normal_entries_are_never_a_directory() {
        for fixture in [
            Fixture::Generated("v2_deeper_tree.sh"),
            Fixture::Generated("v2_more_files.sh"),
        ] {
            let file = fixture.open();
            for entry in file.entries() {
                for ignore_case in [false, true] {
                    assert_eq!(file.directory_kind_by_path_icase(entry.path(&file), ignore_case), None);
                }
            }
        }
    }

    #[test]
    fn inferred() {
        let file = Fixture::Generated("v2_deeper_tree.sh").open();

        let searches = ["d", "d/nested", "sub", "sub/a", "sub/b", "sub/c", "sub/c/d"];
        for search in searches {
            assert_eq!(
                file.directory_kind_by_path_icase(search.into(), false),
                Some(DirectoryKind::Inferred),
                "directories can be inferred if the index contains an entry in them"
            );
        }

        for search in searches.into_iter().map(str::to_ascii_uppercase) {
            assert_eq!(
                file.directory_kind_by_path_icase(search.as_str().into(), true),
                Some(DirectoryKind::Inferred),
                "directories can be inferred if the index contains an entry in them, also in case-insensitive mode"
            );
            assert_eq!(
                file.directory_kind_by_path_icase(search.as_str().into(), false),
                None,
                "nothing can be found in case-sensitive mode"
            );
        }
    }

    #[test]
    fn entries_themselves_are_returned_as_dir_only_if_sparse_or_commits() {
        let file = Fixture::Generated("v2_all_file_kinds.sh").open();
        for (search, ignore_case) in [("sub", false), ("SuB", true)] {
            assert_eq!(
                file.directory_kind_by_path_icase(search.into(), ignore_case),
                Some(DirectoryKind::Submodule),
                "submodules can be found verbatim"
            );
        }

        let file = Fixture::Generated("v3_sparse_index.sh").open();
        for sparse_dir in ["d", "c1/c3"] {
            assert_eq!(
                file.directory_kind_by_path_icase(sparse_dir.into(), false),
                Some(DirectoryKind::SparseDir),
                "sparse directories can be found verbatim"
            );
        }

        for sparse_dir in ["D", "C1/c3", "c1/C3"] {
            assert_eq!(
                file.directory_kind_by_path_icase(sparse_dir.into(), true),
                Some(DirectoryKind::SparseDir),
                "sparse directories can be found verbatim"
            );
        }
    }

    #[test]
    fn icase_handling() {
        let file = Fixture::Generated("v2_icase_name_clashes.sh").open();

        for search in ["d", "D"] {
            assert_eq!(
                file.directory_kind_by_path_icase(search.into(), true),
                Some(DirectoryKind::Inferred),
                "There exists 'd' and 'D/file', and we manage to find the directory"
            );
        }

        for ignore_case in [false, true] {
            for search in ["d/x", "D/X", "D/B", "file", "FILE_X"] {
                assert_eq!(
                    file.directory_kind_by_path_icase(search.into(), ignore_case),
                    None,
                    "even though `D` exists as directory, we are not able to find it, which makes sense as there is no sub-entry"
                );
            }
        }
    }
}

#[test]
fn entry_by_path() {
    let file = icase_fixture();
    for entry in file.entries() {
        let path = entry.path(&file);
        assert_eq!(file.entry_by_path(path), Some(entry));
        assert_eq!(file.entry_by_path_and_stage(path, 0), Some(entry));
    }
}

#[test]
fn entry_by_path_icase() {
    let file = icase_fixture();
    assert_eq!(
        file.entry_by_path("D/b".into()),
        None,
        "the 'b' is uppercase in the index"
    );
    assert_eq!(
        file.entry_by_path_icase("D/b".into(), false),
        None,
        "ignore case off means it's just the same as the non-icase method"
    );
    assert_eq!(
        file.entry_by_path_icase("D/b".into(), true),
        file.entry_by_path("D/B".into()),
        "with case-folding, the index entry can be found"
    );

    assert_eq!(
        file.entry_by_path_icase("file_x".into(), true),
        file.entry_by_path("FILE_x".into()),
        "case-folding can make matches ambiguous, and it's unclear what we get"
    );
}

#[test]
fn prefixed_entries_icase_with_name_clashes() {
    let file = icase_fixture();
    assert_eq!(
        file.prefixed_entries_range("file".into()),
        Some(7..9),
        "case sensitive search yields only two: file_x and file_X"
    );
    assert_eq!(
        file.prefixed_entries_range_icase("file".into(), false),
        Some(7..9),
        "case-sensitivity can be turned off even for icase searches"
    );
    assert_eq!(
        file.prefixed_entries_range_icase("file".into(), true),
        Some(3..9),
        "case sensitive search yields all relevant items, but… it only assures the start and end of the range is correct \
        which is: 3: FILE_X, 4: FILE_x, …[not the right prefix]…, 7: file_X, 8: file_x"
    );

    assert_eq!(
        file.prefixed_entries_range_icase("d/".into(), true),
        Some(1..3),
        "this emulates a directory search (but wouldn't catch git commits or sparse dirs): 1: D/B, 2: D/C"
    );
    assert_eq!(
        file.prefixed_entries_range_icase("d".into(), true),
        Some(1..7),
        "without slash one can get everything that matches: 1: D/B, 2: D/C, …inbetweens… 6: d"
    );
}

#[test]
fn entry_by_path_and_stage() {
    let file = Fixture::Generated("v4_more_files_IEOT").open();
    for entry in file.entries() {
        let path = entry.path(&file);
        assert_eq!(
            file.entry_index_by_path_and_stage(path, 0)
                .map(|idx| &file.entries()[idx]),
            Some(entry)
        );
        assert_eq!(file.entry_by_path_and_stage(path, 0), Some(entry));
    }
}

#[test]
fn entry_by_path_and_stage_icase() {
    let file = icase_fixture();
    assert_eq!(
        file.entry_by_path_and_stage_icase("D/b".into(), 0, true),
        file.entry_by_path_and_stage("D/B".into(), 0),
        "with case-folding, the index entry can be found"
    );
    assert_eq!(
        file.entry_by_path_and_stage_icase("D/b".into(), 0, false),
        None,
        "if case-folding is disabled, it is case-sensitive"
    );

    assert_eq!(
        file.entry_by_path_and_stage_icase("file_x".into(), 0, true),
        file.entry_by_path_and_stage("FILE_x".into(), 0),
        "case-folding can make matches ambiguous, and it's unclear what we get"
    );
}

#[test]
fn entry_by_path_with_conflicting_file() {
    let file = Fixture::Loose("conflicting-file").open();
    for expected_stage in [1 /* common ancestor */, 2 /* ours */, 3 /* theirs */] {
        assert!(
            file.entry_by_path_and_stage("file".into(), expected_stage).is_some(),
            "we have no stage 0 during a conflict, but all other ones. Missed {expected_stage}"
        );
    }

    assert_eq!(
        file.entry_by_path("file".into()).expect("found").stage(),
        2,
        "we always find our stage while in a merge"
    );
}

#[test]
fn prefixed_entries_with_multi_stage_file() {
    let file = Fixture::Loose("conflicting-file").open();

    assert_eq!(
        file.prefixed_entries("fil".into()).expect("present"),
        file.entries(),
        "it's possible to get the entire range"
    );
    assert_eq!(
        file.prefixed_entries("file".into()).expect("present"),
        file.entries(),
        "it's possible to get the entire range even if the same path matches multiple times"
    );
    assert_eq!(
        file.prefixed_entries("".into()).expect("present"),
        file.entries(),
        "empty prefix matches all"
    );
    assert_eq!(file.prefixed_entries_range("".into()), Some(0..3));
    assert_eq!(file.prefixed_entries_range("foo".into()), None);
}

#[test]
fn entry_range() {
    let file = Fixture::Loose("conflicting-file").open();

    assert_eq!(
        file.entry_range("file".into()),
        Some(0..3),
        "three stages, all but stage zero"
    );
    assert_eq!(file.entry_range("foo".into()), None, "does not exist");
}

#[test]
fn remove_entries() {
    let mut file = Fixture::Loose("conflicting-file").open();

    file.remove_entries(|idx, _, _| idx == 0);
    assert_eq!(file.entries().len(), 2);
    file.remove_entries(|idx, _, _| idx == 0);
    assert_eq!(file.entries().len(), 1);
    file.remove_entries(|idx, _, _| idx == 0);
    assert_eq!(file.entries().len(), 0);
    file.remove_entries(|_, _, _| unreachable!("should not be called"));
}

#[test]
fn sort_entries() {
    let mut file = Fixture::Generated("v4_more_files_IEOT").open();
    assert!(file.verify_entries().is_ok());
    let valid_entries = file.entries().len();

    let entry = file.entry(0).clone();
    let new_entry_path = "an initially incorrectly ordered entry".into();
    file.dangerously_push_entry(entry.stat, entry.id, entry.flags, entry.mode, new_entry_path);
    assert!(file.verify_entries().is_err(), "sort order doesn't fit anymore");

    for (idx, entry) in file.entries()[..valid_entries].iter().enumerate() {
        assert_eq!(
            file.entry_index_by_path_and_stage_bounded(entry.path(&file), 0, valid_entries),
            Some(idx),
            "we can still find entries in the correctly sorted region"
        );
    }
    assert_eq!(
        file.entry_by_path_and_stage(new_entry_path, 0),
        None,
        "new entry can't be found due to incorrect order"
    );

    file.sort_entries();
    assert!(file.verify_entries().is_ok(), "sorting of entries restores invariants");

    assert_eq!(
        file.entry_by_path_and_stage(new_entry_path, 0)
            .expect("can be found")
            .path(&file),
        new_entry_path,
        "we can find the correct entry now"
    );

    check_prefix(
        &file,
        "d",
        &["d/a", "d/b", "d/c", "d/last/123", "d/last/34", "d/last/6"],
    );
    check_prefix_icase(
        &file,
        "D",
        &["d/a", "d/b", "d/c", "d/last/123", "d/last/34", "d/last/6"],
    );
    check_prefix(
        &file,
        "d/",
        &["d/a", "d/b", "d/c", "d/last/123", "d/last/34", "d/last/6"],
    );
    check_prefix_icase(
        &file,
        "D/",
        &["d/a", "d/b", "d/c", "d/last/123", "d/last/34", "d/last/6"],
    );
    check_prefix(&file, "d/last", &["d/last/123", "d/last/34", "d/last/6"]);
    check_prefix(&file, "d/last/", &["d/last/123", "d/last/34", "d/last/6"]);
    check_prefix(&file, "d/las", &["d/last/123", "d/last/34", "d/last/6"]);
    check_prefix(&file, "d/last/123", &["d/last/123"]);
    check_prefix(&file, "d/last/34", &["d/last/34"]);
    check_prefix(&file, "d/last/6", &["d/last/6"]);
    check_prefix(&file, "x", &["x"]);
    check_prefix_icase(&file, "X", &["x"]);
    check_prefix(&file, "a", &["a", "an initially incorrectly ordered entry"]);
    check_prefix_icase(&file, "A", &["a", "an initially incorrectly ordered entry"]);
}

#[test]
fn prefixed_entries() {
    let mut file = Fixture::Generated("v4_more_files_IEOT").open();
    let entry = file.entry(0).clone();
    let new_entry_path = "an initially incorrectly ordered entry".into();
    file.dangerously_push_entry(entry.stat, entry.id, entry.flags, entry.mode, new_entry_path);
    file.sort_entries();

    check_prefix(&file, "a", &["a", "an initially incorrectly ordered entry"]);
    check_prefix(&file, "an", &["an initially incorrectly ordered entry"]);
    check_prefix(
        &file,
        "an initially incorrectly ordered entry",
        &["an initially incorrectly ordered entry"],
    );
    check_prefix(&file, "x", &["x"]);
    check_prefix(&file, "b", &["b"]);
    check_prefix(&file, "c", &["c"]);

    assert_eq!(
        file.prefixed_entries_range("".into()),
        Some(0..11),
        "empty prefixes match everything"
    );
    assert!(
        file.prefixed_entries_range("foo".into()).is_none(),
        "there is no match for this prefix"
    );
}

fn check_prefix(index: &gix_index::State, prefix: &str, expected: &[&str]) {
    assert_eq!(
        index
            .prefixed_entries(prefix.into())
            .unwrap_or_else(|| panic!("{prefix:?} must match at least one entry"))
            .iter()
            .map(|e| e.path(index))
            .collect::<Vec<_>>(),
        expected,
        "{prefix:?}"
    );
}

fn check_prefix_icase(index: &gix_index::State, prefix: &str, expected: &[&str]) {
    let range = index
        .prefixed_entries_range_icase(prefix.into(), true)
        .unwrap_or_else(|| panic!("{prefix:?} must match at least one entry"));
    assert_eq!(
        index.entries()[range].iter().map(|e| e.path(index)).collect::<Vec<_>>(),
        expected,
        "{prefix:?}"
    );
}
