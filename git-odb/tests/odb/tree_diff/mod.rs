mod changes {
    mod to_obtain_tree {
        use git_diff::tree::{recorder, recorder::Change::*};
        use git_hash::{oid, ObjectId};
        use git_object::{bstr::ByteSlice, tree::EntryMode, TreeRefIter};
        use git_odb::pack::Find;

        use crate::hex_to_id;

        type Changes = Vec<recorder::Change>;

        fn db(args: impl IntoIterator<Item = &'static str>) -> crate::Result<git_odb::Handle> {
            git_odb::at(
                git_testtools::scripted_fixture_read_only_with_args("make_diff_repo.sh", args)?
                    .join(".git")
                    .join("objects"),
            )
            .map_err(Into::into)
        }

        fn locate_tree_by_commit<'a>(
            db: &git_odb::Handle,
            commit: &oid,
            buf: &'a mut Vec<u8>,
        ) -> crate::Result<TreeRefIter<'a>> {
            let tree_id = db
                .try_find(commit, buf)?
                .ok_or_else(|| format!("start commit {:?} to be present", commit))?
                .0
                .decode()?
                .into_commit()
                .expect("id is actually a commit")
                .tree();

            Ok(db
                .try_find(tree_id, buf)?
                .expect("main tree present")
                .0
                .try_into_tree_iter()
                .expect("id to be a tree"))
        }

        fn diff_commits(db: &git_odb::Handle, lhs: impl Into<Option<ObjectId>>, rhs: &oid) -> crate::Result<Changes> {
            let mut buf = Vec::new();
            let lhs_tree = lhs
                .into()
                .and_then(|lhs| locate_tree_by_commit(db, &lhs, &mut buf).ok());
            let mut buf2 = Vec::new();
            let rhs_tree = locate_tree_by_commit(db, rhs, &mut buf2)?;
            let mut recorder = git_diff::tree::Recorder::default();
            git_diff::tree::Changes::from(lhs_tree).needed_to_obtain(
                rhs_tree,
                git_diff::tree::State::default(),
                |oid, buf| {
                    use git_odb::pack::FindExt;
                    db.find(oid, buf)
                        .map(|obj| obj.0.try_into_tree_iter().expect("only called for trees"))
                },
                &mut recorder,
            )?;
            Ok(recorder.records)
        }

        fn diff_with_previous_commit_from(db: &git_odb::Handle, commit_id: &oid) -> crate::Result<Changes> {
            let mut buf = Vec::new();
            let (main_tree_id, parent_commit_id) = {
                let commit = db
                    .try_find(commit_id, &mut buf)?
                    .ok_or_else(|| format!("start commit {:?} to be present", commit_id))?
                    .0
                    .decode()?
                    .into_commit()
                    .expect("id is actually a commit");

                (commit.tree(), {
                    let p = commit.parents().next();
                    p
                })
            };
            let current_tree = db
                .try_find(main_tree_id, &mut buf)?
                .expect("main tree present")
                .0
                .try_into_tree_iter()
                .expect("id to be a tree");
            let mut buf2 = Vec::new();
            let previous_tree: Option<_> = {
                parent_commit_id
                    .and_then(|id| db.try_find(id, &mut buf2).ok().flatten())
                    .and_then(|(c, _l)| c.decode().ok())
                    .and_then(|c| c.into_commit())
                    .map(|c| c.tree())
                    .and_then(|tree| db.try_find(tree, &mut buf2).ok().flatten())
                    .and_then(|(tree, _)| tree.try_into_tree_iter())
            };

            let mut recorder = git_diff::tree::Recorder::default();
            git_diff::tree::Changes::from(previous_tree).needed_to_obtain(
                current_tree,
                &mut git_diff::tree::State::default(),
                |oid, buf| {
                    use git_odb::pack::FindExt;
                    db.find(oid, buf)
                        .map(|(obj, _)| obj.try_into_tree_iter().expect("only called for trees"))
                },
                &mut recorder,
            )?;
            Ok(recorder.records)
        }

        fn head_of(db: &git_odb::Handle) -> ObjectId {
            ObjectId::from_hex(
                std::fs::read(
                    db.store_ref()
                        .path()
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

        fn all_commits(db: &git_odb::Handle) -> Vec<ObjectId> {
            use git_traverse::commit;

            let head = head_of(db);
            commit::Ancestors::new(Some(head), commit::ancestors::State::default(), |oid, buf| {
                use git_odb::FindExt;
                db.find_commit_iter(oid, buf)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Result<Vec<_>, _>>()
            .expect("valid iteration")
        }

        #[test]
        fn many_different_states() -> crate::Result {
            let db = db(None)?;
            let all_commits = all_commits(&db);
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[0])?,
                vec![Addition {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                    path: "f".into()
                }],
                ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A      f"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[1])?,
                vec![Modification {
                    previous_entry_mode: EntryMode::Blob,
                    previous_oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                }],
                ":100644 100644 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 28ce6a8b26aa170e1de65536fe8abe1832bd3242 M      f"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[2])?,
                vec![Deletion {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                }],
                ":100644 000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 0000000000000000000000000000000000000000 D	f
"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[4])?,
                vec![
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                        path: "f".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("10f2f4b82222d2b5c31985130979a91fd87410f7"),
                        path: "f".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                        path: "f/f".into()
                    }
                ],
                ":100644 000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 0000000000000000000000000000000000000000 D      f
                   :000000 100644 0000000000000000000000000000000000000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 A      f/f"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[5])?,
                vec![
                    Modification {
                        previous_entry_mode: EntryMode::Tree,
                        previous_oid: hex_to_id("10f2f4b82222d2b5c31985130979a91fd87410f7"),
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("ebbe0b3000afdfd1aed15000094b59a2800328eb"),
                        path: "f".into()
                    },
                    Modification {
                        previous_entry_mode: EntryMode::Blob,
                        previous_oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("13c2aca72ab576cb5f22dc8e7f8ba8ddab553a8a"),
                        path: "f/f".into()
                    },
                ],
                ":100644 100644 28ce6a8b26aa170e1de65536fe8abe1832bd3242 13c2aca72ab576cb5f22dc8e7f8ba8ddab553a8a M	f/f"
            );

            #[cfg(windows)]
            let tree_with_link_id = hex_to_id("3b287f8730c81d0b763c2d294618a5e32b67b4f8");
            #[cfg(windows)]
            let link_entry_oid = hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391");
            #[cfg(windows)]
            let link_entry_mode = EntryMode::Blob;
            #[cfg(not(windows))]
            let tree_with_link_id = hex_to_id("7e26dba59b6336f87d1d4ae3505a2da302b91c76");
            #[cfg(not(windows))]
            let link_entry_oid = hex_to_id("2e65efe2a145dda7ee51d1741299f848e5bf752e");
            #[cfg(not(windows))]
            let link_entry_mode = EntryMode::Link;
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[8])?,
                vec![
                    Modification {
                        previous_entry_mode: EntryMode::Tree,
                        previous_oid: hex_to_id("849bd76db90b65ebbd2e6d3970ca70c96ee5592c"),
                        entry_mode: EntryMode::Tree,
                        oid: tree_with_link_id,
                        path: "f".into()
                    },
                    Modification {
                        previous_entry_mode: EntryMode::Blob,
                        previous_oid: hex_to_id("13c2aca72ab576cb5f22dc8e7f8ba8ddab553a8a"),
                        entry_mode: link_entry_mode,
                        oid: link_entry_oid,
                        path: "f/f".into()
                    },
                ],
                ":100644 120000 13c2aca72ab576cb5f22dc8e7f8ba8ddab553a8a 2e65efe2a145dda7ee51d1741299f848e5bf752e T	f/f"
            );

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[10])?,
                vec![
                    Deletion {
                        entry_mode: EntryMode::Tree,
                        oid: tree_with_link_id,
                        path: "f".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f".into()
                    },
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f/a".into()
                    },
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f/b".into()
                    },
                    Deletion {
                        entry_mode: link_entry_mode,
                        oid: link_entry_oid,
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
                    Deletion {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("3d5a503f4062d198b443db5065ca727f8354e7df"),
                        path: "d".into()
                    },
                    Deletion {
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
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "c".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "d".into()
                    },
                    Addition {
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
                    Addition {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"),
                        path: "g".into()
                    },
                    Addition {
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
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "c".into()
                    },
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "d".into()
                    },
                    Deletion {
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
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f".into()
                    },
                    Addition {
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
                    Modification {
                        previous_entry_mode: EntryMode::Tree,
                        previous_oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"),
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("6e5931346904b020301f74f581142826eacc4678"),
                        path: "g".into()
                    },
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "g/a".into()
                    },
                    Addition {
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
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "f".into()
                    },
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "ff".into()
                    },
                ],
                ":100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	f
                  :000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	ff"
            );
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[19])?,
                vec![
                    Modification {
                        previous_entry_mode: EntryMode::Tree,
                        previous_oid: hex_to_id("6e5931346904b020301f74f581142826eacc4678"),
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"),
                        path: "g".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "g/a".into()
                    },
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "g/aa".into()
                    },
                ],
                ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A	g/a
                :100644 000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 0000000000000000000000000000000000000000 D	g/aa"
            );
            Ok(())
        }

        #[test]
        fn many_different_states_nested() -> crate::Result {
            let db = db(["a"].iter().copied())?;
            let all_commits = all_commits(&db);

            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[0])?,
                vec![
                    Addition {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("3d5a503f4062d198b443db5065ca727f8354e7df"),
                        path: "a".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "a/f".into()
                    }
                ],
                ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A      a/f"
            );
            assert_eq!(
                diff_with_previous_commit_from(&db, &all_commits[1])?,
                vec![
                    Modification {
                        previous_entry_mode: EntryMode::Tree,
                        previous_oid: hex_to_id("3d5a503f4062d198b443db5065ca727f8354e7df"),
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("10f2f4b82222d2b5c31985130979a91fd87410f7"),
                        path: "a".into()
                    },
                    Modification {
                        previous_entry_mode: EntryMode::Blob,
                        previous_oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                        path: "a/f".into()
                    }
                ],
                ":100644 100644 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 28ce6a8b26aa170e1de65536fe8abe1832bd3242 M      a/f"
            );

            for commit in all_commits {
                // Just make sure it works - checked results with dbg!() once and am too ~~lazy~~ time constrained to add the
                // assertions now similar to the non-nested version.
                diff_with_previous_commit_from(&db, &commit)?;
            }

            Ok(())
        }

        #[test]
        fn maximal_difference() -> crate::Result {
            let db = db(None)?;
            let all_commits = all_commits(&db);

            assert_eq!(
                diff_commits(
                    &db,
                    all_commits[0].to_owned(),
                    all_commits.last().expect("we have many commits")
                )?,
                vec![
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "b".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"),
                        path: "g".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "g/a".into()
                    }
                ]
            );
            assert_eq!(
                diff_commits(
                    &db,
                    all_commits.last().expect("we have many commits").to_owned(),
                    &all_commits[0]
                )?,
                vec![
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "b".into()
                    },
                    Deletion {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"),
                        path: "g".into()
                    },
                    Deletion {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "g/a".into()
                    }
                ]
            );
            Ok(())
        }

        #[test]
        fn maximal_difference_nested() -> crate::Result {
            let db = db(["a"].iter().copied())?;
            let all_commits = all_commits(&db);

            assert_eq!(
                diff_commits(&db, None::<ObjectId>, &all_commits[all_commits.len() - 6])?,
                vec![
                    Addition {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("0df4d0ed769eacd0a231e7512fca25d3cabdeca4"),
                        path: "a".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "a/b".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "a/c".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "a/d".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "a/e".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "a/f".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Tree,
                        oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba"),
                        path: "a/g".into()
                    },
                    Addition {
                        entry_mode: EntryMode::Blob,
                        oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                        path: "a/g/a".into()
                    }
                ]
            );
            Ok(())
        }
    }
}
