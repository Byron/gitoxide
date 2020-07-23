use crate::loose::db::{locate, object_ids};
use git_object::{owned, HashKind};
use git_odb::Write;

fn id_to_hex(id: &owned::Id) -> String {
    std::str::from_utf8(&id.to_sha1_hex()[..]).unwrap().to_owned()
}

#[test]
fn write() {
    for oid in object_ids() {
        let mut obj = locate(&id_to_hex(&oid));
        let actual = git_odb::sink()
            .write(&obj.decode().unwrap().into(), HashKind::Sha1)
            .unwrap();
        assert_eq!(actual, oid);
    }
}
