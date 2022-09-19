mod diff {
    use crate::remote;
    use git_object::bstr::ByteSlice;
    use git_object::tree::EntryMode;
    use git_repository as git;
    use git_repository::object::tree::diff::Event;
    use std::convert::Infallible;

    #[test]
    fn changes_against_tree_modified() {
        let repo = remote::repo("base");
        let from = tree_named(&repo, "g");
        let to = tree_named(&repo, "h");
        from.changes()
            .for_each_to_obtain_tree(&to, |event| -> Result<_, Infallible> {
                match event {
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
                        Ok(git::diff::tree::visit::Action::Continue)
                    }
                    Event::Deletion { .. } | Event::Addition { .. } => unreachable!("only modification is expected"),
                }
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
