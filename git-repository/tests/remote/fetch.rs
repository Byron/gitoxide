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
                        update_refs: refs,
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

                        assert_eq!(
                            refs.updates,
                            vec![fetch::refs::Update {
                                mode: fetch::refs::update::Mode::New,
                                edit_index: Some(0),
                            }]
                        );
                        for (_update, mapping, _spec, edit) in refs.iter_mapping_updates(
                            &outcome.ref_map.mappings,
                            remote.refspecs(git::remote::Direction::Fetch),
                        ) {
                            let edit = edit.expect("refedit present even if it's a no-op");
                            let r = repo.find_reference(edit.name.as_ref()).unwrap();
                            assert_eq!(
                                r.id(),
                                *mapping.remote.as_id(),
                                "local reference should point to remote id"
                            );
                        }
                    }
                    fetch::Status::NoChange => unreachable!("we firmly expect changes here"),
                }
            }
        }
        Ok(())
    }
}
