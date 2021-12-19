#![allow(unused)]
use git_features::threading::OwnShared;
use git_odb::{store, Find, FindExt, Write};
use git_testtools::{fixture_path, hex_to_id};

fn db() -> git_odb::Handle {
    git_odb::at(fixture_path("objects")).expect("valid object path")
}

#[test]
fn write() -> crate::Result {
    let dir = tempfile::tempdir()?;
    let mut handle = git_odb::at(dir.path())?;
    // It should refresh once even if the refresh mode is never, just to initialize the index
    handle.inner.refresh_mode = store::RefreshMode::Never;

    let written_id = handle.write_buf(git_object::Kind::Blob, b"hello world", git_hash::Kind::Sha1)?;
    assert_eq!(written_id, hex_to_id("95d09f2b10159347eece71399a7e2e907ea3df4f"));
    Ok(())
}

#[test]
fn contains() {
    let mut handle = db();

    assert!(handle.contains(hex_to_id("37d4e6c5c48ba0d245164c4e10d5f41140cab980"))); // loose object
    assert_eq!(
        handle.inner.store().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 1,
            open_indices: 0,
            known_indices: 3,
            open_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1
        },
        "it only refreshed the file list, yielding the loose db to find this object, but no pack was opened yet"
    );

    // pack c043, the biggest one
    assert!(handle.contains(hex_to_id("dd25c539efbb0ab018caa4cda2d133285634e9b5")));

    assert_eq!(
        handle.inner.store().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 1,
            open_indices: 1,
            known_indices: 3,
            open_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1
        },
        "it loaded the biggest back only, which is the first in the list"
    );

    // pack, the smallest one
    // The new handle should make no difference.
    #[allow(clippy::redundant_clone)]
    let mut new_handle = handle.clone();
    assert!(new_handle.contains(hex_to_id("501b297447a8255d3533c6858bb692575cdefaa0")));
    assert_eq!(
        new_handle.inner.store().metrics(),
        git_odb::store::Metrics {
            num_handles: 2,
            num_refreshes: 1,
            open_indices: 3,
            known_indices: 3,
            open_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1
        },
        "when asking for an object in the smallest pack, all inbetween packs are also loaded."
    );

    assert!(!new_handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));
    assert_eq!(
        new_handle.inner.store().metrics(),
        git_odb::store::Metrics {
            num_handles: 2,
            num_refreshes: 2,
            open_indices: 3,
            known_indices: 3,
            open_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1
        },
        "trigger refreshes each time there is an object miss"
    );

    new_handle.inner.refresh_mode = store::RefreshMode::Never;
    assert!(!new_handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));
    assert_eq!(
        new_handle.inner.store().metrics(),
        git_odb::store::Metrics {
            num_handles: 2,
            num_refreshes: 2,
            open_indices: 3,
            known_indices: 3,
            open_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1
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
        handle.inner.store().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 0,
            open_indices: 0,
            known_indices: 0,
            open_packs: 0,
            known_packs: 0,
            unused_slots: 32,
            loose_dbs: 0
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
        open_indices: 3,
        known_indices: 3,
        open_packs: 3,
        known_packs: 3,
        unused_slots: 29,
        loose_dbs: 1,
    };
    assert_eq!(
        handle.inner.store().metrics(),
        all_loaded,
        "all packs and indices are loaded"
    );

    // Loose
    can_locate(&handle, "37d4e6c5c48ba0d245164c4e10d5f41140cab980");

    assert!(matches!(
        handle.inner.refresh_mode,
        store::RefreshMode::AfterAllIndicesLoaded
    ));
    assert!(!handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));

    all_loaded.num_refreshes += 1;
    assert_eq!(
        handle.inner.store().metrics(),
        all_loaded,
        "it tried to refresh once to see if the missing object is there then"
    );

    handle.inner.refresh_mode = store::RefreshMode::Never;
    let previous_refresh_count = all_loaded.num_refreshes;
    assert!(!handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));
    assert_eq!(
        handle.inner.store().metrics().num_refreshes,
        previous_refresh_count,
        "it didn't try to refresh the on-disk state after failing to find the object."
    );
}

#[test]
fn missing_objects_triggers_everything_is_loaded() {
    let handle = db();
    assert!(!handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));

    assert_eq!(
        handle.inner.store().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 2,
            open_indices: 3,
            known_indices: 3,
            open_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1
        },
        "first refresh triggered by on-disk check, second refresh triggered to see if something changed, contains() only sees indices"
    );

    let mut buf = Vec::new();
    assert!(!handle
        .find(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), &mut buf)
        .is_ok());

    assert_eq!(
        handle.inner.store().metrics(),
        git_odb::store::Metrics {
            num_handles: 1,
            num_refreshes: 3,
            open_indices: 3,
            known_indices: 3,
            open_packs: 0,
            known_packs: 3,
            unused_slots: 29,
            loose_dbs: 1
        },
        "there are still no packs opened as no index contained the object"
    );
}

#[test]
fn a_bunch_of_loose_and_packed_objects() -> crate::Result {
    let db = db();
    let iter = db.inner.iter()?;
    assert_eq!(
        iter.size_hint(),
        (139, None),
        "we only count packs and have no upper bound"
    );
    assert_eq!(iter.count(), 146, "it sees the correct amount of objects");
    for id in db.inner.iter()? {
        assert!(db.contains(id?), "each object exists");
    }
    Ok(())
}
