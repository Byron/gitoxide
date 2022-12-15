use git_odb::pack::FindExt;
use git_traverse::tree;

use crate::hex_to_id;

fn db() -> crate::Result<git_odb::Handle> {
    let dir = git_testtools::scripted_fixture_read_only("make_traversal_repo_for_trees.sh")?;
    let db = git_odb::at(dir.join(".git").join("objects"))?;
    Ok(db)
}

#[test]
fn basic_nesting() -> crate::Result<()> {
    let db = db()?;
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();
    let mut commit = db
        .find_commit_iter(hex_to_id("85df34aa34848b8138b2b3dcff5fb5c2b734e0ce"), &mut buf)?
        .0;
    let mut recorder = tree::Recorder::default();
    git_traverse::tree::breadthfirst(
        db.find_tree_iter(commit.tree_id().expect("a tree is available in a commit"), &mut buf2)?
            .0,
        tree::breadthfirst::State::default(),
        |oid, buf| db.find_tree_iter(oid, buf).ok().map(|t| t.0),
        &mut recorder,
    )?;

    use git_object::tree::EntryMode::*;
    use git_traverse::tree::recorder::Entry;
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
