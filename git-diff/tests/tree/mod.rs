mod with_tree {
    use git_odb::{pack, Locate};
    const FIRST_COMMIT: &str = "055df97e18cd537da3cb16bcbdf1733fdcdfb430";

    fn diff_at(commit_id: &str) -> crate::Result {
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

        git_diff::visit::Changes::from(previous_tree.as_ref()).to_obtain_tree(
            &main_tree,
            &mut git_diff::visit::State::default(),
            |_oid, _buf| todo!("Actual lookup in db"),
            &mut git_diff::visit::Recorder::default(),
        )?;
        Ok(())
    }
    #[test]
    #[should_panic]
    fn file_added() {
        diff_at(FIRST_COMMIT).unwrap();
        todo!("detect an added file in the root tree")
    }
}
