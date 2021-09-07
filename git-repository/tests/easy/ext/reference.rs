mod set_namespace {
    use git_repository as git;
    use git_repository::{prelude::ReferenceAccessExt, refs::transaction::PreviousValue};

    fn easy_repo_rw() -> crate::Result<(git::EasyArcExclusive, tempfile::TempDir)> {
        crate::repo_rw("make_references_repo.sh").map(|(r, d)| (r.into_easy_arc_exclusive(), d))
    }

    #[test]
    fn affects_edits_and_iteration() {
        let (mut repo, _keep) = easy_repo_rw().unwrap();
        assert_eq!(
            repo.references().unwrap().all().unwrap().count(),
            15,
            "there are plenty of references in the default namespace"
        );
        assert!(repo.namespace().unwrap().is_none(), "no namespace is set initially");
        assert!(
            repo.set_namespace("foo").unwrap().is_none(),
            "there is no previous namespace"
        );

        assert_eq!(
            repo.references().unwrap().all().unwrap().filter_map(Result::ok).count(),
            0,
            "no references are in the namespace yet"
        );

        repo.tag(
            "new-tag",
            git::hash::ObjectId::empty_tree(git::hash::Kind::Sha1),
            PreviousValue::MustNotExist,
        )
        .unwrap();

        repo.reference(
            "refs/heads/new-branch",
            git::hash::ObjectId::empty_tree(git::hash::Kind::Sha1),
            PreviousValue::MustNotExist,
            "message",
        )
        .unwrap();

        assert_eq!(
            repo.references()
                .unwrap()
                .all()
                .unwrap()
                .filter_map(Result::ok)
                .map(|r| r.name().as_bstr().to_owned())
                .collect::<Vec<_>>(),
            vec!["refs/heads/new-branch", "refs/tags/new-tag"],
            "namespaced references appear like normal ones"
        );

        assert_eq!(
            repo.references()
                .unwrap()
                .prefixed("refs/tags/")
                .unwrap()
                .filter_map(Result::ok)
                .map(|r| r.name().as_bstr().to_owned())
                .collect::<Vec<_>>(),
            vec!["refs/tags/new-tag"],
            "namespaced references appear like normal ones"
        );
        let fully_qualified_tag_name = "refs/tags/new-tag";
        assert_eq!(
            repo.find_reference(fully_qualified_tag_name).unwrap().name().as_bstr(),
            fully_qualified_tag_name,
            "fully qualified (yet namespaced) names work"
        );
        assert_eq!(
            repo.find_reference("new-tag").unwrap().name().as_bstr(),
            fully_qualified_tag_name,
            "namespaces are transparent"
        );

        let previous_ns = repo.clear_namespace().unwrap().expect("namespace set");
        assert_eq!(previous_ns.as_bstr(), "refs/namespaces/foo/");
        assert!(
            repo.clear_namespace().unwrap().is_none(),
            "it doesn't invent namespaces"
        );

        assert_eq!(
            repo.references().unwrap().all().unwrap().count(),
            17,
            "it lists all references, also the ones in namespaces"
        );
    }
}

mod iter_references {
    use git_repository as git;
    use git_repository::prelude::ReferenceAccessExt;

    fn repo() -> crate::Result<git::Easy> {
        crate::repo("make_references_repo.sh").map(|r| r.into_easy())
    }

    #[test]
    fn all() -> crate::Result {
        let repo = repo()?;
        assert_eq!(
            repo.references()?
                .all()?
                .filter_map(Result::ok)
                .map(|r| r.name().as_bstr().to_owned())
                .collect::<Vec<_>>(),
            vec![
                "refs/d1",
                "refs/heads/d1",
                "refs/heads/dt1",
                "refs/heads/main",
                "refs/heads/multi-link-target1",
                "refs/loop-a",
                "refs/loop-b",
                "refs/multi-link",
                "refs/remotes/origin/HEAD",
                "refs/remotes/origin/main",
                "refs/remotes/origin/multi-link-target3",
                "refs/tags/dt1",
                "refs/tags/multi-link-target2",
                "refs/tags/t1"
            ]
        );
        Ok(())
    }

    #[test]
    fn prefixed() -> crate::Result {
        let repo = repo()?;
        assert_eq!(
            repo.references()?
                .prefixed("refs/heads/")?
                .filter_map(Result::ok)
                .map(|r| r.name().as_bstr().to_owned())
                .collect::<Vec<_>>(),
            vec![
                "refs/heads/d1",
                "refs/heads/dt1",
                "refs/heads/main",
                "refs/heads/multi-link-target1",
            ]
        );
        Ok(())
    }
}

mod head {

    use git_ref::transaction::PreviousValue;
    use git_repository as git;
    use git_repository::prelude::ReferenceAccessExt;
    use git_testtools::hex_to_id;

    #[test]
    fn symbolic() -> crate::Result {
        let repo = crate::basic_repo()?;
        let head = repo.head()?;
        match &head.kind {
            git::easy::head::Kind::Symbolic(r) => {
                assert_eq!(
                    r.target.as_id().map(ToOwned::to_owned),
                    Some(hex_to_id("3189cd3cb0af8586c39a838aa3e54fd72a872a41"))
                );
            }
            _ => panic!("unexpected head kind"),
        }
        assert_eq!(head.name().expect("born").as_bstr(), "refs/heads/main");
        assert!(!head.is_detached());
        Ok(())
    }

    #[test]
    fn detached() -> crate::Result {
        let (repo, _keep) = crate::basic_rw_repo()?;
        repo.reference(
            "HEAD",
            hex_to_id("3189cd3cb0af8586c39a838aa3e54fd72a872a41"),
            PreviousValue::Any,
            "",
        )?;

        let head = repo.head()?;
        assert!(head.is_detached(), "head is detached");
        assert!(head.name().is_none());
        Ok(())
    }
}
