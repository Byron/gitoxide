use std::process::Command;

use git_odb::{store, Find, FindExt, Write};
use git_testtools::{fixture_path, hex_to_id};

fn db() -> git_odb::Handle {
    git_odb::at(fixture_path("objects")).expect("valid object path")
}

/// indices, multi-pack-index, loose odb
fn db_with_all_object_sources() -> crate::Result<(git_odb::Handle, tempfile::TempDir)> {
    let objects_dir = git_testtools::tempfile::tempdir()?;
    git_testtools::copy_recursively_into_existing_dir(fixture_path("objects"), &objects_dir)?;

    let multi_pack_index = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(objects_dir.path().join("pack/multi-pack-index"))?;
    git_odb::pack::multi_index::File::write_from_index_paths(
        vec![
            fixture_path("objects/pack/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx"),
            fixture_path("objects/pack/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx"),
        ],
        multi_pack_index,
        git_features::progress::Discard,
        &std::sync::atomic::AtomicBool::default(),
        git_odb::pack::multi_index::write::Options {
            object_hash: git_hash::Kind::Sha1,
        },
    )?;
    Ok((git_odb::at(objects_dir.path())?, objects_dir))
}

#[test]
fn multi_index_access() -> crate::Result {
    let dir = git_testtools::scripted_fixture_repo_writable("make_repo_multi_index.sh")?;
    let handle = git_odb::at(dir.path().join(".git/objects"))?;

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 0,
            open_reachable_indices: 0,
            known_reachable_indices: 0,
            open_reachable_packs: 0,
            known_packs: 0,
            unused_slots: 32,
            loose_dbs: 0,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "it starts out knowing nothing, it's completely lazy"
    );

    let mut count = 0;
    let mut buf = Vec::new();
    for oid in handle.iter()? {
        let oid = oid?;
        assert!(handle.contains(oid));
        assert!(handle.find(oid, &mut buf).is_ok());
        count += 1;
    }
    assert_eq!(count, 868);

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 1,
            open_reachable_indices: 1,
            known_reachable_indices: 1,
            open_reachable_packs: 1,
            known_packs: 1,
            unused_slots: 31,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "it opened only a single multi-index and its pack - hard to see it's actually a multi-index as it's just one index anyway…"
    );

    let non_existing_to_trigger_refresh = hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    handle.contains(non_existing_to_trigger_refresh);

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 2,
            open_reachable_indices: 1,
            known_reachable_indices: 1,
            open_reachable_packs: 1,
            known_packs: 1,
            unused_slots: 31,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "A miss means just another refresh with no other change"
    );

    filetime::set_file_mtime(
        handle.store_ref().path().join("pack/multi-pack-index"),
        filetime::FileTime::now(),
    )?;
    handle.contains(non_existing_to_trigger_refresh);

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 2 + 1 /*legit refresh with changes*/ + 1 /*a refresh attempt with no changes, causing 'contains()' to give up*/,
            open_reachable_indices: 1,
            known_reachable_indices: 1,
            open_reachable_packs: 0,
            known_packs: 1,
            unused_slots: 31,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "everything seems to remain as it was, even though we moved our multi-index to a new slot and removed the old one"
    );
    Ok(())
}

#[test]
fn multi_index_keep_open() -> crate::Result {
    let dir = git_testtools::scripted_fixture_repo_writable("make_repo_multi_index.sh")?;
    let (stable_handle, handle) = {
        let mut stable_handle = git_odb::at(dir.path().join(".git/objects"))?;
        let handle = stable_handle.clone();
        stable_handle.prevent_pack_unload();
        (stable_handle, handle)
    };
    let oid = handle.iter()?.next().expect("first oid")?;

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 2,
            num_refreshes: 1,
            open_reachable_indices: 1,
            known_reachable_indices: 1,
            open_reachable_packs: 0,
            known_packs: 1,
            unused_slots: 31,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "it opened the multi-pack index for iteration"
    );
    let mut buf = Vec::new();
    use git_pack::Find;
    let location = stable_handle
        .location_by_oid(oid, &mut buf)
        .expect("oid exists and is packed");

    let non_existing_to_trigger_refresh = hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    filetime::set_file_mtime(
        handle.store_ref().path().join("pack/multi-pack-index"),
        filetime::FileTime::now(),
    )?;
    git_odb::Find::contains(&handle, non_existing_to_trigger_refresh);

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 2,
            num_refreshes: 3,
            open_reachable_indices: 1,
            known_reachable_indices: 1,
            open_reachable_packs: 0, /*no pack is open anymore at least as seen from the index*/
            known_packs: 1,
            unused_slots: 30,
            loose_dbs: 1,
            unreachable_indices: 1,
            unreachable_packs: 1
        },
        "now there is an unreachable index and pack which is still loaded, but whose pack hasn't been loaded"
    );

    assert!(
        git_odb::pack::Find::entry_by_location(&stable_handle, &location).is_some(),
        "the entry can still be found even though the location is invalid"
    );
    Ok(())
}

#[test]
fn write() -> crate::Result {
    let dir = tempfile::tempdir()?;
    let mut handle = git_odb::at(dir.path())?;
    // It should refresh once even if the refresh mode is never, just to initialize the index
    handle.refresh_never();

    let written_id = handle.write_buf(git_object::Kind::Blob, b"hello world")?;
    assert_eq!(written_id, hex_to_id("95d09f2b10159347eece71399a7e2e907ea3df4f"));
    Ok(())
}

#[test]
fn contains() {
    let handle = db();

    assert!(handle.contains(hex_to_id("37d4e6c5c48ba0d245164c4e10d5f41140cab980"))); // loose object
    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 1,
            open_reachable_indices: 0,
            known_reachable_indices: 3,
            open_reachable_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "it only refreshed the file list, yielding the loose db to find this object, but no pack was opened yet"
    );

    // pack c043, the biggest one
    assert!(handle.contains(hex_to_id("dd25c539efbb0ab018caa4cda2d133285634e9b5")));

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 1,
            open_reachable_indices: 1,
            known_reachable_indices: 3,
            open_reachable_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "it loaded the biggest back only, which is the first in the list"
    );

    // pack, the smallest one
    // The new handle should make no difference.
    #[allow(clippy::redundant_clone)]
    let mut new_handle = handle.clone();
    assert!(new_handle.contains(hex_to_id("501b297447a8255d3533c6858bb692575cdefaa0")));
    assert_eq!(
        new_handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 2,
            num_refreshes: 1,
            open_reachable_indices: 3,
            known_reachable_indices: 3,
            open_reachable_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "when asking for an object in the smallest pack, all inbetween packs are also loaded."
    );

    assert!(!new_handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));
    assert_eq!(
        new_handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 2,
            num_refreshes: 2,
            open_reachable_indices: 3,
            known_reachable_indices: 3,
            open_reachable_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "trigger refreshes each time there is an object miss"
    );

    new_handle.refresh_never();
    assert!(!new_handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));
    assert_eq!(
        new_handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 2,
            num_refreshes: 2,
            open_reachable_indices: 3,
            known_reachable_indices: 3,
            open_reachable_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "if no refreshes are allowed, there is no additional refresh"
    );
}

#[test]
fn lookup() {
    let mut handle = db();

    fn can_locate(db: &git_odb::Handle, hex_id: &str) {
        let id = hex_to_id(hex_id);
        let mut buf = Vec::new();
        assert!(db.find(id, &mut buf).is_ok());
        assert!(db.contains(id));
    }
    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 0,
            open_reachable_indices: 0,
            known_reachable_indices: 0,
            open_reachable_packs: 0,
            known_packs: 0,
            unused_slots: 32,
            loose_dbs: 0,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "nothing happened yet, the store is totally lazy"
    );

    // pack (in sort order inherent to the store)
    can_locate(&handle, "501b297447a8255d3533c6858bb692575cdefaa0"); // pack 11fd
    can_locate(&handle, "4dac9989f96bc5b5b1263b582c08f0c5f0b58542"); // pack a2bf
    can_locate(&handle, "dd25c539efbb0ab018caa4cda2d133285634e9b5"); // pack c043

    let mut all_loaded = git_odb::store::Metrics {
        num_handles: 1,
        num_refreshes: 1,
        open_reachable_indices: 3,
        known_reachable_indices: 3,
        open_reachable_packs: 3,
        known_packs: 3,
        unused_slots: 29,
        loose_dbs: 1,
        unreachable_indices: 0,
        unreachable_packs: 0,
    };
    assert_eq!(
        handle.store_ref().metrics(),
        all_loaded,
        "all packs and indices are loaded"
    );

    // Loose
    can_locate(&handle, "37d4e6c5c48ba0d245164c4e10d5f41140cab980");

    assert!(matches!(
        handle.refresh_mode(),
        store::RefreshMode::AfterAllIndicesLoaded
    ));
    assert!(!handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));

    all_loaded.num_refreshes += 1;
    assert_eq!(
        handle.store_ref().metrics(),
        all_loaded,
        "it tried to refresh once to see if the missing object is there then"
    );

    handle.refresh_never();
    let previous_refresh_count = all_loaded.num_refreshes;
    assert!(!handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));
    assert_eq!(
        handle.store_ref().metrics().num_refreshes,
        previous_refresh_count,
        "it didn't try to refresh the on-disk state after failing to find the object."
    );
}

mod lookup_prefix {
    use crate::store::dynamic::db_with_all_object_sources;
    use git_testtools::hex_to_id;

    #[test]
    fn returns_none_for_prefixes_without_any_match() {
        let (handle, _tmp) = db_with_all_object_sources().unwrap();
        let prefix = git_hash::Prefix::new(git_hash::ObjectId::null(git_hash::Kind::Sha1), 7).unwrap();
        assert!(handle.lookup_prefix(prefix).unwrap().is_none());
    }

    #[test]
    fn returns_some_err_for_prefixes_with_more_than_one_match() {
        let (store, _tmp) = db_with_all_object_sources().unwrap();
        let prefix = git_hash::Prefix::new(hex_to_id("a7065b5e971a6d8b55875d8cf634a3a37202ab23"), 4).unwrap();
        assert_eq!(
            store.lookup_prefix(prefix).unwrap(),
            Some(Err(())),
            "there are two objects with that prefix"
        );
    }

    #[test]
    fn iterable_objects_can_be_looked_up_with_varying_prefix_lengths() {
        let (store, _tmp) = db_with_all_object_sources().unwrap();
        let hex_lengths = &[5, 7, 40];
        for (index, oid) in store.iter().unwrap().map(Result::unwrap).enumerate() {
            let hex_len = hex_lengths[index % hex_lengths.len()];
            let prefix = git_hash::Prefix::new(oid, hex_len).unwrap();
            assert_eq!(
                store
                    .lookup_prefix(prefix)
                    .unwrap()
                    .expect("object exists")
                    .expect("unambiguous"),
                oid
            );
        }
    }
}

#[test]
fn missing_objects_triggers_everything_is_loaded() {
    let handle = db();
    assert!(!handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 2,
            open_reachable_indices: 3,
            known_reachable_indices: 3,
            open_reachable_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "first refresh triggered by on-disk check, second refresh triggered to see if something changed, contains() only sees indices"
    );

    let mut buf = Vec::new();
    assert!(!handle
        .find(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), &mut buf)
        .is_ok());

    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 3,
            open_reachable_indices: 3,
            known_reachable_indices: 3,
            open_reachable_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "there are still no packs opened as no index contained the object"
    );
}

#[test]
fn iterate_over_a_bunch_of_loose_and_packed_objects() -> crate::Result {
    let (db, _tmp) = db_with_all_object_sources()?;
    let iter = db.iter()?;
    assert_eq!(
        iter.size_hint(),
        (139, None),
        "we only count packs and have no upper bound"
    );
    assert_eq!(iter.count(), 146, "it sees the correct amount of objects");
    for id in db.iter()? {
        assert!(db.contains(id?), "each object exists");
    }
    Ok(())
}

#[test]
fn auto_refresh_with_and_without_id_stability() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    assert!(
        Command::new("git")
            .arg("-C")
            .arg(tmp.path())
            .arg("init")
            .arg("--bare")
            .status()?
            .success(),
        "git should work"
    );
    git_testtools::copy_recursively_into_existing_dir(fixture_path("objects/pack"), tmp.path().join("objects/pack"))?;
    let hide_pack = |name: &str| {
        let stem = tmp.path().join("objects/pack").join(name);
        std::fs::rename(stem.with_extension("idx"), stem.with_extension("idx.bak")).unwrap();
        std::fs::rename(stem.with_extension("pack"), stem.with_extension("pack.bak")).unwrap();
    };
    let unhide_pack = |name: &str| {
        let stem = tmp.path().join("objects/pack").join(name);
        std::fs::rename(stem.with_extension("idx.bak"), stem.with_extension("idx")).unwrap();
        std::fs::rename(stem.with_extension("pack.bak"), stem.with_extension("pack")).unwrap();
    };
    hide_pack("pack-11fdfa9e156ab73caae3b6da867192221f2089c2");
    hide_pack("pack-a2bf8e71d8c18879e499335762dd95119d93d9f1");

    let handle = git_odb::at(tmp.path().join("objects"))?;
    let mut buf = Vec::new();
    assert!(
        handle
            .find(hex_to_id("dd25c539efbb0ab018caa4cda2d133285634e9b5"), &mut buf)
            .is_ok(),
        "can find object in existing pack at pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx"
    );
    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 1,
            open_reachable_indices: 1,
            known_reachable_indices: 1,
            open_reachable_packs: 1,
            known_packs: 1,
            unused_slots: 31,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "one pack was opened"
    );

    hide_pack("pack-c0438c19fb16422b6bbcce24387b3264416d485b");
    unhide_pack("pack-11fdfa9e156ab73caae3b6da867192221f2089c2");
    assert!(
        handle
            .find(hex_to_id("501b297447a8255d3533c6858bb692575cdefaa0"), &mut buf)
            .is_ok(),
        "now finding the object in the new pack"
    );
    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 2,
            open_reachable_indices: 1,
            known_reachable_indices: 1,
            open_reachable_packs: 1,
            known_packs: 1,
            unused_slots: 31,
            loose_dbs: 1,
            unreachable_indices: 0,
            unreachable_packs: 0
        },
        "the old pack was removed, the new was loaded"
    );

    {
        use git_pack::Find;
        let mut stable_handle = handle.clone();
        stable_handle.prevent_pack_unload();
        let location = stable_handle
            .location_by_oid(hex_to_id("501b297447a8255d3533c6858bb692575cdefaa0"), &mut buf)
            .expect("object exists");
        assert!(
            stable_handle.entry_by_location(&location).is_some(),
            "entries can be found by location as the pack is definitely still loaded, the index didn't change"
        );

        hide_pack("pack-11fdfa9e156ab73caae3b6da867192221f2089c2");
        unhide_pack("pack-a2bf8e71d8c18879e499335762dd95119d93d9f1");

        assert!(
            stable_handle
                .location_by_oid(hex_to_id("4dac9989f96bc5b5b1263b582c08f0c5f0b58542"), &mut buf)
                .is_some(),
            "it finds the object in the newly unhidden pack, which also triggers a refresh providing it with new indices"
        );
        assert_eq!(
            handle.store_ref().metrics(),
            git_odb::store::Metrics {
                num_handles: 2,
                num_refreshes: 3,
                open_reachable_indices: 1,
                known_reachable_indices: 1,
                open_reachable_packs: 1,
                known_packs: 1,
                unused_slots: 30,
                loose_dbs: 1,
                unreachable_indices: 1,
                unreachable_packs: 1
            },
            "the removed pack is still loaded"
        );
        assert!(
            stable_handle.entry_by_location(&location).is_some(),
            "it finds the old removed location (still loaded) on the old id, it's still cached in the handle, too"
        );
        assert!(
            stable_handle.clone().entry_by_location(&location).is_some(),
            "handles without any internal cache also work"
        );
    }

    hide_pack("pack-a2bf8e71d8c18879e499335762dd95119d93d9f1");
    unhide_pack("pack-c0438c19fb16422b6bbcce24387b3264416d485b");

    assert!(
        handle
            .find(hex_to_id("dd25c539efbb0ab018caa4cda2d133285634e9b5"), &mut buf)
            .is_ok(),
        "new pack is loaded, previously loaded is forgotten, lack of cache triggers refresh"
    );
    assert_eq!(
        handle.store_ref().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 4,
            open_reachable_indices: 1,
            known_reachable_indices: 1,
            open_reachable_packs: 1,
            known_packs: 1,
            unused_slots: 30,
            loose_dbs: 1,
            unreachable_indices: 1,
            unreachable_packs: 1
        },
        "garbaged slots aren't reclaimed until there is the need. Keeping indices open despite them not being accessible anymore."
    );
    Ok(())
}

mod verify {
    use std::sync::atomic::AtomicBool;

    use git_features::progress;
    use git_testtools::fixture_path;

    use crate::store::dynamic::db;

    #[test]
    fn integrity() {
        let handle = db();
        let outcome = handle
            .store_ref()
            .verify_integrity(progress::Discard, &AtomicBool::new(false), Default::default())
            .unwrap();
        assert_eq!(outcome.index_statistics.len(), 3, "there are only three packs to check");
        assert_eq!(
            outcome.index_statistics[0].path,
            fixture_path("objects/pack/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx")
        );
        assert_eq!(
            outcome.index_statistics[1].path,
            fixture_path("objects/pack/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx")
        );
        assert_eq!(
            outcome.index_statistics[2].path,
            fixture_path("objects/pack/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx")
        );
        assert_eq!(
            outcome.loose_object_stores,
            vec![git_odb::store::verify::integrity::LooseObjectStatistics {
                path: fixture_path("objects"),
                statistics: git_odb::loose::verify::integrity::Statistics { num_objects: 7 }
            }]
        );

        assert_eq!(
            handle.store_ref().metrics(),
            git_odb::store::Metrics {
                num_handles: 1,
                num_refreshes: 1,
                open_reachable_indices: 0,
                known_reachable_indices: 3,
                open_reachable_packs: 0,
                known_packs: 3,
                unused_slots: 29,
                loose_dbs: 1,
                unreachable_indices: 0,
                unreachable_packs: 0
            },
            "verification only discovers files on disk but won't cause them to be opened permanently"
        );
    }
}
