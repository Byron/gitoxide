use crate::index::file::read;

#[test]
fn entry_by_path_and_stage() {
    let file = read::file("v4_more_files_IEOT");
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

mod set_path {
    use crate::index::file::read;

    #[test]
    fn future_writes_respect_the_newly_set_path() -> crate::Result {
        let mut file = read::file("v4_more_files_IEOT");
        let tmp = git_testtools::tempfile::TempDir::new()?;
        let new_index_path = tmp.path().join("new-index");

        file.set_path(&new_index_path);
        assert!(!new_index_path.is_file());
        assert_eq!(file.path(), new_index_path);

        file.write(Default::default())?;
        assert_eq!(file.path(), new_index_path);
        assert!(new_index_path.is_file());

        Ok(())
    }
}
