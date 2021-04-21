use crate::fixture_path;
use git_odb::linked::Db;

fn db() -> Db {
    Db::at(fixture_path("objects")).expect("valid object path")
}

mod iter {
    use crate::linked::db;

    #[test]
    fn arc_iter() {
        let db = std::sync::Arc::new(db());
        let _ = db.arc_iter();
    }

    #[test]
    fn a_bunch_of_loose_and_packed_objects() {
        let db = db();
        let iter = db.iter();
        assert_eq!(
            iter.size_hint(),
            (139, None),
            "we only count packs and have no upper bound"
        );
        assert_eq!(iter.count(), 140, "it sees the correct amount of objects");
    }
}

mod locate {
    use crate::{hex_to_id, linked::db};
    use git_odb::{linked::Db, pack};

    fn can_locate(db: &Db, hex_id: &str) {
        let mut buf = vec![];
        assert!(db
            .locate(hex_to_id(hex_id), &mut buf, &mut pack::cache::Noop)
            .expect("no read error")
            .is_some());
    }

    #[test]
    fn loose_object() {
        can_locate(&db(), "37d4e6c5c48ba0d245164c4e10d5f41140cab980");
    }

    #[test]
    fn pack_object() {
        can_locate(&db(), "501b297447a8255d3533c6858bb692575cdefaa0"); // pack 11fd
        can_locate(&db(), "4dac9989f96bc5b5b1263b582c08f0c5f0b58542"); // pack a2bf
        can_locate(&db(), "dd25c539efbb0ab018caa4cda2d133285634e9b5"); // pack c043
    }
}

mod init {
    use crate::{alternate::alternate, linked::db};
    use git_odb::linked;
    use std::convert::TryFrom;

    #[test]
    fn multiple_linked_repositories_via_alternates() -> crate::Result {
        let tmp = test_tools::tempdir::TempDir::new("alternates")?;
        let (object_path, linked_object_path) = alternate(tmp.path().join("a"), tmp.path().join("b"))?;
        let db = linked::Db::try_from(object_path.clone())?;
        assert_eq!(db.dbs.len(), 2);
        assert_eq!(db.dbs[0].loose.path, object_path);
        assert_eq!(db.dbs[1].loose.path, linked_object_path);
        Ok(())
    }

    #[test]
    fn a_linked_db_without_alternates() -> crate::Result {
        let tmp = test_tools::tempdir::TempDir::new("alternates")?;
        let db = linked::Db::at(tmp.path())?;
        assert_eq!(db.dbs.len(), 1);
        assert_eq!(db.dbs[0].loose.path, tmp.path());
        Ok(())
    }

    #[test]
    fn has_packs() {
        let db = db();
        assert_eq!(db.dbs.len(), 1);
        assert_eq!(db.dbs[0].packs.len(), 3)
    }
}
