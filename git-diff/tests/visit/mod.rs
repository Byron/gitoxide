mod changes {
    mod to_obtain_tree {
        use crate::hex_to_id;
        use git_diff::visit::recorder;
        use git_hash::{oid, ObjectId};
        use git_object::{bstr::ByteSlice, tree::EntryMode};
        use git_odb::{linked, pack, Locate};

        fn db() -> crate::Result<linked::Db> {
            linked::Db::at(
                test_tools::scripted_fixture_repo_read_only("make_diff_repo.sh")?
                    .join(".git")
                    .join("objects"),
            )
            .map_err(Into::into)
        }

        fn diff_with_previous_commit_from(db: &linked::Db, commit_id: &oid) -> crate::Result<recorder::Changes> {
            let mut buf = Vec::new();
            let (main_tree_id, parent_commit_id) = {
                let commit = db
                    .locate(commit_id, &mut buf, &mut pack::cache::Never)?
                    .ok_or_else(|| String::from(format!("start commit {:?} to be present", commit_id)))?
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

        fn head_of(db: &linked::Db) -> ObjectId {
            ObjectId::from_hex(
                &std::fs::read(
                    db.dbs[0]
                        .loose
                        .path
                        .parent()
                        .unwrap()
                        .join("refs")
                        .join("heads")
                        .join("main"),
                )
                .expect("head ref")
                .as_bstr()
                .trim(),
            )
            .expect("valid hex id")
        }

        fn all_commits(db: &linked::Db) -> Vec<ObjectId> {
            let head = head_of(db);
            git_odb::traverse::Ancestors::new(db, Some(head), &mut pack::cache::Never)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Result<Vec<_>, _>>()
                .expect("valid iteration")
        }

        #[test]
        fn many_different_states() -> crate::Result {
            let db = db()?;
            let all_commits = all_commits(&db);
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[0])?,
                vec![recorder::Change::Addition {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                    path: "f".into()
                }]
                , ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A      f");

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[1])?,
                vec![recorder::Change::Modification {
                    previous_entry_mode: EntryMode::Blob,
                    previous_oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                }]
                , ":100644 100644 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 28ce6a8b26aa170e1de65536fe8abe1832bd3242 M      f");

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[2])?,
                vec![recorder::Change::Deletion {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                }],
                ":100644 000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 0000000000000000000000000000000000000000 D	f
"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[4])?,
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
                diff_with_previous_commit_from(&db, &all_commits[5])?,
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
                diff_with_previous_commit_from(&db, &all_commits[8])?,
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
                diff_with_previous_commit_from(&db, &all_commits[10])?,
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
                diff_with_previous_commit_from(&db, &all_commits[12])?,
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
                diff_with_previous_commit_from(&db, &all_commits[13])?,
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
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[14])?,
                vec![
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"),
                        path: "g".into()
                    },
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "g/a".into()
                    },
                ],
                ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	g/a"
            );
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[15])?,
                vec![
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "c".into()
                    },
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "d".into()
                    },
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "e".into()
                    },
                ],
                ":100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	c
                 :100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	d
                 :100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	e"
            );
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[16])?,
                vec![
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f".into()
                    },
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "ff".into()
                    },
                ],
                ":100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	f
                  :000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	ff"
            );
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[17])?,
                vec![
                    recorder::Change::Modification {
                        previous_entry_mode: EntryMode::Tree,
                        previous_oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"),
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("6e5931346904b020301f74f581142826eacc4678"),
                        path: "g".into()
                    },
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "g/a".into()
                    },
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "g/aa".into()
                    },
                ],
                ":100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	g/a
                 :000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	g/aa"
            );
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[18])?,
                vec![
                    recorder::Change::Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f".into()
                    },
                    recorder::Change::Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "ff".into()
                    },
                ],
                ":100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	f
                  :000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	ff"
            );
            Ok(())
        }
    }
}
