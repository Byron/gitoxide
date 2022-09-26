mod update {

    use crate as git;

    fn repo(name: &str) -> git::Repository {
        let dir = git_testtools::scripted_fixture_repo_read_only_with_args(
            "make_fetch_repos.sh",
            [git::path::realpath(
                git_testtools::scripted_fixture_repo_read_only("make_remote_repos.sh")
                    .unwrap()
                    .join("base"),
            )
            .unwrap()
            .to_string_lossy()],
        )
        .unwrap();
        git::open_opts(dir.join(name), git::open::Options::isolated()).unwrap()
    }

    use crate::remote::fetch;
    use git_ref::TargetRef;

    #[test]
    fn various_valid_updates() {
        let repo = repo("two-origins");
        // TODO: test reflog message (various cases if it's new)
        for (spec, expected_mode, has_edit_index, detail) in [
            (
                "refs/heads/main:refs/remotes/origin/main",
                fetch::refs::update::Mode::NoChangeNeeded,
                true,
                "these refs are en-par since the initial clone",
            ),
            (
                "refs/heads/main",
                fetch::refs::update::Mode::NoChangeNeeded,
                false,
                "without local destination ref there is nothing to do for us, ever (except for FETCH_HEADs) later",
            ),
            (
                "refs/heads/main:refs/remotes/origin/new-main",
                fetch::refs::update::Mode::New,
                true,
                "the destination branch doesn't exist and needs to be created",
            ),
            (
                "+refs/heads/main:refs/remotes/origin/g",
                fetch::refs::update::Mode::Forced,
                true,
                "a forced non-fastforward (main goes backwards)",
            ),
            // (
            //     "refs/remotes/origin/g:refs/heads/not-currently-checked-out",
            //     fetch::refs::update::Mode::FastForward,
            //     true,
            //     "a fast-forward only fast-forward situation, all good",
            // ),
        ] {
            let (mapping, specs) = mapping_from_spec(spec, &repo);
            let out = fetch::refs::update(&repo, &mapping, &specs, fetch::DryRun::Yes).unwrap();

            assert_eq!(
                out.updates,
                vec![fetch::refs::Update {
                    mode: expected_mode,
                    edit_index: has_edit_index.then(|| 0),
                    spec_index: 0
                }],
                "{spec:?}: {detail}"
            );
            assert_eq!(out.edits.len(), has_edit_index.then(|| 1).unwrap_or(0));
        }
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn fast_forward_is_not_implemented_yet() {
        // TODO: move it above for acceptable case, test here for non-fastforwards being denied.
        let repo = repo("two-origins");
        let (mappings, specs) = mapping_from_spec("+refs/heads/main:refs/heads/g", &repo);
        let out = fetch::refs::update(&repo, &mappings, &specs, fetch::DryRun::Yes).unwrap();

        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::FastForward,
                edit_index: Some(0),
                spec_index: 0,
            }]
        );
        assert_eq!(out.edits.len(), 1);
    }

    fn mapping_from_spec(spec: &str, repo: &git::Repository) -> (Vec<fetch::Mapping>, Vec<git::refspec::RefSpec>) {
        let spec = git_refspec::parse(spec.into(), git_refspec::parse::Operation::Fetch).unwrap();
        let group = git_refspec::MatchGroup::from_fetch_specs(Some(spec));
        let references = repo.references().unwrap();
        let references: Vec<_> = references.all().unwrap().map(|r| into_remote_ref(r.unwrap())).collect();
        let mappings = group
            .match_remotes(references.iter().map(remote_ref_to_item))
            .mappings
            .into_iter()
            .map(|m| fetch::Mapping {
                remote: fetch::Source::Ref(
                    references[m.item_index.expect("set as all items are backed by ref")].clone(),
                ),
                local: m.rhs.map(|r| r.into_owned()),
                spec_index: m.spec_index,
            })
            .collect();
        (mappings, vec![spec.to_owned()])
    }

    fn into_remote_ref(mut r: git::Reference<'_>) -> git_protocol::fetch::Ref {
        let full_ref_name = r.name().as_bstr().into();
        match r.target() {
            TargetRef::Peeled(id) => git_protocol::fetch::Ref::Direct {
                full_ref_name,
                object: id.into(),
            },
            TargetRef::Symbolic(name) => {
                let target = name.as_bstr().into();
                let id = r.peel_to_id_in_place().unwrap();
                git_protocol::fetch::Ref::Symbolic {
                    full_ref_name,
                    target,
                    object: id.detach(),
                }
            }
        }
    }

    fn remote_ref_to_item(r: &git_protocol::fetch::Ref) -> git_refspec::match_group::Item<'_> {
        let (full_ref_name, target, object) = r.unpack();
        git_refspec::match_group::Item {
            full_ref_name,
            target,
            object,
        }
    }
}
