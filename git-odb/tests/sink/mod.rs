use crate::loose::db::{locate_oid, object_ids};
use git_object::HashKind;
use git_odb::Write;

#[test]
fn write() {
    for oid in object_ids() {
        let mut obj = locate_oid(oid);
        let actual = git_odb::sink()
            .write(&obj.decode().unwrap().into(), HashKind::Sha1)
            .unwrap();
        assert_eq!(actual, oid);
    }
}
