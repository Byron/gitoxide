#[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
mod blocking_and_async_io {
    use std::sync::atomic::AtomicBool;

    use gix::remote::{fetch, Direction::Fetch};
    use gix_features::progress;
    use gix_protocol::maybe_async;

    use crate::{
        remote::{into_daemon_remote_if_async, spawn_git_daemon_if_async},
        util::hex_to_id,
    };

    pub(crate) fn base_repo_path() -> String {
        gix::path::realpath(
            gix_testtools::scripted_fixture_read_only("make_remote_repos.sh")
                .unwrap()
                .join("base"),
        )
        .unwrap()
        .to_string_lossy()
        .into_owned()
    }

    pub(crate) fn repo_path(name: &str) -> std::path::PathBuf {
        let dir =
            gix_testtools::scripted_fixture_read_only_with_args("make_fetch_repos.sh", [base_repo_path()]).unwrap();
        dir.join(name)
    }

    #[allow(clippy::result_large_err)]
    pub(crate) fn try_repo_rw(
        name: &str,
    ) -> Result<(gix::Repository, gix_testtools::tempfile::TempDir), gix::open::Error> {
        let dir = gix_testtools::scripted_fixture_writable_with_args(
            "make_fetch_repos.sh",
            [base_repo_path()],
            gix_testtools::Creation::ExecuteScript,
        )
        .unwrap();
        let repo = gix::open_opts(dir.path().join(name), crate::restricted())?;
        Ok((repo, dir))
    }

    pub(crate) fn repo_rw(name: &str) -> (gix::Repository, gix_testtools::tempfile::TempDir) {
        try_repo_rw(name).unwrap()
    }

    #[test]
    #[cfg(feature = "blocking-network-client")]
    fn collate_fetch_error() -> Result<(), gix::env::collate::fetch::Error<std::io::Error>> {
        let (repo, _tmp) = try_repo_rw("two-origins")?;
        let remote = repo
            .head()?
            .into_remote(Fetch)
            .expect("present")?
            .with_fetch_tags(fetch::Tags::Included);

        remote
            .connect(Fetch, gix::progress::Discard)?
            .prepare_fetch(Default::default())?
            .receive(&AtomicBool::default())?;

        assert!(
            repo.path()
                .join("HEAD")
                .metadata()
                .map_err(gix::env::collate::fetch::Error::Other)?
                .is_file(),
            "just to show off the 'Other' error type"
        );
        Ok(())
    }

    #[maybe_async::test(
        feature = "blocking-network-client",
        async(feature = "async-network-client-async-std", async_std::test)
    )]
    async fn fetch_empty_pack() -> crate::Result {
        let daemon = spawn_git_daemon_if_async(repo_path("base"))?;
        for (fetch_tags, expected_ref_count) in [
            (fetch::Tags::None, 1),
            (fetch::Tags::Included, 7),
            (fetch::Tags::All, 7),
        ] {
            let (repo, _tmp) = repo_rw("two-origins");
            let mut remote = into_daemon_remote_if_async(
                repo.head()?
                    .into_remote(Fetch)
                    .expect("present")?
                    .with_fetch_tags(fetch_tags),
                daemon.as_ref(),
                None,
            );
            remote.replace_refspecs(Some("HEAD:refs/remotes/origin/does-not-yet-exist"), Fetch)?;

            let res = remote
                .connect(Fetch, gix::progress::Discard)
                .await?
                .prepare_fetch(Default::default())
                .await?
                .receive(&AtomicBool::default())
                .await?;

            match res.status {
                fetch::Status::Change { write_pack_bundle, update_refs } => {
                    assert_eq!(write_pack_bundle.index.data_hash, hex_to_id("029d08823bd8a8eab510ad6ac75c823cfd3ed31e"));
                    assert_eq!(write_pack_bundle.index.num_objects, 0, "empty pack");
                    assert!(write_pack_bundle.data_path.as_deref().map_or(false, |p| p.is_file()));
                    assert!(write_pack_bundle.index_path.as_deref().map_or(false, |p| p.is_file()));
                    assert_eq!(update_refs.edits.len(), expected_ref_count);
                    assert!(!write_pack_bundle.keep_path.as_deref().map_or(false, |p| p.is_file()), ".keep files are deleted if at least one ref-edit was made or the pack is empty");
                },
                _ => unreachable!("Naive negotiation sends the same have and wants, resulting in an empty pack (technically no change, but we don't detect it) - empty packs are fine")
            }
        }
        Ok(())
    }

    #[maybe_async::test(
        feature = "blocking-network-client",
        async(feature = "async-network-client-async-std", async_std::test)
    )]
    async fn fetch_pack_without_local_destination() -> crate::Result {
        let daemon = spawn_git_daemon_if_async(repo_path("clone-as-base-with-changes"))?;
        for (fetch_tags, expected_data_hash, num_objects_offset, expected_ref_edits) in [
            (
                gix::remote::fetch::Tags::None,
                "de303ef102bd5705a40a0c42ae2972eb1a668455",
                0,
                0,
            ),
            (
                gix::remote::fetch::Tags::Included,
                "edc8cc8a25e64e73aacea469fc765564dd2c3f65",
                1,
                7,
            ),
            (
                gix::remote::fetch::Tags::All,
                "edc8cc8a25e64e73aacea469fc765564dd2c3f65",
                1,
                7,
            ),
        ] {
            let (repo, _tmp) = repo_rw("two-origins");
            let mut remote = into_daemon_remote_if_async(
                repo.find_remote("changes-on-top-of-origin")?
                    .with_fetch_tags(fetch_tags),
                daemon.as_ref(),
                None,
            );
            remote.replace_refspecs(Some("HEAD"), Fetch)?;

            let res: gix::remote::fetch::Outcome = remote
                .connect(Fetch, gix::progress::Discard)
                .await?
                .prepare_fetch(Default::default())
                .await?
                .receive(&AtomicBool::default())
                .await?;

            match res.status {
                gix::remote::fetch::Status::Change { write_pack_bundle, update_refs } => {
                    assert_eq!(write_pack_bundle.index.data_hash, hex_to_id(expected_data_hash), );
                    assert_eq!(write_pack_bundle.index.num_objects, 3 + num_objects_offset, "{fetch_tags:?}");
                    assert!(write_pack_bundle.data_path.as_deref().map_or(false, |p| p.is_file()));
                    assert!(write_pack_bundle.index_path.as_deref().map_or(false, |p| p.is_file()));
                    assert_eq!(update_refs.edits.len(), expected_ref_edits, "{fetch_tags:?}");
                    assert_eq!(write_pack_bundle.keep_path.as_deref().map_or(false, |p| p.is_file()), update_refs.edits.is_empty(),".keep are kept if there was no edit to prevent `git gc` from clearing out the pack as it's not referred to necessarily");
                },
                _ => unreachable!("Naive negotiation sends the same have and wants, resulting in an empty pack (technically no change, but we don't detect it) - empty packs are fine")
            }
        }
        Ok(())
    }

    #[maybe_async::test(
        feature = "blocking-network-client",
        async(feature = "async-network-client-async-std", async_std::test)
    )]
    async fn fetch_pack() -> crate::Result {
        let daemon = spawn_git_daemon_if_async({
            let mut p = repo_path("base");
            p.pop();
            p
        })?;
        for version in [
            None,
            Some(gix::protocol::transport::Protocol::V2),
            Some(gix::protocol::transport::Protocol::V1),
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
                let remote = into_daemon_remote_if_async(repo.find_remote("origin")?, daemon.as_ref(), "base");
                {
                    remote
                        .connect(Fetch, progress::Discard)
                        .await?
                        .prepare_fetch(Default::default())
                        .await?;
                    // early drops are fine and won't block.
                }
                let outcome = remote
                    .connect(Fetch, progress::Discard)
                    .await?
                    .prepare_fetch(Default::default())
                    .await?
                    .receive(&AtomicBool::default())
                    .await?;
                assert!(matches!(
                    outcome.status,
                    gix::remote::fetch::Status::NoPackReceived { .. }
                ));
            }

            // Some updates to be fetched
            for dry_run in [true, false] {
                let remote = into_daemon_remote_if_async(
                    repo.find_remote("changes-on-top-of-origin")?
                        .with_fetch_tags(gix::remote::fetch::Tags::None),
                    daemon.as_ref(),
                    "clone-as-base-with-changes",
                );
                let outcome: gix::remote::fetch::Outcome = remote
                    .connect(Fetch, progress::Discard)
                    .await?
                    .prepare_fetch(Default::default())
                    .await?
                    .with_dry_run(dry_run)
                    .receive(&AtomicBool::default())
                    .await?;
                let refs = match outcome.status {
                    fetch::Status::Change {
                        write_pack_bundle,
                        update_refs,
                    } => {
                        assert_eq!(write_pack_bundle.pack_version, gix::odb::pack::data::Version::V2);
                        assert_eq!(write_pack_bundle.object_hash, repo.object_hash());
                        assert_eq!(write_pack_bundle.index.num_objects, 4, "{dry_run}: this value is 4 when git does it with 'consecutive' negotiation style, but could be 33 if completely naive.");
                        assert_eq!(
                            write_pack_bundle.index.index_version,
                            gix::odb::pack::index::Version::V2
                        );
                        assert_eq!(
                            write_pack_bundle.index.index_hash,
                            hex_to_id("d07c527cf14e524a8494ce6d5d08e28079f5c6ea")
                        );
                        assert!(write_pack_bundle.data_path.map_or(false, |f| f.is_file()));
                        assert!(write_pack_bundle.index_path.map_or(false, |f| f.is_file()));
                        assert_eq!(update_refs.edits.len(), 2);

                        let edit = &update_refs.edits[0];
                        assert_eq!(edit.name.as_bstr(), "refs/remotes/changes-on-top-of-origin/main");
                        assert!(
                            edit.change.new_value().expect("no deletion").try_id().is_some(),
                            "a simple peeled ref"
                        );
                        let edit = &update_refs.edits[1];
                        assert_eq!(edit.name.as_bstr(), "refs/remotes/changes-on-top-of-origin/symbolic");
                        assert!(
                            edit.change.new_value().expect("no deletion").try_id().is_some(),
                            "on the remote this is a symbolic ref, we just write its destination object id though"
                        );

                        assert!(
                            !write_pack_bundle.keep_path.map_or(false, |f| f.is_file()),
                            ".keep files are deleted if there is one edit"
                        );

                        update_refs
                    }
                    fetch::Status::DryRun { update_refs } => update_refs,
                    fetch::Status::NoPackReceived { .. } => unreachable!("we firmly expect changes here"),
                };

                assert_eq!(
                    refs.updates,
                    vec![
                        fetch::refs::Update {
                            mode: fetch::refs::update::Mode::New,
                            edit_index: Some(0),
                        },
                        fetch::refs::Update {
                            mode: fetch::refs::update::Mode::New,
                            edit_index: Some(1),
                        }
                    ]
                );
                for (_update, mapping, _spec, edit) in refs.iter_mapping_updates(
                    &outcome.ref_map.mappings,
                    remote.refspecs(Fetch),
                    &outcome.ref_map.extra_refspecs,
                ) {
                    let edit = edit.expect("refedit present even if it's a no-op");
                    if dry_run {
                        assert_eq!(
                            edit.change.new_value().expect("no deletions").id(),
                            mapping.remote.as_id().expect("no unborn")
                        );
                        assert!(
                            repo.try_find_reference(edit.name.as_ref())?.is_none(),
                            "no ref created in dry-run mode"
                        );
                    } else {
                        let r = repo.find_reference(edit.name.as_ref()).unwrap();
                        assert_eq!(
                            r.id(),
                            *mapping.remote.as_id().expect("no unborn"),
                            "local reference should point to remote id"
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
