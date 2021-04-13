mod init {
    use crate::alternate::alternate;
    use git_odb::linked;
    use std::convert::TryFrom;

    #[test]
    fn multiple_linked_repositories_via_alternates() -> crate::Result {
        let tmp = tempdir::TempDir::new("alternates")?;
        let (object_path, linked_object_path) = alternate(tmp.path().join("a"), tmp.path().join("b"))?;
        let db = linked::Db::try_from(object_path.clone())?;
        assert_eq!(db.dbs.len(), 2);
        assert_eq!(db.dbs[0].loose.path, object_path);
        assert_eq!(db.dbs[1].loose.path, linked_object_path);
        Ok(())
    }

    #[test]
    fn a_linked_db_without_alternates() -> crate::Result {
        let tmp = tempdir::TempDir::new("alternates")?;
        let db = linked::Db::at(tmp.path())?;
        assert_eq!(db.dbs.len(), 1);
        assert_eq!(db.dbs[0].loose.path, tmp.path());
        Ok(())
    }
}
