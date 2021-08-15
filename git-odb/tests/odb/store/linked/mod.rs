use git_odb::linked::Store;

use crate::fixture_path;

fn db() -> Store {
    Store::at(fixture_path("objects")).expect("valid object path")
}

mod iter {
    use crate::odb::store::linked::db;

    #[test]
    fn arc_iter() {
        let db = std::sync::Arc::new(db());
        let _ = db.arc_iter();
    }

    #[test]
    fn a_bunch_of_loose_and_packed_objects() -> crate::Result {
        let db = db();
        let iter = db.iter();
        assert_eq!(
            iter.size_hint(),
            (139, None),
            "we only count packs and have no upper bound"
        );
        assert_eq!(iter.count(), 146, "it sees the correct amount of objects");
        for id in db.iter() {
            assert!(db.contains(id?), "each object exists");
        }
        Ok(())
    }
}

mod locate {
    use git_odb::{linked::Store, pack, Find};

    use crate::{hex_to_id, odb::store::linked::db};

    fn can_locate(db: &Store, hex_id: &str) {
        let mut buf = vec![];
        assert!(db
            .find(hex_to_id(hex_id), &mut buf, &mut pack::cache::Never)
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
    use std::convert::TryFrom;

    use git_odb::linked;

    use crate::odb::{alternate::alternate, store::linked::db};

    #[test]
    fn multiple_linked_repositories_via_alternates() -> crate::Result {
        let tmp = git_testtools::tempfile::TempDir::new()?;
        let (object_path, linked_object_path) = alternate(tmp.path().join("a"), tmp.path().join("b"))?;
        let db = linked::Store::try_from(object_path.clone())?;
        assert_eq!(db.dbs.len(), 2);
        assert_eq!(db.dbs[0].loose.path, object_path);
        assert_eq!(db.dbs[1].loose.path, linked_object_path);
        Ok(())
    }

    #[test]
    fn a_linked_db_without_alternates() -> crate::Result {
        let tmp = git_testtools::tempfile::TempDir::new()?;
        let db = linked::Store::at(tmp.path())?;
        assert_eq!(db.dbs.len(), 1);
        assert_eq!(db.dbs[0].loose.path, tmp.path());
        Ok(())
    }

    #[test]
    fn has_packs() {
        let db = db();
        assert_eq!(db.dbs.len(), 1);
        assert_eq!(db.dbs[0].bundles.len(), 3)
    }
}
