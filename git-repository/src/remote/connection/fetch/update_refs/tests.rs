mod update {
    use crate as git;
    use git_testtools::{hex_to_id, Result};

    fn base_repo_path() -> String {
        git::path::realpath(
            git_testtools::scripted_fixture_repo_read_only("make_remote_repos.sh")
                .unwrap()
                .join("base"),
        )
        .unwrap()
        .to_string_lossy()
        .into_owned()
    }

    fn repo(name: &str) -> git::Repository {
        let dir = git_testtools::scripted_fixture_repo_read_only_with_args("make_fetch_repos.sh", [base_repo_path()])
            .unwrap();
        git::open_opts(dir.join(name), git::open::Options::isolated()).unwrap()
    }
    fn repo_rw(name: &str) -> (git::Repository, git_testtools::tempfile::TempDir) {
        let dir = git_testtools::scripted_fixture_repo_writable_with_args(
            "make_fetch_repos.sh",
            [base_repo_path()],
            git_testtools::Creation::ExecuteScript,
        )
        .unwrap();
        let repo = git::open_opts(dir.path().join(name), git::open::Options::isolated()).unwrap();
        (repo, dir)
    }
    use crate::remote::fetch;
    use git_ref::transaction::Change;
    use git_ref::TargetRef;

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
                fetch::refs::update::Mode::Forced,
                Some("forced-update"),
                "a forced non-fastforward (main goes backwards)",
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
                "action",
                &mapping,
                &specs,
                reflog_message.map(|_| fetch::DryRun::Yes).unwrap_or(fetch::DryRun::No),
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
    fn checked_out_branches_in_worktrees_are_rejected_with_additional_infromation() -> Result {
        let root = git_path::realpath(git_testtools::scripted_fixture_repo_read_only_with_args(
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
            let out = fetch::refs::update(&repo, "action", &mappings, &specs, fetch::DryRun::Yes)?;

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
    fn symbolic_refs_are_never_written() {
        let repo = repo("two-origins");
        let (mappings, specs) = mapping_from_spec("refs/heads/main:refs/heads/symbolic", &repo);
        let out = fetch::refs::update(&repo, "action", &mappings, &specs, fetch::DryRun::Yes).unwrap();

        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::RejectedSymbolic,
                edit_index: None,
            }],
            "this also protects from writing HEAD, which should in theory be impossible to get from a refspec as it normalizes partial ref names"
        );
        assert_eq!(out.edits.len(), 0);
    }

    #[test]
    fn non_fast_forward_is_rejected_but_appears_to_be_fast_forward_in_dryrun_mode() {
        let repo = repo("two-origins");
        let (mappings, specs) = mapping_from_spec("refs/heads/main:refs/remotes/origin/g", &repo);
        let out = fetch::refs::update(&repo, "action", &mappings, &specs, fetch::DryRun::Yes).unwrap();

        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::FastForward,
                edit_index: Some(0),
            }],
            "The caller has to be aware and note that dry-runs can't know about fast-forwards as they don't have remote objects"
        );
        assert_eq!(out.edits.len(), 1);
    }

    #[test]
    fn non_fast_forward_is_rejected_if_dry_run_is_disabled() {
        let (repo, _tmp) = repo_rw("two-origins");
        let (mappings, specs) = mapping_from_spec("refs/remotes/origin/g:refs/heads/not-currently-checked-out", &repo);
        let out = fetch::refs::update(&repo, "action", &mappings, &specs, fetch::DryRun::No).unwrap();

        assert_eq!(
            out.updates,
            vec![fetch::refs::Update {
                mode: fetch::refs::update::Mode::RejectedNonFastForward,
                edit_index: None,
            }]
        );
        assert_eq!(out.edits.len(), 0);

        let (mappings, specs) = mapping_from_spec("refs/heads/main:refs/remotes/origin/g", &repo);
        let out = fetch::refs::update(&repo, "prefix", &mappings, &specs, fetch::DryRun::No).unwrap();

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
        let references: Vec<_> = references.all().unwrap().map(|r| into_remote_ref(r.unwrap())).collect();
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
