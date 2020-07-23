use crate::loose::db::{locate, object_ids};
use git_object::owned;
use git_odb::Write;

fn id_to_hex(id: &owned::Id) -> String {
    std::str::from_utf8(&id.to_sha1_hex()[..]).unwrap().to_owned()
}

#[test]
fn write() {
    for oid in object_ids() {
        let mut obj = locate(&id_to_hex(&oid));
        assert_eq!(git_odb::sink().write(&obj.decode().unwrap().into()).unwrap(), oid);
    }
}
