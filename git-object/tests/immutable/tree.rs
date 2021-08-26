mod iter {
    use git_object::{bstr::ByteSlice, tree, tree::EntryRef, TreeRefIter};

    use crate::{hex_to_id, immutable::fixture_bytes};

    #[test]
    fn empty() {
        assert_eq!(TreeRefIter::from_bytes(&[]).count(), 0, "empty trees are definitely ok");
    }

    #[test]
    fn error_handling() {
        let data = fixture_bytes("tree", "everything.tree");
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
            TreeRefIter::from_bytes(&fixture_bytes("tree", "everything.tree")).collect::<Result<Vec<_>, _>>()?,
            vec![
                EntryRef {
                    mode: tree::EntryMode::BlobExecutable,
                    filename: b"exe".as_bstr(),
                    oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                },
                EntryRef {
                    mode: tree::EntryMode::Blob,
                    filename: b"file".as_bstr(),
                    oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                },
                EntryRef {
                    mode: tree::EntryMode::Commit,
                    filename: b"grit-submodule".as_bstr(),
                    oid: &hex_to_id("b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8")
                },
                EntryRef {
                    mode: tree::EntryMode::Tree,
                    filename: b"subdir".as_bstr(),
                    oid: &hex_to_id("4d5fcadc293a348e88f777dc0920f11e7d71441c")
                },
                EntryRef {
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
    use git_object::{bstr::ByteSlice, tree, tree::EntryRef, TreeRef};

    use crate::{hex_to_id, immutable::fixture_bytes};

    #[test]
    fn empty() -> crate::Result {
        assert_eq!(
            TreeRef::from_bytes(&[])?,
            TreeRef { entries: vec![] },
            "empty trees are valid despite usually rare in the wild"
        );
        Ok(())
    }

    #[test]
    fn everything() -> crate::Result {
        assert_eq!(
            TreeRef::from_bytes(&fixture_bytes("tree", "everything.tree"))?,
            TreeRef {
                entries: vec![
                    EntryRef {
                        mode: tree::EntryMode::BlobExecutable,
                        filename: b"exe".as_bstr(),
                        oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                    },
                    EntryRef {
                        mode: tree::EntryMode::Blob,
                        filename: b"file".as_bstr(),
                        oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                    },
                    EntryRef {
                        mode: tree::EntryMode::Commit,
                        filename: b"grit-submodule".as_bstr(),
                        oid: &hex_to_id("b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8")
                    },
                    EntryRef {
                        mode: tree::EntryMode::Tree,
                        filename: b"subdir".as_bstr(),
                        oid: &hex_to_id("4d5fcadc293a348e88f777dc0920f11e7d71441c")
                    },
                    EntryRef {
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
            TreeRef::from_bytes(&fixture_bytes("tree", "maybe-special.tree"))?
                .entries
                .len(),
            160
        );
        Ok(())
    }

    #[test]
    fn definitely_special() -> crate::Result {
        assert_eq!(
            TreeRef::from_bytes(&fixture_bytes("tree", "definitely-special.tree"))?
                .entries
                .len(),
            19
        );
        Ok(())
    }
}
