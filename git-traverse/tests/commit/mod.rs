mod ancestor {
    use git_hash::{oid, ObjectId};
    use git_odb::linked::Store;
    use git_odb::{pack, FindExt};
    use git_traverse::commit;

    use crate::hex_to_id;

    fn db() -> crate::Result<Store> {
        let dir = git_testtools::scripted_fixture_repo_read_only("make_traversal_repo_for_commits.sh")?;
        let db = Store::at(dir.join(".git").join("objects"))?;
        Ok(db)
    }

    fn filtered_iter(
        tips: impl IntoIterator<Item = impl Into<ObjectId>>,
        predicate: impl FnMut(&oid) -> bool,
    ) -> impl Iterator<Item = Result<ObjectId, commit::ancestors::Error>> {
        let db = db().expect("db instantiation works as its definitely valid");
        commit::Ancestors::filtered(
            tips,
            commit::ancestors::State::default(),
            move |oid, buf| db.find_existing_commit_iter(oid, buf, &mut pack::cache::Never).ok(),
            predicate,
        )
    }

    fn check_filtered_traversal_with_shared_reference(
        tips: &[&str],
        expected: &[&str],
        predicate: impl FnMut(&oid) -> bool,
    ) -> crate::Result {
        let tips: Vec<_> = tips.iter().copied().map(hex_to_id).collect();
        let oids: Result<Vec<_>, _> = filtered_iter(tips.iter().cloned(), predicate).collect();
        let expected: Vec<_> = tips
            .into_iter()
            .chain(expected.iter().map(|hex_id| hex_to_id(hex_id)))
            .collect();
        assert_eq!(oids?, expected);
        Ok(())
    }

    fn new_iter(
        tips: impl IntoIterator<Item = impl Into<ObjectId>>,
    ) -> impl Iterator<Item = Result<ObjectId, commit::ancestors::Error>> {
        let db = db().expect("db instantiation works as its definitely valid");
        commit::Ancestors::new(tips, commit::ancestors::State::default(), move |oid, buf| {
            db.find_existing_commit_iter(oid, buf, &mut pack::cache::Never).ok()
        })
    }

    fn check_traversal_with_shared_reference(tips: &[&str], expected: &[&str]) -> crate::Result {
        let tips: Vec<_> = tips.iter().copied().map(hex_to_id).collect();
        let oids: Result<Vec<_>, _> = new_iter(tips.iter().cloned()).collect();
        let expected: Vec<_> = tips
            .into_iter()
            .chain(expected.iter().map(|hex_id| hex_to_id(hex_id)))
            .collect();
        assert_eq!(oids?, expected);
        Ok(())
    }

    #[test]
    fn instantiate_with_arc() -> crate::Result {
        let _ = new_iter(vec![git_hash::ObjectId::null_sha1()]);
        Ok(())
    }

    #[test]
    fn instantiate_with_box() -> crate::Result {
        let _ = new_iter(vec![git_hash::ObjectId::null_sha1()]);
        Ok(())
    }

    #[test]
    fn linear_history_no_branch() -> crate::Result {
        check_traversal_with_shared_reference(
            &["9556057aee5abb06912922e9f26c46386a816822"],
            &[
                "17d78c64cef6c33a10a604573fd2c429e477fd63",
                "9902e3c3e8f0c569b4ab295ddf473e6de763e1e7",
                "134385f6d781b7e97062102c6a483440bfda2a03",
            ],
        )
    }

    #[test]
    fn simple_branch_with_merge() -> crate::Result {
        check_traversal_with_shared_reference(
            &["01ec18a3ebf2855708ad3c9d244306bc1fae3e9b"],
            &[
                "efd9a841189668f1bab5b8ebade9cd0a1b139a37",
                "ce2e8ffaa9608a26f7b21afc1db89cadb54fd353",
                "9556057aee5abb06912922e9f26c46386a816822",
                "9152eeee2328073cf23dcf8e90c949170b711659",
                "17d78c64cef6c33a10a604573fd2c429e477fd63",
                "9902e3c3e8f0c569b4ab295ddf473e6de763e1e7",
                "134385f6d781b7e97062102c6a483440bfda2a03",
            ],
        )
    }

    #[test]
    fn multiple_tips() -> crate::Result {
        check_traversal_with_shared_reference(
            &[
                "01ec18a3ebf2855708ad3c9d244306bc1fae3e9b",
                "9556057aee5abb06912922e9f26c46386a816822",
            ],
            &[
                "efd9a841189668f1bab5b8ebade9cd0a1b139a37",
                "ce2e8ffaa9608a26f7b21afc1db89cadb54fd353",
                "17d78c64cef6c33a10a604573fd2c429e477fd63",
                "9152eeee2328073cf23dcf8e90c949170b711659",
                "9902e3c3e8f0c569b4ab295ddf473e6de763e1e7",
                "134385f6d781b7e97062102c6a483440bfda2a03",
            ],
        )
    }

    #[test]
    fn filtered_commit_does_not_block_ancestors_reachable_from_another_commit() -> crate::Result {
        // I don't see a use case for the predicate returning false for a commit but return true for
        // at least one of its ancestors, so this test is kind of dubious. But we do want
        // `Ancestors` to not eagerly blacklist all of a commit's ancestors when blacklisting that
        // one commit, and this test happens to check that.
        check_filtered_traversal_with_shared_reference(
            &["01ec18a3ebf2855708ad3c9d244306bc1fae3e9b"],
            &[
                "efd9a841189668f1bab5b8ebade9cd0a1b139a37",
                "ce2e8ffaa9608a26f7b21afc1db89cadb54fd353",
                "9556057aee5abb06912922e9f26c46386a816822",
                "17d78c64cef6c33a10a604573fd2c429e477fd63",
                "9902e3c3e8f0c569b4ab295ddf473e6de763e1e7",
                "134385f6d781b7e97062102c6a483440bfda2a03",
            ],
            |id| id != hex_to_id("9152eeee2328073cf23dcf8e90c949170b711659"),
        )
    }

    #[test]
    fn predicate_only_called_once_even_if_fork_point() -> crate::Result {
        // The `self.seen` check should come before the `self.predicate` check, as we don't know how
        // expensive calling `self.predicate` may be.
        let mut seen = false;
        check_filtered_traversal_with_shared_reference(
            &["01ec18a3ebf2855708ad3c9d244306bc1fae3e9b"],
            &[
                "efd9a841189668f1bab5b8ebade9cd0a1b139a37",
                "ce2e8ffaa9608a26f7b21afc1db89cadb54fd353",
                "9152eeee2328073cf23dcf8e90c949170b711659",
            ],
            move |id| {
                if id == hex_to_id("9556057aee5abb06912922e9f26c46386a816822") {
                    assert_eq!(seen, false);
                    seen = true;
                    false
                } else {
                    true
                }
            },
        )
    }
}
