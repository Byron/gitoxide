mod changes {
    mod to_obtain_tree {
        use crate::hex_to_id;
        use git_diff::visit::recorder;
        use git_object::tree::EntryMode;
        use git_odb::{linked, pack, Locate};

        const COMMIT_1: &str = "055df97e18cd537da3cb16bcbdf1733fdcdfb430";
        const COMMIT_2: &str = "a5ebf9ee3b1cac5daf3dc9056026ee848be52da2";
        const COMMIT_3: &str = "65cd7e777303b4b3a2d41e81303b5c2dd15041fa";
        const COMMIT_5: &str = "69bbebb6608472d98be684f4e6ef1faaac2a03bc";
        const COMMIT_6: &str = "9bd749db486b2af4a0d4df2de1972db2f198903d";
        const COMMIT_9: &str = "ac0a340c76810b53b23e6dc44cf1445ebbd52201";
        const COMMIT_11: &str = "76a3f837e9b4aad1840df6be5ca413d696eabc9d";
        const COMMIT_13: &str = "05533d594489fae72d4e7422fbdf061c1b70bc22";
        const COMMIT_14: &str = "ac7c4c37c3939b820f3ff9003a7ed11d6143dc2b";

        fn db() -> crate::Result<linked::Db> {
            linked::Db::at(
                test_tools::scripted_fixture_repo_read_only("make_diff_repo.sh")?
                    .join(".git")
                    .join("objects"),
            )
            .map_err(Into::into)
        }

        fn diff_with_previous_commit_from(db: &linked::Db, commit_id: &str) -> crate::Result<recorder::Changes> {
            let commit_id = git_hash::ObjectId::from_hex(commit_id.as_bytes())?;
            let mut buf = Vec::new();
            let (main_tree_id, parent_commit_id) = {
                let commit = db
                    .locate(commit_id, &mut buf, &mut pack::cache::Never)?
                    .expect("start commit to be present")
                    .decode()?
                    .into_commit()
                    .expect("id is actually a commit");

                (commit.tree(), {
                    let p = commit.parents().next();
                    p
                })
            };
            let main_tree = db
                .locate(main_tree_id, &mut buf, &mut pack::cache::Never)?
                .expect("main tree present")
                .into_tree_iter()
                .expect("id to be a tree");
            let mut buf2 = Vec::new();
            let previous_tree: Option<_> = {
                parent_commit_id
                    .and_then(|id| db.locate(id, &mut buf2, &mut pack::cache::Never).ok().flatten())
                    .and_then(|c| c.decode().ok())
                    .and_then(|c| c.into_commit())
                    .map(|c| c.tree())
                    .and_then(|tree| db.locate(tree, &mut buf2, &mut pack::cache::Never).ok().flatten())
                    .and_then(|tree| tree.into_tree_iter())
            };

            let mut recorder = git_diff::visit::Recorder::default();
            git_diff::visit::Changes::from(previous_tree).needed_to_obtain(
                main_tree,
                &mut git_diff::visit::State::default(),
                |oid, buf| {
                    db.locate(oid, buf, &mut pack::cache::Never)
                        .ok()
                        .flatten()
                        .and_then(|obj| obj.into_tree_iter())
                },
                &mut recorder,
            )?;
            Ok(recorder.records)
        }

        #[test]
        fn many_different_states() -> crate::Result {
            let db = db()?;
            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_1)?,
                vec![recorder::Change::Addition {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                    path: "f".into()
                }]
                , ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A      f");

            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_2)?,
                vec![recorder::Change::Modification {
                    previous_entry_mode: EntryMode::Blob,
                    previous_oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                }]
                , ":100644 100644 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 28ce6a8b26aa170e1de65536fe8abe1832bd3242 M      f");

            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_3)?,
                vec![recorder::Change::Deletion {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                }],
                ":100644 000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 0000000000000000000000000000000000000000 D	f
"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_5)?,
                vec![recorder::Change::Deletion {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                },
                recorder::Change::Addition {
                    entry_mode: EntryMode::Tree,
                    oid: hex_to_id("10f2f4b82222d2b5c31985130979a91fd87410f7"),
                    path: "f".into()
                },
                recorder::Change::Addition {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f/f".into()
                }]
                , ":100644 000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 0000000000000000000000000000000000000000 D      f
                   :000000 100644 0000000000000000000000000000000000000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 A      f/f");

            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_6)?,
                vec![
                    recorder::Change::Modification {
                        previous_entry_mode: EntryMode::Tree,
                        previous_oid: hex_to_id("10f2f4b82222d2b5c31985130979a91fd87410f7"),
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("ebbe0b3000afdfd1aed15000094b59a2800328eb"),
                        path: "f".into()
                    },
                    recorder::Change::Modification {
                        previous_entry_mode: EntryMode::Blob,
                        previous_oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("13c2aca72ab576cb5f22dc8e7f8ba8ddab553a8a"),
                        path: "f/f".into()
                    },
                ],
                ":100644 100644 28ce6a8b26aa170e1de65536fe8abe1832bd3242 13c2aca72ab576cb5f22dc8e7f8ba8ddab553a8a M	f/f"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_9)?,
                vec![
                    recorder::Change::Modification {
                        previous_entry_mode: EntryMode::Tree,
                        previous_oid: hex_to_id("849bd76db90b65ebbd2e6d3970ca70c96ee5592c"),
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("7e26dba59b6336f87d1d4ae3505a2da302b91c76"),
                        path: "f".into()
                    },
                    recorder::Change::Modification {
                        previous_entry_mode: EntryMode::Blob,
                        previous_oid: hex_to_id("13c2aca72ab576cb5f22dc8e7f8ba8ddab553a8a"),
                        entry_mode: EntryMode::Link,
                        oid: hex_to_id("2e65efe2a145dda7ee51d1741299f848e5bf752e"),
                        path: "f/f".into()
                    },
                ],
                ":100644 120000 13c2aca72ab576cb5f22dc8e7f8ba8ddab553a8a 2e65efe2a145dda7ee51d1741299f848e5bf752e T	f/f"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_11)?,
                vec![
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("7e26dba59b6336f87d1d4ae3505a2da302b91c76"),
                        path: "f".into()
                    },
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f".into()
                    },
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f/a".into()
                    },
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f/b".into()
                    },
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Link,
                        oid: hex_to_id("2e65efe2a145dda7ee51d1741299f848e5bf752e"),
                        path: "f/f".into()
                    },
                ],
                ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	f
                 :100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	f/a
                 :100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	f/b
                 :120000 000000 2e65efe2a145dda7ee51d1741299f848e5bf752e 0000000000000000000000000000000000000000 D	f/f"
            );
            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_13)?,
                vec![
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("3d5a503f4062d198b443db5065ca727f8354e7df"),
                        path: "d".into()
                    },
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "d/f".into()
                    },
                ],
                ":100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	d/f"
            );
            assert_eq!(
                diff_with_previous_commit_from(&db, COMMIT_14)?,
                vec![
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "c".into()
                    },
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "d".into()
                    },
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "e".into()
                    },
                ],
                ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	c
                 :000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	d
                 :000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	e"
            );
            Ok(())
        }
    }
}
