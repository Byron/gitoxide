mod iter {
    use git_object::{
        bstr::ByteSlice,
        immutable::{tree::Entry, TreeIter},
        tree,
    };

    use crate::{hex_to_id, immutable::fixture_bytes};

    #[test]
    fn empty() {
        assert_eq!(TreeIter::from_bytes(&[]).count(), 0, "empty trees are definitely ok");
    }

    #[test]
    fn error_handling() {
        let data = fixture_bytes("tree", "everything.tree");
        let iter = TreeIter::from_bytes(&data[..data.len() / 2]);
        let entries = iter.collect::<Vec<_>>();
        assert!(
            entries.last().expect("at least one token").is_err(),
            "errors are propagated and none is returned from that point on"
        );
    }

    #[test]
    fn everything() -> crate::Result {
        assert_eq!(
            TreeIter::from_bytes(&fixture_bytes("tree", "everything.tree")).collect::<Result<Vec<_>, _>>()?,
            vec![
                Entry {
                    mode: tree::EntryMode::BlobExecutable,
                    filename: b"exe".as_bstr(),
                    oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                },
                Entry {
                    mode: tree::EntryMode::Blob,
                    filename: b"file".as_bstr(),
                    oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                },
                Entry {
                    mode: tree::EntryMode::Commit,
                    filename: b"grit-submodule".as_bstr(),
                    oid: &hex_to_id("b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8")
                },
                Entry {
                    mode: tree::EntryMode::Tree,
                    filename: b"subdir".as_bstr(),
                    oid: &hex_to_id("4d5fcadc293a348e88f777dc0920f11e7d71441c")
                },
                Entry {
                    mode: tree::EntryMode::Link,
                    filename: b"symlink".as_bstr(),
                    oid: &hex_to_id("1a010b1c0f081b2e8901d55307a15c29ff30af0e")
                }
            ]
        );
        Ok(())
    }
}

mod from_bytes {
    use git_object::{
        bstr::ByteSlice,
        immutable::{tree::Entry, Tree},
        tree,
    };

    use crate::{hex_to_id, immutable::fixture_bytes};

    #[test]
    fn empty() -> crate::Result {
        assert_eq!(
            Tree::from_bytes(&[])?,
            Tree { entries: vec![] },
            "empty trees are valid despite usually rare in the wild"
        );
        Ok(())
    }

    #[test]
    fn everything() -> crate::Result {
        assert_eq!(
            Tree::from_bytes(&fixture_bytes("tree", "everything.tree"))?,
            Tree {
                entries: vec![
                    Entry {
                        mode: tree::EntryMode::BlobExecutable,
                        filename: b"exe".as_bstr(),
                        oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                    },
                    Entry {
                        mode: tree::EntryMode::Blob,
                        filename: b"file".as_bstr(),
                        oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                    },
                    Entry {
                        mode: tree::EntryMode::Commit,
                        filename: b"grit-submodule".as_bstr(),
                        oid: &hex_to_id("b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8")
                    },
                    Entry {
                        mode: tree::EntryMode::Tree,
                        filename: b"subdir".as_bstr(),
                        oid: &hex_to_id("4d5fcadc293a348e88f777dc0920f11e7d71441c")
                    },
                    Entry {
                        mode: tree::EntryMode::Link,
                        filename: b"symlink".as_bstr(),
                        oid: &hex_to_id("1a010b1c0f081b2e8901d55307a15c29ff30af0e")
                    }
                ]
            }
        );
        Ok(())
    }

    #[test]
    fn maybe_special() -> crate::Result {
        assert_eq!(
            Tree::from_bytes(&fixture_bytes("tree", "maybe-special.tree"))?
                .entries
                .len(),
            160
        );
        Ok(())
    }

    #[test]
    fn definitely_special() -> crate::Result {
        assert_eq!(
            Tree::from_bytes(&fixture_bytes("tree", "definitely-special.tree"))?
                .entries
                .len(),
            19
        );
        Ok(())
    }
}
