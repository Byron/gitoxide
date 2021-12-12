#![allow(unused)]
use git_features::threading::OwnShared;
use git_odb::{general, Find, FindExt};
use git_testtools::{fixture_path, hex_to_id};

fn db() -> git_odb::Handle {
    git_odb::at(fixture_path("objects")).expect("valid object path")
}

#[test]
#[ignore]
fn basics() {
    let handle = db();

    fn can_locate(db: &git_odb::Handle, hex_id: &str) {
        let id = hex_to_id(hex_id);
        let mut buf = Vec::new();
        assert!(db.contains(id));
        assert!(db.find(id, &mut buf).is_ok());
    }

    // Loose
    can_locate(&handle, "37d4e6c5c48ba0d245164c4e10d5f41140cab980");

    // Loose
    can_locate(&handle, "501b297447a8255d3533c6858bb692575cdefaa0"); // pack 11fd
    can_locate(&handle, "4dac9989f96bc5b5b1263b582c08f0c5f0b58542"); // pack a2bf
    can_locate(&handle, "dd25c539efbb0ab018caa4cda2d133285634e9b5"); // pack c043

    assert!(matches!(
        handle.inner.refresh_mode,
        git_odb::RefreshMode::AfterAllIndicesLoaded
    ));
    assert!(!handle.contains(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")));
}
