use crate::{fixture_path, hex_to_id};
use git_odb::compound::Db;

fn db() -> Db {
    Db::at(fixture_path("objects")).expect("valid object path")
}

fn can_locate(db: &Db, hex_id: &str) {
    let mut buf = vec![];
    assert!(db.locate(hex_to_id(hex_id).to_borrowed(), &mut buf).is_some());
}

#[test]
fn loose_object_lookup() {
    can_locate(&db(), "37d4e6c5c48ba0d245164c4e10d5f41140cab980");
}

#[test]
#[should_panic]
fn pack_object_lookup() {
    can_locate(&db(), "501b297447a8255d3533c6858bb692575cdefaa0");
}
