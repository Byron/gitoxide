use git_repository as git;

use crate::remote;

#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use git_object::bstr::ByteSlice;
    use git_ref::TargetRef;
    use git_repository as git;
    use git_repository::remote::fetch::SpecIndex;

    use crate::remote;

    #[test]
    fn fetch_only_with_configuration() -> crate::Result {
        let tmp = git_testtools::tempfile::TempDir::new()?;
        let called_configure_remote = std::sync::Arc::new(std::sync::atomic::AtomicBool::default());
        let remote_name = "special";
        let desired_fetch_tags = git::remote::fetch::Tags::Included;
        let mut prepare = git::clone::PrepareFetch::new(
            remote::repo("base").path(),
            tmp.path(),
            git::create::Kind::Bare,
            Default::default(),
            git::open::Options::isolated().config_overrides([
                "init.defaultBranch=unused-as-overridden-by-remote",
                "core.logAllRefUpdates",
            ]),
        )?
        .with_remote_name(remote_name)?
        .configure_remote({
            let called_configure_remote = called_configure_remote.clone();
            move |r| {
                called_configure_remote.store(true, std::sync::atomic::Ordering::Relaxed);
                let r = r
                    .with_refspecs(Some("+refs/tags/b-tag:refs/tags/b-tag"), git::remote::Direction::Fetch)?
                    .with_fetch_tags(desired_fetch_tags);
                Ok(r)
            }
        });
        let (repo, out) = prepare.fetch_only(git::progress::Discard, &std::sync::atomic::AtomicBool::default())?;
        drop(prepare);

        assert!(
            called_configure_remote.load(std::sync::atomic::Ordering::Relaxed),
            "custom remote configuration is called"
        );
        assert_eq!(repo.remote_names().len(), 1, "only ever one remote");
        let remote = repo.find_remote(remote_name)?;
        let num_refspecs = remote.refspecs(git::remote::Direction::Fetch).len();
        assert_eq!(
            num_refspecs, 2,
            "our added spec was stored as well, but no implied specs due to the `Tags::All` setting"
        );
        assert_eq!(
            remote.fetch_tags(),
            desired_fetch_tags,
            "fetch-tags are persisted via the 'tagOpt` key"
        );
        assert!(
            git::path::from_bstr(
                remote
                    .url(git::remote::Direction::Fetch)
                    .expect("present")
                    .path
                    .as_ref()
            )
            .is_absolute(),
            "file urls can't be relative paths"
        );

        let (explicit_max_idx, implicit_max_index) =
            out.ref_map
                .mappings
                .iter()
                .map(|m| m.spec_index)
                .fold((0, 0), |(a, b), i| match i {
                    SpecIndex::ExplicitInRemote(idx) => (idx.max(a), b),
                    SpecIndex::Implicit(idx) => (a, idx.max(b)),
                });
        assert_eq!(
            explicit_max_idx,
            num_refspecs - 1,
            "mappings don't refer to non-existing explicit refspecs"
        );
        assert_eq!(
            implicit_max_index,
            &out.ref_map.extra_refspecs.len() - 1,
            "mappings don't refer to non-existing implicit refspecs"
        );
        let packed_refs = repo
            .refs
            .cached_packed_buffer()?
            .expect("packed refs should be present");
        assert_eq!(
            repo.refs.loose_iter()?.count(),
            2,
            "HEAD and an actual symbolic ref we received"
        );
        assert_eq!(
            packed_refs.iter()?.count(),
            14,
            "all non-symbolic refs should be stored, if reachable from our refs"
        );

        match out.status {
            git_repository::remote::fetch::Status::Change { update_refs, .. } => {
                for edit in &update_refs.edits {
                    use git_odb::Find;
                    match edit.change.new_value().expect("always set/no deletion") {
                        TargetRef::Symbolic(referent) => {
                            assert!(
                                repo.find_reference(referent).is_ok(),
                                "if we set up a symref, the target should exist by now"
                            )
                        }
                        TargetRef::Peeled(id) => {
                            assert!(repo.objects.contains(id), "part of the fetched pack");
                        }
                    }
                    let r = repo
                        .find_reference(edit.name.as_ref())
                        .unwrap_or_else(|_| panic!("didn't find created reference: {:?}", edit));
                    if r.name().category().expect("known") != git_ref::Category::Tag {
                        assert!(r
                            .name()
                            .category_and_short_name()
                            .expect("computable")
                            .1
                            .starts_with_str(remote_name));
                        let mut logs = r.log_iter();
                        assert_reflog(logs.all());
                    }
                }
                let mut out_of_graph_tags = Vec::new();
                for mapping in update_refs
                    .updates
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, update)| {
                        matches!(
                            update.mode,
                            git::remote::fetch::refs::update::Mode::ImplicitTagNotSentByRemote
                        )
                        .then(|| idx)
                    })
                    .map(|idx| &out.ref_map.mappings[idx])
                {
                    out_of_graph_tags.push(
                        mapping
                            .remote
                            .as_name()
                            .expect("tag always has a path")
                            .to_str()
                            .expect("valid UTF8"),
                    );
                }
                assert_eq!(
                    out_of_graph_tags,
                    &[
                        "refs/tags/annotated-detached-tag",
                        "refs/tags/annotated-future-tag",
                        "refs/tags/detached-tag",
                        "refs/tags/future-tag"
                    ]
                );
            }
            _ => unreachable!("clones are always causing changes and dry-runs aren't possible"),
        }

        let remote_head = repo
            .find_reference(&format!("refs/remotes/{remote_name}/HEAD"))
            .expect("remote HEAD present");
        assert_eq!(
            remote_head
                .target()
                .try_name()
                .expect("remote HEAD is symbolic")
                .as_bstr(),
            format!("refs/remotes/{remote_name}/main"),
            "it points to the local tracking branch of what the remote actually points to"
        );

        let head = repo.head()?;
        {
            let mut logs = head.log_iter();
            assert_reflog(logs.all());
        }

        let referent = head.try_into_referent().expect("symbolic ref is present");
        assert!(
            referent.id().object().is_ok(),
            "the object pointed to by HEAD was fetched as well"
        );
        assert_eq!(
            referent.name().as_bstr(),
            remote::repo("base").head_name()?.expect("symbolic").as_bstr(),
            "local clone always adopts the name of the remote"
        );

        let short_name = referent.name().shorten();
        assert_eq!(
            repo.branch_remote_name(short_name).expect("remote is set").as_ref(),
            remote_name,
            "the remote branch information is fully configured"
        );
        assert_eq!(
            repo.branch_remote_ref(short_name).expect("present")?.as_bstr(),
            "refs/heads/main"
        );

        {
            let mut logs = referent.log_iter();
            assert_reflog(logs.all());
        }
        Ok(())
    }

    fn assert_reflog(log: std::io::Result<Option<git_ref::file::log::iter::Forward<'_>>>) {
        let lines = log
            .unwrap()
            .expect("log present")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(lines.len(), 1, "just created");
        let line = &lines[0];
        assert!(
            line.message.starts_with(b"clone: from "),
            "{:?} unexpected",
            line.message
        );
        let path = git_path::from_bstr(line.message.rsplit(|b| *b == b' ').next().expect("path").as_bstr());
        assert!(path.is_absolute(), "{:?} must be absolute", path);
    }

    #[test]
    fn fetch_and_checkout() -> crate::Result {
        let tmp = git_testtools::tempfile::TempDir::new()?;
        let mut prepare = git::prepare_clone(remote::repo("base").path(), tmp.path())?;
        let (mut checkout, _out) =
            prepare.fetch_then_checkout(git::progress::Discard, &std::sync::atomic::AtomicBool::default())?;
        let (repo, _) = checkout.main_worktree(git::progress::Discard, &std::sync::atomic::AtomicBool::default())?;

        let index = repo.index()?;
        assert_eq!(index.entries().len(), 1, "All entries are known as per HEAD tree");

        let work_dir = repo.work_dir().expect("non-bare");
        for entry in index.entries() {
            let entry_path = work_dir.join(git_path::from_bstr(entry.path(&index)));
            assert!(entry_path.is_file(), "{:?} not found on disk", entry_path)
        }
        Ok(())
    }

    #[test]
    fn fetch_and_checkout_empty_remote_repo() -> crate::Result {
        let tmp = git_testtools::tempfile::TempDir::new()?;
        let mut prepare = git::prepare_clone(
            git_testtools::scripted_fixture_read_only("make_empty_repo.sh")?,
            tmp.path(),
        )?;
        let (mut checkout, out) = prepare
            .fetch_then_checkout(git::progress::Discard, &std::sync::atomic::AtomicBool::default())
            .unwrap();
        let (repo, _) = checkout.main_worktree(git::progress::Discard, &std::sync::atomic::AtomicBool::default())?;

        assert!(!repo.index_path().is_file(), "newly initialized repos have no index");
        let head = repo.head()?;
        assert!(head.is_unborn());

        assert!(
            head.log_iter().all()?.is_none(),
            "no reflog for unborn heads (as it needs non-null destination hash)"
        );

        if out
            .ref_map
            .handshake
            .capabilities
            .capability("ls-refs")
            .expect("has ls-refs")
            .supports("unborn")
            == Some(true)
        {
            assert_eq!(
                head.referent_name().expect("present").as_bstr(),
                "refs/heads/special",
                "we pick up the name as present on the server, not the one we default to"
            );
        } else {
            assert_eq!(
                head.referent_name().expect("present").as_bstr(),
                "refs/heads/main",
                "we simply keep our own post-init HEAD which defaults to the branch name we configured locally"
            );
        }

        Ok(())
    }

    #[test]
    fn fetch_only_without_configuration() -> crate::Result {
        let tmp = git_testtools::tempfile::TempDir::new()?;
        let (repo, out) = git::prepare_clone_bare(remote::repo("base").path(), tmp.path())?
            .fetch_only(git::progress::Discard, &std::sync::atomic::AtomicBool::default())?;
        assert!(repo.find_remote("origin").is_ok(), "default remote name is 'origin'");
        match out.status {
            git_repository::remote::fetch::Status::Change { write_pack_bundle, .. } => {
                assert!(
                    write_pack_bundle.keep_path.is_none(),
                    "keep files aren't kept if refs are written"
                );
            }
            _ => unreachable!("a clone always carries a change"),
        }
        Ok(())
    }
}

#[test]
fn clone_and_early_persist_without_receive() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    let repo = git::prepare_clone_bare(remote::repo("base").path(), tmp.path())?.persist();
    assert!(repo.is_bare(), "repo is now ours and remains");
    Ok(())
}

#[test]
fn clone_and_destination_must_be_empty() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    std::fs::write(tmp.path().join("file"), b"hello")?;
    match git::prepare_clone_bare(remote::repo("base").path(), tmp.path()) {
        Ok(_) => unreachable!("this should fail as the directory isn't empty"),
        Err(err) => assert!(err
            .to_string()
            .starts_with("Refusing to initialize the non-empty directory as ")),
    }
    Ok(())
}

#[test]
fn clone_bare_into_empty_directory_and_early_drop() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    // this breaks isolation, but shouldn't be affecting the test. If so, use isolation options for opening the repo.
    let prep = git::prepare_clone_bare(remote::repo("base").path(), tmp.path())?;
    let head = tmp.path().join("HEAD");
    assert!(head.is_file(), "now a bare basic repo is present");
    drop(prep);

    assert!(!head.is_file(), "we cleanup if the clone isn't followed through");
    Ok(())
}

#[test]
fn clone_into_empty_directory_and_early_drop() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    let prep = git::prepare_clone(remote::repo("base").path(), tmp.path())?;
    let head = tmp.path().join(".git").join("HEAD");
    assert!(head.is_file(), "now a basic repo is present");
    drop(prep);

    assert!(!head.is_file(), "we cleanup if the clone isn't followed through");
    Ok(())
}
