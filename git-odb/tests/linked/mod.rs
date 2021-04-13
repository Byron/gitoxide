mod init {
    use git_odb::linked;

    #[test]
    fn a_linked_db_without_alternates() -> crate::Result {
        let tmp = tempdir::TempDir::new("alternates")?;
        let _ = linked::Db::at(tmp.path())?;
        Ok(())
    }
}
