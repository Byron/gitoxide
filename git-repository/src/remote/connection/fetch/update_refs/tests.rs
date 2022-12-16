mod update {
    use std::convert::TryInto;

    use git_testtools::{hex_to_id, Result};

    use crate as git;

    fn base_repo_path() -> String {
        git::path::realpath(
            git_testtools::scripted_fixture_read_only("make_remote_repos.sh")
                .unwrap()
                .join("base"),
        )
        .unwrap()
        .to_string_lossy()
        .into_owned()
    }

    fn repo(name: &str) -> git::Repository {
        let dir =
            git_testtools::scripted_fixture_read_only_with_args("make_fetch_repos.sh", [base_repo_path()]).unwrap();
        git::open_opts(dir.join(name), git::open::Options::isolated()).unwrap()
    }
    fn repo_rw(name: &str) -> (git::Repository, git_testtools::tempfile::TempDir) {
        let dir = git_testtools::scripted_fixture_writable_with_args(
            "make_fetch_repos.sh",
            [base_repo_path()],
            git_testtools::Creation::ExecuteScript,
        )
        .unwrap();
        let repo = git::open_opts(dir.path().join(name), git::open::Options::isolated()).unwrap();
        (repo, dir)
    }
    use git_ref::{transaction::Change, TargetRef};

    use crate::remote::fetch::SpecIndex;
    use crate::{
        bstr::BString,
        remote::{
            fetch,
            fetch::{Mapping, RefLogMessage, Source},
        },
    };

    #[test]
    fn various_valid_updates() {
        let repo = repo("two-origins");
        for (spec, expected_mode, reflog_message, detail) in [
            (
                "refs/heads/main:refs/remotes/origin/main",
                fetch::refs::update::Mode::NoChangeNeeded,
                Some("no update will be performed"),
                "these refs are en-par since the initial clone",
            ),
            (
                "refs/heads/main",
                fetch::refs::update::Mode::NoChangeNeeded,
                None,
                "without local destination ref there is nothing to do for us, ever (except for FETCH_HEADs) later",
            ),
            (
                "refs/heads/main:refs/remotes/origin/new-main",
                fetch::refs::update::Mode::New,
                Some("storing ref"),
                "the destination branch doesn't exist and needs to be created",
            ),
            (
                "refs/heads/main:refs/heads/feature",
                fetch::refs::update::Mode::New,
                Some("storing head"),
                "reflog messages are specific to the type of branch stored, to some limited extend",
            ),
            (
                "refs/heads/main:refs/tags/new-tag",
                fetch::refs::update::Mode::New,
                Some("storing tag"),
                "reflog messages are specific to the type of branch stored, to some limited extend",
            ),
            (
                "+refs/heads/main:refs/remotes/origin/new-main",
                fetch::refs::update::Mode::New,
                Some("storing ref"),
                "just to validate that we really are in dry-run mode, or else this ref would be present now",
            ),
            (
                "+refs/heads/main:refs/remotes/origin/g",
                fetch::refs::update::Mode::FastForward,
                Some("fast-forward (guessed in dry-run)"),
                "a forced non-fastforward (main goes backwards), but dry-run calls it fast-forward",
            ),
            (
                "+refs/heads/main:refs/tags/b-tag",
                fetch::refs::update::Mode::Forced,
                Some("updating tag"),
                "tags can only be forced",
            ),
            (
                "refs/heads/main:refs/tags/b-tag",
                fetch::refs::update::Mode::RejectedTagUpdate,
                None,
                "otherwise a tag is always refusing itself to be overwritten (no-clobber)",
            ),
            (
                "+refs/remotes/origin/g:refs/heads/main",
                fetch::refs::update::Mode::RejectedCurrentlyCheckedOut {
                    worktree_dir: repo.work_dir().expect("present").to_owned(),
                },
                None,
                "checked out branches cannot be written, as it requires a merge of sorts which isn't done here",
            ),
            (
                "ffffffffffffffffffffffffffffffffffffffff:refs/heads/invalid-source-object",
                fetch::refs::update::Mode::RejectedSourceObjectNotFound {
                    id: hex_to_id("ffffffffffffffffffffffffffffffffffffffff"),
                },
                None,
                "checked out branches cannot be written, as it requires a merge of sorts which isn't done here",
            ),
            (
                "refs/remotes/origin/g:refs/heads/not-currently-checked-out",
                fetch::refs::update::Mode::FastForward,
                Some("fast-forward (guessed in dry-run)"),
                "a fast-forward only fast-forward situation, all good",
            ),
        ] {
            let (mapping, specs) = mapping_from_spec(spec, &repo);
            let out = fetch::refs::update(
                &repo,
                prefixed("action"),
                &mapping,
                &specs,
                &[],
                fetch::Tags::None,
                reflog_message.map(|_| fetch::DryRun::Yes).unwrap_or(fetch::DryRun::No),
                fetch::WritePackedRefs::Never,
            )
            .unwrap();

            assert_eq!(
                out.updates,
                vec![fetch::refs::Update {
                    mode: expected_mode.clone(),
                    edit_index: reflog_message.map(|_| 0),
                }],
                "{spec:?}: {detail}"
            );
            assert_eq!(out.edits.len(), reflog_message.map(|_| 1).unwrap_or(0));
            if let Some(reflog_message) = reflog_message {
                let edit = &out.edits[0];
                match &edit.change {
                    Change::Update { log, new, .. } => {
                        assert_eq!(
                            log.message,
                            format!("action: {}", reflog_message),
                            "{}: reflog messages are specific and we emulate git word for word",
                            spec
                        );
                        let remote_ref = repo
                            .find_reference(specs[0].to_ref().source().expect("always present"))
                            .unwrap();
                        assert_eq!(
                            new.id(),
                            remote_ref.target().id(),
                            "remote ref provides the id to set in the local reference"
                        )
                    }
                    _ => unreachable!("only updates"),
                }
            }
        }
    }

    #[test]
    fn checked_out_branches_in_worktrees_are_rejected_with_additional_information() -> Result {
        let root = git_path::realpath(git_testtools::scripted_fixture_read_only_with_args(
            "make_fetch_repos.sh",
            [base_repo_path()],
        )?)?;
        let repo = root.join("worktree-root");
        let repo = git::open_opts(repo, git::open::Options::isolated())?;
        for (branch, path_from_root) in [
            ("main", "worktree-root"),
            ("wt-a-nested", "prev/wt-a-nested"),
            ("wt-a", "wt-a"),
            ("nested-wt-b", "wt-a/nested-wt-b"),
            ("wt-c-locked", "wt-c-locked"),
            ("wt-deleted", "wt-deleted"),
        ] {
            let spec = format!("refs/heads/main:refs/heads/{}", branch);
            let (mappings, specs) = mapping_from_spec(&spec, &repo);
            let out = fetch::refs::update(
                &repo,
                prefixed("action"),
                &mappings,
                &specs,
                &[],
                fetch::Tags::None,
                fetch::DryRun::Yes,
                fetch::WritePackedRefs::Never,
            )?;

            assert_eq!(
                out.updates,
                vec![fetch::refs::Update {
                    mode: fetch::refs::update::Mode::RejectedCurrentlyCheckedOut {
                        worktree_dir: root.join(path_from_root),
                    },
                    edit_index: None,
                }],
                "{}: checked-out checks are done before checking if a change would actually be required (here it isn't)", spec
            );
            assert_eq!(out.edits.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn local_symbolic_refs_are_never_written() {
        let repo = repo("two-origins");
        for source in ["refs/heads/main", "refs/heads/symbolic", "HEAD"] {
            let (mappings, specs) = mapping_from_spec(&format!("{source}:refs/heads/symbolic"), &repo);
            let out = fetch::refs::update(
                &repo,
                prefixed("action"),
                &mappings,
                &specs,
                &[],
                fetch::Tags::None,
                fetch::DryRun::Yes,
                fetch::WritePackedRefs::Never,
            )
            .unwrap();

            assert_eq!(out.edits.len(), 0);
            assert_eq!(
                out.updates,
                vec![fetch::refs::Update {
                    mode: fetch::refs::update::Mode::RejectedSymbolic,
                    edit_index: None
                }],
                "we don't overwrite these as the checked-out check needs to consider much more than it currently does, we are playing it safe"
            );
        }
    }

    #[test]
    fn remote_symbolic_refs_can_always_be_set_as_there_is_no_scenario_where_it_could_be_nonexisting_and_rejected() {
        let repo = repo("two-origins");
        let (mut mappings, specs) = mapping_from_spec("refs/heads/symbolic:refs/remotes/origin/new", &repo);
        mappings.push(Mapping {
            remote: Source::Ref(git_protocol::handshake::Ref::Direct {
                full_ref_name: "refs/heads/main".try_into().unwrap(),
                object: hex_to_id("f99771fe6a1b535783af3163eba95a927aae21d5"),
            }),
            local: Some("refs/heads/symbolic".into()),
            spec_index: SpecIndex::ExplicitInRemote(0),
        });
        let out = fetch::refs::update(
            &repo,
            prefixed("action"),
            &mappings,
            &specs,
            &[],
            fetch::Tags::None,
            fetch::DryRun::Yes,
            fetch::WritePackedRefs::Never,
        )
        .unwrap();

        assert_eq!(out.edits.len(), 1);
        assert_eq!(
            out.updates,
            vec![
                fetch::refs::Update {
                    mode: fetch::refs::update::Mode::New,
                    edit_index: Some(0)
                },
                fetch::refs::Update {
                    mode: fetch::refs::update::Mode::RejectedSymbolic,
                    edit_index: None
                }
            ],
        );
        let edit = &out.edits[0];
        match &edit.change {
            Change::Update { log, new, .. } => {
                assert_eq!(log.message, "action: storing ref");
                assert!(
                    new.try_name().is_some(),
                    "remote falls back to peeled id as it's the only thing we seem to have locally, it won't refer to a non-existing local ref"
                );
            }
            _ => unreachable!("only updates"),
        }
    }

    #[test]
    fn local_direct_refs_are_never_written_with_symbolic_ones_but_see_only_the_destination() {
        let repo = repo("two-origins");
        let (mappings, specs) = mapping_from_spec("refs/heads/symbolic:refs/heads/not-currently-checked-out", &repo);
        let out = fetch::refs::update(
            &repo,
            prefixed("action"),
            &mappings,
            &specs,
            &[],
            fetch::Tags::None,
            fetch::DryRun::Yes,
            fetch::WritePackedRefs::Never,
        )
        .unwrap();

        assert_eq!(out.edits.len(), 1);
        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::NoChangeNeeded,
                edit_index: Some(0)
            }],
        );
    }

    #[test]
    fn remote_refs_cannot_map_to_local_head() {
        let repo = repo("two-origins");
        let (mappings, specs) = mapping_from_spec("refs/heads/main:HEAD", &repo);
        let out = fetch::refs::update(
            &repo,
            prefixed("action"),
            &mappings,
            &specs,
            &[],
            fetch::Tags::None,
            fetch::DryRun::Yes,
            fetch::WritePackedRefs::Never,
        )
        .unwrap();

        assert_eq!(out.edits.len(), 1);
        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::New,
                edit_index: Some(0),
            }],
        );
        let edit = &out.edits[0];
        match &edit.change {
            Change::Update { log, new, .. } => {
                assert_eq!(log.message, "action: storing head");
                assert!(
                    new.try_id().is_some(),
                    "remote is peeled, so local will be peeled as well"
                );
            }
            _ => unreachable!("only updates"),
        }
        assert_eq!(
            edit.name.as_bstr(),
            "refs/heads/HEAD",
            "it's not possible to refer to the local HEAD with refspecs"
        );
    }

    #[test]
    fn remote_symbolic_refs_can_be_written_locally_and_point_to_tracking_branch() {
        let repo = repo("two-origins");
        let (mut mappings, specs) = mapping_from_spec("HEAD:refs/remotes/origin/new-HEAD", &repo);
        mappings.push(Mapping {
            remote: Source::Ref(git_protocol::handshake::Ref::Direct {
                full_ref_name: "refs/heads/main".try_into().unwrap(),
                object: hex_to_id("f99771fe6a1b535783af3163eba95a927aae21d5"),
            }),
            local: Some("refs/remotes/origin/main".into()),
            spec_index: SpecIndex::ExplicitInRemote(0),
        });
        let out = fetch::refs::update(
            &repo,
            prefixed("action"),
            &mappings,
            &specs,
            &[],
            fetch::Tags::None,
            fetch::DryRun::Yes,
            fetch::WritePackedRefs::Never,
        )
        .unwrap();

        assert_eq!(
            out.updates,
            vec![
                fetch::refs::Update {
                    mode: fetch::refs::update::Mode::New,
                    edit_index: Some(0),
                },
                fetch::refs::Update {
                    mode: fetch::refs::update::Mode::NoChangeNeeded,
                    edit_index: Some(1),
                }
            ],
        );
        assert_eq!(out.edits.len(), 2);
        let edit = &out.edits[0];
        match &edit.change {
            Change::Update { log, new, .. } => {
                assert_eq!(log.message, "action: storing ref");
                assert_eq!(
                    new.try_name().expect("symbolic ref").as_bstr(),
                    "refs/remotes/origin/main",
                    "remote is symbolic, so local will be symbolic as well, but is rewritten to tracking branch"
                );
            }
            _ => unreachable!("only updates"),
        }
        assert_eq!(edit.name.as_bstr(), "refs/remotes/origin/new-HEAD",);
    }

    #[test]
    fn non_fast_forward_is_rejected_but_appears_to_be_fast_forward_in_dryrun_mode() {
        let repo = repo("two-origins");
        let (mappings, specs) = mapping_from_spec("refs/heads/main:refs/remotes/origin/g", &repo);
        let reflog_message: BString = "very special".into();
        let out = fetch::refs::update(
            &repo,
            RefLogMessage::Override {
                message: reflog_message.clone(),
            },
            &mappings,
            &specs,
            &[],
            fetch::Tags::None,
            fetch::DryRun::Yes,
            fetch::WritePackedRefs::Never,
        )
        .unwrap();

        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::FastForward,
                edit_index: Some(0),
            }],
            "The caller has to be aware and note that dry-runs can't know about fast-forwards as they don't have remote objects"
        );
        assert_eq!(out.edits.len(), 1);
        let edit = &out.edits[0];
        match &edit.change {
            Change::Update { log, .. } => {
                assert_eq!(log.message, reflog_message);
            }
            _ => unreachable!("only updates"),
        }
    }

    #[test]
    fn non_fast_forward_is_rejected_if_dry_run_is_disabled() {
        let (repo, _tmp) = repo_rw("two-origins");
        let (mappings, specs) = mapping_from_spec("refs/remotes/origin/g:refs/heads/not-currently-checked-out", &repo);
        let out = fetch::refs::update(
            &repo,
            prefixed("action"),
            &mappings,
            &specs,
            &[],
            fetch::Tags::None,
            fetch::DryRun::No,
            fetch::WritePackedRefs::Never,
        )
        .unwrap();

        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::RejectedNonFastForward,
                edit_index: None,
            }]
        );
        assert_eq!(out.edits.len(), 0);

        let (mappings, specs) = mapping_from_spec("refs/heads/main:refs/remotes/origin/g", &repo);
        let out = fetch::refs::update(
            &repo,
            prefixed("prefix"),
            &mappings,
            &specs,
            &[],
            fetch::Tags::None,
            fetch::DryRun::No,
            fetch::WritePackedRefs::Never,
        )
        .unwrap();

        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::FastForward,
                edit_index: Some(0),
            }]
        );
        assert_eq!(out.edits.len(), 1);
        let edit = &out.edits[0];
        match &edit.change {
            Change::Update { log, .. } => {
                assert_eq!(log.message, format!("prefix: {}", "fast-forward"));
            }
            _ => unreachable!("only updates"),
        }
    }

    #[test]
    fn fast_forwards_are_called_out_even_if_force_is_given() {
        let (repo, _tmp) = repo_rw("two-origins");
        let (mappings, specs) = mapping_from_spec("+refs/heads/main:refs/remotes/origin/g", &repo);
        let out = fetch::refs::update(
            &repo,
            prefixed("prefix"),
            &mappings,
            &specs,
            &[],
            fetch::Tags::None,
            fetch::DryRun::No,
            fetch::WritePackedRefs::Never,
        )
        .unwrap();

        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::FastForward,
                edit_index: Some(0),
            }]
        );
        assert_eq!(out.edits.len(), 1);
        let edit = &out.edits[0];
        match &edit.change {
            Change::Update { log, .. } => {
                assert_eq!(log.message, format!("prefix: {}", "fast-forward"));
            }
            _ => unreachable!("only updates"),
        }
    }

    fn mapping_from_spec(spec: &str, repo: &git::Repository) -> (Vec<fetch::Mapping>, Vec<git::refspec::RefSpec>) {
        let spec = git_refspec::parse(spec.into(), git_refspec::parse::Operation::Fetch).unwrap();
        let group = git_refspec::MatchGroup::from_fetch_specs(Some(spec));
        let references = repo.references().unwrap();
        let mut references: Vec<_> = references.all().unwrap().map(|r| into_remote_ref(r.unwrap())).collect();
        references.push(into_remote_ref(repo.find_reference("HEAD").unwrap()));
        let mappings = group
            .match_remotes(references.iter().map(remote_ref_to_item))
            .mappings
            .into_iter()
            .map(|m| fetch::Mapping {
                remote: m
                    .item_index
                    .map(|idx| fetch::Source::Ref(references[idx].clone()))
                    .unwrap_or_else(|| match m.lhs {
                        git_refspec::match_group::SourceRef::ObjectId(id) => fetch::Source::ObjectId(id),
                        _ => unreachable!("not a ref, must be id: {:?}", m),
                    }),
                local: m.rhs.map(|r| r.into_owned()),
                spec_index: SpecIndex::ExplicitInRemote(m.spec_index),
            })
            .collect();
        (mappings, vec![spec.to_owned()])
    }

    fn into_remote_ref(mut r: git::Reference<'_>) -> git_protocol::handshake::Ref {
        let full_ref_name = r.name().as_bstr().into();
        match r.target() {
            TargetRef::Peeled(id) => git_protocol::handshake::Ref::Direct {
                full_ref_name,
                object: id.into(),
            },
            TargetRef::Symbolic(name) => {
                let target = name.as_bstr().into();
                let id = r.peel_to_id_in_place().unwrap();
                git_protocol::handshake::Ref::Symbolic {
                    full_ref_name,
                    target,
                    object: id.detach(),
                }
            }
        }
    }

    fn remote_ref_to_item(r: &git_protocol::handshake::Ref) -> git_refspec::match_group::Item<'_> {
        let (full_ref_name, target, object) = r.unpack();
        git_refspec::match_group::Item {
            full_ref_name,
            target: target.expect("no unborn HEAD"),
            object,
        }
    }

    fn prefixed(action: &str) -> RefLogMessage {
        RefLogMessage::Prefixed { action: action.into() }
    }
}
