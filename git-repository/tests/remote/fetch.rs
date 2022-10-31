#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use std::sync::atomic::AtomicBool;

    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::{fetch, Direction::Fetch};
    use git_testtools::hex_to_id;

    use crate::remote;

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

    #[test]
    fn fetch_empty_pack() -> crate::Result {
        let (repo, _tmp) = repo_rw("two-origins");
        let mut remote = repo.head()?.into_remote(Fetch).expect("present")?;
        remote.replace_refspecs(Some("HEAD:refs/remotes/origin/does-not-yet-exist"), Fetch)?;

        let res: git::remote::fetch::Outcome = remote
            .connect(Fetch, git::progress::Discard)?
            .prepare_fetch(Default::default())?
            .receive(&AtomicBool::default())?;

        match res.status {
            git::remote::fetch::Status::Change {write_pack_bundle, update_refs} => {
                assert_eq!(write_pack_bundle.index.data_hash, hex_to_id("029d08823bd8a8eab510ad6ac75c823cfd3ed31e"));
                assert_eq!(write_pack_bundle.index.num_objects, 0, "empty pack");
                assert!(write_pack_bundle.data_path.as_deref().map_or(false, |p| p.is_file()));
                assert!(write_pack_bundle.index_path.as_deref().map_or(false, |p| p.is_file()));
                assert_eq!(update_refs.edits.len(), 1);
                assert!(!write_pack_bundle.keep_path.as_deref().map_or(false, |p| p.is_file()), ".keep files are deleted if at least one ref-edit was made or the pack is empty");
            },
            _ => unreachable!("Naive negotiation sends the same have and wants, resulting in an empty pack (technically no change, but we don't detect it) - empty packs are fine")
        }
        Ok(())
    }

    #[test]
    fn fetch_pack_without_local_destination() -> crate::Result {
        let (repo, _tmp) = repo_rw("two-origins");
        let mut remote = repo.find_remote("changes-on-top-of-origin")?;
        remote.replace_refspecs(Some("HEAD"), Fetch)?;

        let res: git::remote::fetch::Outcome = remote
            .connect(Fetch, git::progress::Discard)?
            .prepare_fetch(Default::default())?
            .receive(&AtomicBool::default())?;

        match res.status {
            git::remote::fetch::Status::Change {write_pack_bundle, update_refs} => {
                assert_eq!(write_pack_bundle.index.data_hash, hex_to_id("edc8cc8a25e64e73aacea469fc765564dd2c3f65"));
                assert_eq!(write_pack_bundle.index.num_objects, 4);
                assert!(write_pack_bundle.data_path.as_deref().map_or(false, |p| p.is_file()));
                assert!(write_pack_bundle.index_path.as_deref().map_or(false, |p| p.is_file()));
                assert_eq!(update_refs.edits.len(), 0);
                assert!(write_pack_bundle.keep_path.as_deref().map_or(false, |p| p.is_file()), ".keep are kept if there was no edit to bind the packs objects to our commit graph");
            },
            _ => unreachable!("Naive negotiation sends the same have and wants, resulting in an empty pack (technically no change, but we don't detect it) - empty packs are fine")
        }
        Ok(())
    }

    #[test]
    fn fetch_pack() -> crate::Result {
        for (version, expected_objects, expected_hash) in [
            (None, 4, "d07c527cf14e524a8494ce6d5d08e28079f5c6ea"),
            (
                Some(git::protocol::transport::Protocol::V2),
                4,
                "d07c527cf14e524a8494ce6d5d08e28079f5c6ea",
            ),
            (
                Some(git::protocol::transport::Protocol::V1),
                4,
                "d07c527cf14e524a8494ce6d5d08e28079f5c6ea", // TODO: if these are the same, remove them
            ),
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
            for dry_run in [true, false] {
                let remote = repo.find_remote("changes-on-top-of-origin")?;
                let outcome: git::remote::fetch::Outcome = remote
                    .connect(Fetch, progress::Discard)?
                    .prepare_fetch(Default::default())?
                    .with_dry_run(dry_run)
                    .receive(&AtomicBool::default())?;
                let refs = match outcome.status {
                    fetch::Status::Change {
                        write_pack_bundle,
                        update_refs,
                    } => {
                        assert_eq!(write_pack_bundle.pack_version, git::odb::pack::data::Version::V2);
                        assert_eq!(write_pack_bundle.object_hash, repo.object_hash());
                        assert_eq!(write_pack_bundle.index.num_objects, expected_objects, "{dry_run}: this value is 4 when git does it with 'consecutive' negotiation style, but could be 33 if completely naive.");
                        assert_eq!(
                            write_pack_bundle.index.index_version,
                            git::odb::pack::index::Version::V2
                        );
                        assert_eq!(write_pack_bundle.index.index_hash, hex_to_id(expected_hash));
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
                    fetch::Status::NoChange => unreachable!("we firmly expect changes here"),
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
                for (_update, mapping, _spec, edit) in
                    refs.iter_mapping_updates(&outcome.ref_map.mappings, remote.refspecs(Fetch))
                {
                    let edit = edit.expect("refedit present even if it's a no-op");
                    if dry_run {
                        assert_eq!(
                            edit.change.new_value().expect("no deletions").id(),
                            mapping.remote.as_id()
                        );
                        assert!(
                            repo.try_find_reference(edit.name.as_ref())?.is_none(),
                            "no ref created in dry-run mode"
                        );
                    } else {
                        let r = repo.find_reference(edit.name.as_ref()).unwrap();
                        assert_eq!(
                            r.id(),
                            *mapping.remote.as_id(),
                            "local reference should point to remote id"
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
