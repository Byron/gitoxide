#![allow(unused)]

use std::process::Command;

use git_features::threading::OwnShared;
use git_odb::{store, Find, FindExt, Write};
use git_testtools::{fixture_path, hex_to_id};

fn db() -> git_odb::Handle {
    git_odb::at(fixture_path("objects")).expect("valid object path")
}

#[test]
#[ignore]
fn multi_index_access() {
    let dir = git_testtools::scripted_fixture_repo_writable("make_repo_multi_index.sh").unwrap();
    let handle = git_odb::at(dir.path().join(".git/objects")).unwrap();

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
            unreachable_indices: 0
        },
        "it starts out knowing nothing, it's completely lazy"
    );

    let mut count = 0;
    let mut buf = Vec::new();
    for oid in handle.iter().unwrap() {
        let oid = oid.unwrap();
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
            unreachable_indices: 0
        },
        "it opened only a single multi-index and its pack - hard to see it's actually a multi-index as it's just one index anyway…"
    );

    let non_existing_to_trigger_refresh = hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    handle.contains(non_existing_to_trigger_refresh);

    filetime::set_file_mtime(
        handle.store_ref().path().join("pack/multi-pack-index"),
        filetime::FileTime::now(),
    )
    .unwrap();
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
            unreachable_indices: 0
        },
        "everything seems to remain as it was, even though we moved our multi-index to a new slot and removed the old one"
    );
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
    let mut handle = db();

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
            unreachable_indices: 0
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
            unreachable_indices: 0
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
            unreachable_indices: 0
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
            unreachable_indices: 0
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
            unreachable_indices: 0
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
            unreachable_indices: 0
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
            unreachable_indices: 0
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
            unreachable_indices: 0
        },
        "there are still no packs opened as no index contained the object"
    );
}

#[test]
fn a_bunch_of_loose_and_packed_objects() -> crate::Result {
    let db = db();
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
            unreachable_indices: 0
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
            unreachable_indices: 0
        },
        "the old pack was removed, the new was loaded"
    );

    {
        use git_pack::{Find, FindExt};
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
                unreachable_indices: 1
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
            unreachable_indices: 1
        },
        "garbaged slots aren't reclaimed until there is the need. Keeping indices open despite them not being accessible anymore."
    );
    Ok(())
}
