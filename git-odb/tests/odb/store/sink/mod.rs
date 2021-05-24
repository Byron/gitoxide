use crate::store::loose::backend::{locate_oid, object_ids};
use git_odb::write::Write;

#[test]
fn write() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    for oid in object_ids() {
        let obj = locate_oid(oid, &mut buf);
        let actual = git_odb::store::sink().write(&obj.decode()?.into(), git_hash::Kind::Sha1)?;
        assert_eq!(actual, oid);
    }
    Ok(())
}
