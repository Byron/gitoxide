mod changes {
    mod to_obtain_tree {
        use crate::hex_to_id;
        use git_diff::visit::recorder;
        use git_object::tree::EntryMode;
        use git_odb::{pack, Locate};

        const FIRST_COMMIT: &str = "055df97e18cd537da3cb16bcbdf1733fdcdfb430";
        const SECOND_COMMIT: &str = "a5ebf9ee3b1cac5daf3dc9056026ee848be52da2";
        const FIFTH_COMMIT: &str = "69bbebb6608472d98be684f4e6ef1faaac2a03bc";

        fn diff_with_previous_commit_from(commit_id: &str) -> crate::Result<recorder::Changes> {
            let db = git_odb::linked::Db::at(
                test_tools::scripted_fixture_repo_read_only("make_diff_repo.sh")?
                    .join(".git")
                    .join("objects"),
            )?;
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
                .decode()?
                .into_tree()
                .expect("id to be a tree");
            let mut buf2 = Vec::new();
            let previous_tree: Option<_> = {
                parent_commit_id
                    .and_then(|id| db.locate(id, &mut buf2, &mut pack::cache::Never).ok().flatten())
                    .and_then(|c| c.decode().ok())
                    .and_then(|c| c.into_commit())
                    .map(|c| c.tree())
                    .and_then(|tree| db.locate(tree, &mut buf2, &mut pack::cache::Never).ok().flatten())
                    .and_then(|tree| tree.decode().ok())
                    .and_then(|tree| tree.into_tree())
            };

            let mut recorder = git_diff::visit::Recorder::default();
            git_diff::visit::Changes::from(previous_tree.as_ref()).to_obtain_tree(
                &main_tree,
                &mut git_diff::visit::State::default(),
                |_oid, _buf| todo!("Actual lookup in db"),
                &mut recorder,
            )?;
            Ok(recorder.records)
        }

        #[test]
        fn many_different_states() {
            assert_eq!(
                diff_with_previous_commit_from(FIRST_COMMIT).unwrap(),
                vec![recorder::Change::Addition {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                    path: "f".into()
                }]
                , ":000000 100644 0000000000000000000000000000000000000000 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 A      f");
            assert_eq!(
                diff_with_previous_commit_from(SECOND_COMMIT).unwrap(),
                vec![recorder::Change::Modification {
                    previous_entry_mode: EntryMode::Blob,
                    previous_oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                }]
                , ":100644 100644 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391 28ce6a8b26aa170e1de65536fe8abe1832bd3242 M      f");
            assert_eq!(
                diff_with_previous_commit_from(FIFTH_COMMIT).unwrap(),
                vec![recorder::Change::Deletion {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f".into()
                },
                recorder::Change::Addition {
                    entry_mode: EntryMode::Blob,
                    oid: hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242"),
                    path: "f/f".into()
                }]
                , ":100644 000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 0000000000000000000000000000000000000000 D      f
                   :000000 100644 0000000000000000000000000000000000000000 28ce6a8b26aa170e1de65536fe8abe1832bd3242 A      f/f");
        }
    }
}
