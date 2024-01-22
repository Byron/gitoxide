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
}

mod from_bytes {
    use gix_object::{bstr::ByteSlice, tree, tree::EntryRef, TreeRef, TreeRefIter};

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
            assert_eq!(
                TreeRef::from_bytes(&fixture)?.entries.len(),
                expected_entry_count,
                "{name}"
            );
            assert_eq!(
                TreeRefIter::from_bytes(&fixture).map(Result::unwrap).count(),
                expected_entry_count,
                "{name}"
            );
        }
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
    use gix_object::tree::{EntryKind, EntryMode};

    #[test]
    fn size_in_bytes() {
        assert_eq!(
            std::mem::size_of::<EntryMode>(),
            2,
            "it should not change without notice"
        );
    }

    #[test]
    fn is_methods() {
        fn mode(kind: EntryKind) -> EntryMode {
            kind.into()
        }

        assert!(mode(EntryKind::Blob).is_blob());
        assert!(EntryMode(0o100645).is_blob());
        assert_eq!(EntryMode(0o100645).kind(), EntryKind::Blob);
        assert!(!EntryMode(0o100675).is_executable());
        assert!(EntryMode(0o100700).is_executable());
        assert_eq!(EntryMode(0o100700).kind(), EntryKind::BlobExecutable);
        assert!(!mode(EntryKind::Blob).is_link());
        assert!(mode(EntryKind::BlobExecutable).is_blob());
        assert!(mode(EntryKind::BlobExecutable).is_executable());
        assert!(mode(EntryKind::Blob).is_blob_or_symlink());
        assert!(mode(EntryKind::BlobExecutable).is_blob_or_symlink());

        assert!(!mode(EntryKind::Link).is_blob());
        assert!(mode(EntryKind::Link).is_link());
        assert!(EntryMode(0o121234).is_link());
        assert_eq!(EntryMode(0o121234).kind(), EntryKind::Link);
        assert!(mode(EntryKind::Link).is_blob_or_symlink());
        assert!(mode(EntryKind::Tree).is_tree());
        assert!(EntryMode(0o040101).is_tree());
        assert_eq!(EntryMode(0o040101).kind(), EntryKind::Tree);
        assert!(mode(EntryKind::Commit).is_commit());
        assert!(EntryMode(0o167124).is_commit());
        assert_eq!(EntryMode(0o167124).kind(), EntryKind::Commit);
        assert_eq!(
            EntryMode(0o000000).kind(),
            EntryKind::Commit,
            "commit is really 'anything else' as `kind()` can't fail"
        );
    }

    #[test]
    fn as_bytes() {
        let mut buf = Default::default();
        for (mode, expected) in [
            (EntryMode::from(EntryKind::Tree), EntryKind::Tree.as_octal_str()),
            (EntryKind::Blob.into(), EntryKind::Blob.as_octal_str()),
            (
                EntryKind::BlobExecutable.into(),
                EntryKind::BlobExecutable.as_octal_str(),
            ),
            (EntryKind::Link.into(), EntryKind::Link.as_octal_str()),
            (EntryKind::Commit.into(), EntryKind::Commit.as_octal_str()),
            (
                EntryMode::try_from(b"100744 ".as_ref()).expect("valid"),
                "100744".into(),
            ),
            (
                EntryMode::try_from(b"100644 ".as_ref()).expect("valid"),
                "100644".into(),
            ),
        ] {
            assert_eq!(mode.as_bytes(&mut buf), expected)
        }
    }
}
