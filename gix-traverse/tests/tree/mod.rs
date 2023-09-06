use gix_object::bstr::BString;
use gix_odb::pack::FindExt;
use gix_traverse::{tree, tree::recorder::Location};

use crate::hex_to_id;

fn db() -> crate::Result<gix_odb::Handle> {
    let dir = gix_testtools::scripted_fixture_read_only_standalone("make_traversal_repo_for_trees.sh")?;
    let db = gix_odb::at(dir.join(".git").join("objects"))?;
    Ok(db)
}

#[test]
fn breadth_first_full_path() -> crate::Result<()> {
    let db = db()?;
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();
    let mut commit = db
        .find_commit_iter(&hex_to_id("85df34aa34848b8138b2b3dcff5fb5c2b734e0ce"), &mut buf)?
        .0;
    // Full paths - that's the default.
    let mut recorder = tree::Recorder::default();
    gix_traverse::tree::breadthfirst(
        db.find_tree_iter(&commit.tree_id().expect("a tree is available in a commit"), &mut buf2)?
            .0,
        tree::breadthfirst::State::default(),
        |oid, buf| db.find_tree_iter(oid, buf).ok().map(|t| t.0),
        &mut recorder,
    )?;

    use gix_object::tree::EntryMode::*;
    use gix_traverse::tree::recorder::Entry;
    assert_eq!(
        recorder.records,
        vec![
            Entry {
                mode: Blob,
                filepath: "a".into(),
                oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            Entry {
                mode: Blob,
                filepath: "b".into(),
                oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            Entry {
                mode: Blob,
                filepath: "c".into(),
                oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            Entry {
                mode: Tree,
                filepath: "d".into(),
                oid: hex_to_id("496d6428b9cf92981dc9495211e6e1120fb6f2ba")
            },
            Entry {
                mode: Tree,
                filepath: "e".into(),
                oid: hex_to_id("4277b6e69d25e5efa77c455340557b384a4c018a")
            },
            Entry {
                mode: Tree,
                filepath: "f".into(),
                oid: hex_to_id("70fb16fc77b03e16acb4a5b1a6caf79ba302919a")
            },
            Entry {
                mode: Blob,
                filepath: "d/a".into(),
                oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            Entry {
                mode: Blob,
                filepath: "e/b".into(),
                oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            Entry {
                mode: Blob,
                filepath: "f/c".into(),
                oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            Entry {
                mode: Tree,
                filepath: "f/d".into(),
                oid: hex_to_id("5805b676e247eb9a8046ad0c4d249cd2fb2513df")
            },
            Entry {
                mode: Blob,
                filepath: "f/z".into(),
                oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            },
            Entry {
                mode: Blob,
                filepath: "f/d/x".into(),
                oid: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
            }
        ]
    );
    Ok(())
}

#[test]
fn breadth_first_filename_only() -> crate::Result<()> {
    let db = db()?;
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();
    let mut commit = db
        .find_commit_iter(&hex_to_id("85df34aa34848b8138b2b3dcff5fb5c2b734e0ce"), &mut buf)?
        .0;
    let mut recorder = tree::Recorder::default().track_location(Some(Location::FileName));
    gix_traverse::tree::breadthfirst(
        db.find_tree_iter(&commit.tree_id().expect("a tree is available in a commit"), &mut buf2)?
            .0,
        tree::breadthfirst::State::default(),
        |oid, buf| db.find_tree_iter(oid, buf).ok().map(|t| t.0),
        &mut recorder,
    )?;

    assert_eq!(
        recorder.records.into_iter().map(|e| e.filepath).collect::<Vec<_>>(),
        ["a", "b", "c", "d", "e", "f", "a", "b", "c", "d", "z", "x"]
            .into_iter()
            .map(BString::from)
            .collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn breadth_first_no_location() -> crate::Result<()> {
    let db = db()?;
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();
    let mut commit = db
        .find_commit_iter(&hex_to_id("85df34aa34848b8138b2b3dcff5fb5c2b734e0ce"), &mut buf)?
        .0;
    let mut recorder = tree::Recorder::default().track_location(None);
    gix_traverse::tree::breadthfirst(
        db.find_tree_iter(&commit.tree_id().expect("a tree is available in a commit"), &mut buf2)?
            .0,
        tree::breadthfirst::State::default(),
        |oid, buf| db.find_tree_iter(oid, buf).ok().map(|t| t.0),
        &mut recorder,
    )?;

    for path in recorder.records.into_iter().map(|e| e.filepath) {
        assert_eq!(path, "", "path should be empty as it's not tracked at all")
    }
    Ok(())
}
