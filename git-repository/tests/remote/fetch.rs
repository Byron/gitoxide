#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use crate::remote;
    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::fetch;
    use git_repository::remote::Direction::Fetch;
    use git_testtools::hex_to_id;
    use std::sync::atomic::AtomicBool;

    fn repo_rw(name: &str) -> (git::Repository, git_testtools::tempfile::TempDir) {
        let dir = git_testtools::scripted_fixture_repo_writable_with_args(
            "make_fetch_repos.sh",
            [git::path::realpath(remote::repo_path("base"))
                .unwrap()
                .to_string_lossy()],
            git_testtools::Creation::ExecuteScript,
        )
        .unwrap();
        let repo = git::open_opts(dir.path().join(name), git::open::Options::isolated()).unwrap();
        (repo, dir)
    }

    mod refs {
        use crate::remote;
        use git_repository as git;

        fn repo(name: &str) -> git::Repository {
            let dir = git_testtools::scripted_fixture_repo_read_only_with_args(
                "make_fetch_repos.sh",
                [git::path::realpath(remote::repo_path("base"))
                    .unwrap()
                    .to_string_lossy()],
            )
            .unwrap();
            git::open_opts(dir.join(name), git::open::Options::isolated()).unwrap()
        }

        mod update {
            use crate::remote::fetch::blocking_io::refs::repo;
            use git_ref::TargetRef;
            use git_repository as git;
            use git_repository::remote::fetch;

            #[test]
            #[ignore]
            fn various_valid_updates() {
                let repo = repo("two-origins");
                // TODO: test reflog message (various cases if it's new)
                for (spec, expected_mode, detail) in [
                    (
                        "refs/heads/main:refs/remotes/origin/main",
                        fetch::refs::update::Mode::NoChangeNeeded,
                        "these refs are en-par since the initial clone"
                    ),
                    (
                        "refs/heads/main",
                        fetch::refs::update::Mode::NoChangeNeeded,
                        "without local destination ref there is nothing to do for us, ever (except for FETCH_HEADs) later"
                    ),
                    (
                        "refs/heads/main:refs/remotes/origin/new-main",
                        fetch::refs::update::Mode::New,
                        "the destination branch doesn't exist and needs to be created"
                    ),
                    ("+refs/heads/main:refs/heads/g", fetch::refs::update::Mode::Forced, "a forced non-fastforward (main goes backwards)"),
                    // ("refs/heads/g:refs/heads/main", fetch::refs::update::Mode::FastForward, "a fast-forward only fast-forward situation, all good"),
                ] {
                    let (mapping, specs) = mapping_from_spec(spec, &repo);
                    let out = fetch::refs::update(&repo, &mapping, &specs, fetch::DryRun::Yes).unwrap();

                    assert_eq!(
                        out.updates,
                        vec![fetch::refs::Update {
                            mode: expected_mode,
                            edit_index: Some(0),
                            spec_index: 0
                        }],
                        "{spec:?}: {detail}"
                    );
                    assert_eq!(out.edits.len(), 1);
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

            fn mapping_from_spec(
                spec: &str,
                repo: &git::Repository,
            ) -> (Vec<fetch::Mapping>, Vec<git::refspec::RefSpec>) {
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
    }

    #[test]
    fn fetch_pack() -> crate::Result {
        for version in [
            None,
            Some(git::protocol::transport::Protocol::V2),
            Some(git::protocol::transport::Protocol::V1),
        ] {
            let (mut repo, _tmp) = repo_rw("two-origins");
            if let Some(version) = version {
                repo.config_snapshot_mut().set_raw_value(
                    "protocol",
                    None,
                    "version",
                    (version as u8).to_string().as_str(),
                )?;
            }

            // No updates
            {
                let remote = repo.find_remote("origin")?;
                {
                    remote
                        .connect(Fetch, progress::Discard)?
                        .prepare_fetch(Default::default())?;
                    // early drops are fine and won't block.
                }
                let outcome = remote
                    .connect(Fetch, progress::Discard)?
                    .prepare_fetch(Default::default())?
                    .receive(&AtomicBool::default())?;
                assert!(matches!(outcome.status, git::remote::fetch::Status::NoChange));
            }

            // Some updates to be fetched
            {
                let remote = repo.find_remote("changes-on-top-of-origin")?;
                let outcome: git::remote::fetch::Outcome = remote
                    .connect(Fetch, progress::Discard)?
                    .prepare_fetch(Default::default())?
                    .receive(&AtomicBool::default())?;
                match outcome.status {
                    fetch::Status::Change {
                        write_pack_bundle,
                        update_refs: _, // TODO: validate update refs
                    } => {
                        assert_eq!(write_pack_bundle.pack_kind, git::odb::pack::data::Version::V2);
                        assert_eq!(write_pack_bundle.object_hash, repo.object_hash());
                        assert_eq!(write_pack_bundle.index.num_objects, 3, "this value is 4 when git does it with 'consecutive' negotiation style, but could be 33 if completely naive.");
                        assert_eq!(
                            write_pack_bundle.index.index_version,
                            git::odb::pack::index::Version::V2
                        );
                        assert_eq!(
                            write_pack_bundle.index.index_hash,
                            hex_to_id("c75114f60ab2c9389916f3de1082bbaa47491e3b")
                        );
                        assert!(write_pack_bundle.data_path.map_or(false, |f| f.is_file()));
                        assert!(write_pack_bundle.index_path.map_or(false, |f| f.is_file()));
                    }
                    fetch::Status::NoChange => unreachable!("we firmly expect changes here"),
                }
            }
        }
        Ok(())
    }
}
