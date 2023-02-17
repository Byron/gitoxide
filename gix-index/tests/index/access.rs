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
        "new entry can't be found to to incorrect order"
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
}
