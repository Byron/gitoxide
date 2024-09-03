use gix_object::{bstr::ByteSlice, tree, tree::EntryRef, Tree, TreeRef, TreeRefIter, WriteTo};

use crate::{fixture_name, hex_to_id};

#[test]
fn empty() -> crate::Result {
    let tree_ref = TreeRef::from_bytes(&[])?;
    assert_eq!(
        tree_ref,
        TreeRef { entries: vec![] },
        "empty trees are valid despite usually rare in the wild"
    );

    let mut buf = Vec::new();
    tree_ref.write_to(&mut buf)?;
    assert!(buf.is_empty());

    buf.clear();
    Tree::from(tree_ref).write_to(&mut buf)?;
    assert!(buf.is_empty());
    Ok(())
}

#[test]
fn everything() -> crate::Result {
    let fixture = fixture_name("tree", "everything.tree");
    let tree_ref = TreeRef::from_bytes(&fixture)?;
    assert_eq!(
        tree_ref,
        TreeRef {
            entries: vec![
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
        }
    );
    Ok(())
}

#[test]
fn invalid() {
    let fixture = fixture_name("tree", "definitely-special.tree");
    let partial_tree = &fixture[..fixture.len() / 2];
    let err = TreeRef::from_bytes(partial_tree).unwrap_err().to_string();
    if cfg!(feature = "verbose-object-parsing-errors") {
        assert!(err.starts_with("object parsing failed at `100644"), "{err}");
    } else {
        assert_eq!(err, "object parsing failed");
    }
    assert_eq!(
        TreeRefIter::from_bytes(partial_tree).take_while(Result::is_ok).count(),
        9,
        "we can decode about half of it before failing"
    );
}

#[test]
fn fuzzed() {
    assert!(gix_object::TreeRef::from_bytes(b"2").is_err(), "fail, but don't crash");
}

#[test]
fn special_trees() -> crate::Result {
    for (name, expected_entry_count) in [
        ("maybe-special", 160),
        ("definitely-special", 19),
        ("special-1", 5),
        ("special-2", 18),
        ("special-3", 5),
        ("special-4", 18),
        ("special-5", 17),
    ] {
        let fixture = fixture_name("tree", &format!("{name}.tree"));
        let actual = TreeRef::from_bytes(&fixture)?;
        assert_eq!(actual.entries.len(), expected_entry_count, "{name}");
        assert_eq!(
            TreeRefIter::from_bytes(&fixture).map(Result::unwrap).count(),
            expected_entry_count,
            "{name}"
        );
    }
    Ok(())
}
