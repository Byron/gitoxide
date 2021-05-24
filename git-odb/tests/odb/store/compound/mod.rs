use git_odb::store::compound::Db;

use crate::fixture_path;

fn db() -> Db {
    Db::at(fixture_path("objects")).expect("valid object path")
}

mod init {
    use crate::odb::store::compound::db;

    #[test]
    fn has_packs() {
        assert_eq!(db().bundles.len(), 3)
    }
}

mod locate {
    use git_odb::store::compound::Db;

    use crate::hex_to_id;
    use crate::odb::store::compound::db;

    fn can_locate(db: &Db, hex_id: &str) {
        let mut buf = vec![];
        assert!(db
            .find(hex_to_id(hex_id), &mut buf, &mut git_odb::pack::cache::Never)
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
