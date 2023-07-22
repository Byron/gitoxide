mod iter {
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
    use gix_object::{bstr::ByteSlice, tree, tree::EntryRef, TreeRef};

    use crate::{fixture_name, hex_to_id};

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
            TreeRef::from_bytes(&fixture_name("tree", "everything.tree"))?,
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
            TreeRef::from_bytes(&fixture_name("tree", "maybe-special.tree"))?
                .entries
                .len(),
            160
        );
        Ok(())
    }

    #[test]
    fn definitely_special() -> crate::Result {
        assert_eq!(
            TreeRef::from_bytes(&fixture_name("tree", "definitely-special.tree"))?
                .entries
                .len(),
            19
        );
        Ok(())
    }
}

mod entries {
    use gix_object::{Tree, TreeRef};

    #[test]
    fn sort_order_is_correct() -> crate::Result {
        let root = gix_testtools::scripted_fixture_read_only("make_trees.sh")?;
        let input = std::fs::read(root.join("tree.baseline"))?;

        let mut tree = TreeRef::from_bytes(&input)?;
        let expected = tree.entries.clone();

        tree.entries.sort();
        assert_eq!(tree.entries, expected);
        let mut failures_when_searching_by_name = 0;
        for entry in expected {
            assert!(
                tree.entries.binary_search_by(|e| e.cmp(&entry)).is_ok(),
                "ordering works with binary search"
            );
            failures_when_searching_by_name += usize::from(
                tree.entries
                    .binary_search_by(|e| e.filename.cmp(entry.filename))
                    .is_err(),
            );
            assert_eq!(
                tree.bisect_entry(entry.filename, entry.mode.is_tree())
                    .expect("entry is present"),
                entry
            )
        }

        assert_eq!(
            failures_when_searching_by_name, 2,
            "it's not possible to do a binary search by name alone"
        );

        let mut tree: Tree = tree.into();
        let expected = tree.entries.clone();
        tree.entries.sort();

        assert_eq!(tree.entries, expected);
        Ok(())
    }
}

mod entry_mode {
    use gix_object::tree::EntryMode;

    #[test]
    fn size_in_bytes() {
        assert_eq!(
            std::mem::size_of::<EntryMode>(),
            2,
            "it should not change without notice"
        );
    }
}
