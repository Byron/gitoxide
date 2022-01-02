use git_odb::Write;

use crate::store::loose::{locate_oid, object_ids};

#[test]
fn write() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    for oid in object_ids() {
        let obj = locate_oid(oid, &mut buf);
        let actual = git_odb::sink(git_hash::Kind::Sha1).write(&obj.decode()?)?;
        assert_eq!(actual, oid);
    }
    Ok(())
}
