mod diff {
    use std::convert::Infallible;

    use git_object::{bstr::ByteSlice, tree::EntryMode};
    use git_repository as git;
    use git_repository::object::tree::diff::change::Event;

    use crate::named_repo;

    #[test]
    fn changes_against_tree_modified() {
        let repo = named_repo("make_diff_repo.sh").unwrap();
        let from = tree_named(&repo, "@^{/c3}~1");
        let to = tree_named(&repo, ":/c3");
        from.changes()
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                assert_eq!(change.location, "", "without configuration the location field is empty");
                match change.event {
                    Event::Modification {
                        previous_entry_mode,
                        previous_id,
                        entry_mode,
                        id,
                    } => {
                        assert_eq!(previous_entry_mode, EntryMode::Blob);
                        assert_eq!(entry_mode, EntryMode::Blob);
                        assert_eq!(previous_id.object().unwrap().data.as_bstr(), "a\n");
                        assert_eq!(id.object().unwrap().data.as_bstr(), "a\na1\n");
                    }
                    Event::Deletion { .. } | Event::Addition { .. } => unreachable!("only modification is expected"),
                };

                let count = change
                    .event
                    .diff()
                    .expect("changed file")
                    .expect("objects available")
                    .lines(git::diff::text::Algorithm::Myers, |_| {
                        git_diff::text::imara::sink::Counter::default()
                    });
                assert_eq!(count.insertions, 1);
                assert_eq!(count.removals, 0);
                Ok(Default::default())
            })
            .unwrap();
    }

    #[test]
    fn changes_against_tree_with_filename_tracking() {
        let repo = named_repo("make_diff_repo.sh").unwrap();
        let from = tree_named(
            &repo,
            &git::hash::ObjectId::empty_tree(git::hash::Kind::Sha1).to_string(),
        );
        let to = tree_named(&repo, ":/c1");

        let mut expected = vec!["a", "b", "c", "d"];
        from.changes()
            .track_filename()
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                expected.retain(|name| name != change.location);
                Ok(Default::default())
            })
            .unwrap();
        assert_eq!(expected, Vec::<&str>::new(), "all paths should have been seen");

        let mut expected = vec!["a", "b", "dir/c", "d"];
        from.changes()
            .track_path()
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                expected.retain(|name| name != change.location);
                Ok(Default::default())
            })
            .unwrap();
        assert_eq!(expected, Vec::<&str>::new(), "all paths should have been seen");
    }

    fn tree_named<'repo>(repo: &'repo git::Repository, rev_spec: &str) -> git::Tree<'repo> {
        repo.rev_parse_single(rev_spec)
            .unwrap()
            .object()
            .unwrap()
            .peel_to_kind(git::object::Kind::Tree)
            .unwrap()
            .into_tree()
    }
}
