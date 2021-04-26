mod with_tree {
    use git_odb::{pack, Locate};

    fn diff_at(commit_id: &str) -> crate::Result {
        let db = git_odb::linked::Db::at(test_tools::scripted_fixture_repo_read_only("make_diff_repo.sh")?)?;
        let commit_id = git_hash::ObjectId::from_hex(commit_id.as_bytes())?;
        let mut buf = Vec::new();
        let (_main_tree_id, _parent_commit_id) = {
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

        Ok(())
    }
    #[test]
    #[should_panic]
    fn file_added() {
        todo!("detect an added file in the root tree")
    }
}
