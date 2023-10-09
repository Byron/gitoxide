use crate::index::Fixture;

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
    check_prefix(
        &file,
        "d/",
        &["d/a", "d/b", "d/c", "d/last/123", "d/last/34", "d/last/6"],
    );
    check_prefix(&file, "d/last", &["d/last/123", "d/last/34", "d/last/6"]);
    check_prefix(&file, "d/last/", &["d/last/123", "d/last/34", "d/last/6"]);
    check_prefix(&file, "d/las", &["d/last/123", "d/last/34", "d/last/6"]);
    check_prefix(&file, "d/last/123", &["d/last/123"]);
    check_prefix(&file, "d/last/34", &["d/last/34"]);
    check_prefix(&file, "d/last/6", &["d/last/6"]);
    check_prefix(&file, "x", &["x"]);
    check_prefix(&file, "a", &["a", "an initially incorrectly ordered entry"]);
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
