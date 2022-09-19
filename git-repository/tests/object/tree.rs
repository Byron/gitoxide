mod diff {
    use crate::remote;
    use git_object::bstr::ByteSlice;
    use git_object::tree::EntryMode;
    use git_repository as git;
    use git_repository::object::tree::diff::change::Event;
    use std::convert::Infallible;

    #[test]
    fn changes_against_tree_modified() {
        let repo = remote::repo("base");
        let from = tree_named(&repo, "g");
        let to = tree_named(&repo, "h");
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
                        assert_eq!(previous_id.object().unwrap().data.as_bstr(), "g\n");
                        assert_eq!(id.object().unwrap().data.as_bstr(), "h\n");
                        Ok(Default::default())
                    }
                    Event::Deletion { .. } | Event::Addition { .. } => unreachable!("only modification is expected"),
                }
            })
            .unwrap();

        from.changes()
            .track_filename()
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                assert_eq!(change.location, "file");
                Ok(git::object::tree::diff::Action::Continue)
            })
            .unwrap();
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
