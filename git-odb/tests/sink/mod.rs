use crate::loose::db::{locate_oid, object_ids};
use git_object::HashKind;
use git_odb::Write;

#[test]
fn write() -> Result<(), Box<dyn std::error::Error>> {
    for oid in object_ids() {
        let mut obj = locate_oid(oid);
        let actual = git_odb::sink().write(&obj.decode()?.into(), HashKind::Sha1)?;
        assert_eq!(actual, oid);
    }
    Ok(())
}
