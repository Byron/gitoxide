use gix_object::{bstr::ByteSlice, tree, tree::EntryRef, TreeRefIter};

use crate::{fixture_name, hex_to_id};

#[test]
fn empty() {
    assert_eq!(TreeRefIter::from_bytes(&[]).count(), 0, "empty trees are definitely ok");
}

#[test]
fn error_handling() {
    let data = fixture_name("tree", "everything.tree");
    let iter = TreeRefIter::from_bytes(&data[..data.len() / 2]);
    let entries = iter.collect::<Vec<_>>();
    assert!(
        entries.last().expect("at least one token").is_err(),
        "errors are propagated and none is returned from that point on"
    );
}

#[test]
fn everything() -> crate::Result {
    assert_eq!(
        TreeRefIter::from_bytes(&fixture_name("tree", "everything.tree")).collect::<Result<Vec<_>, _>>()?,
        vec![
            EntryRef {
                mode: tree::EntryKind::BlobExecutable.into(),
                filename: b"exe".as_bstr(),
                oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            EntryRef {
                mode: tree::EntryKind::Blob.into(),
                filename: b"file".as_bstr(),
                oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            EntryRef {
                mode: tree::EntryKind::Commit.into(),
                filename: b"grit-submodule".as_bstr(),
                oid: &hex_to_id("b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8")
            },
            EntryRef {
                mode: tree::EntryKind::Tree.into(),
                filename: b"subdir".as_bstr(),
                oid: &hex_to_id("4d5fcadc293a348e88f777dc0920f11e7d71441c")
            },
            EntryRef {
                mode: tree::EntryKind::Link.into(),
                filename: b"symlink".as_bstr(),
                oid: &hex_to_id("1a010b1c0f081b2e8901d55307a15c29ff30af0e")
            }
        ]
    );
    Ok(())
}
