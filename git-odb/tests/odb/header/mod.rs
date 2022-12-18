use crate::fixture_path;

fn new_store() -> git_odb::Handle {
    git_odb::at(fixture_path("objects")).expect("valid object path")
}

use crate::hex_to_id;

fn find_header(db: impl git_odb::Header, hex_id: &str) -> git_odb::find::Header {
    db.try_header(hex_to_id(hex_id))
        .expect("no read error")
        .expect("object exists")
}

#[test]
fn loose_object() {
    find_header(&new_store(), "37d4e6c5c48ba0d245164c4e10d5f41140cab980");
}

#[test]
fn pack_object() {
    let db = new_store();
    assert_eq!(
        find_header(&db, "501b297447a8255d3533c6858bb692575cdefaa0"), // pack 11fd
        git_odb::find::Header::Packed(git_pack::data::decode::header::Outcome {
            kind: git_object::Kind::Commit,
            object_size: 225,
            num_deltas: 0,
        })
    );
    assert_eq!(
        find_header(&db, "4dac9989f96bc5b5b1263b582c08f0c5f0b58542"), // pack a2bf
        git_odb::find::Header::Packed(git_pack::data::decode::header::Outcome {
            kind: git_object::Kind::Tree,
            object_size: 34,
            num_deltas: 0,
        })
    );
    assert_eq!(
        find_header(&db, "dd25c539efbb0ab018caa4cda2d133285634e9b5"), // pack c043
        git_odb::find::Header::Packed(git_pack::data::decode::header::Outcome {
            kind: git_object::Kind::Blob,
            object_size: 860,
            num_deltas: 0,
        })
    );
}
