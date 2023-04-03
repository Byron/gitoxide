use std::fs::{self};
use std::path::Path;

use bstr::BString;
use gix_worktree as worktree;
use worktree::diff::{ChangeKind, FileModification};

fn compute_diff(name: &str, make_worktree_dirty: impl FnOnce(&Path)) -> Vec<(ChangeKind, BString)> {
    let work_tree =
        gix_testtools::scripted_fixture_writable(Path::new(name).with_extension("sh")).expect("script works");
    let git_dir = work_tree.path().join(".git");
    make_worktree_dirty(work_tree.path());
    let index = gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, Default::default()).unwrap();
    let capapilites = worktree::fs::Capabilities::probe(git_dir);
    let mut buf = Vec::with_capacity(8 * 1024);
    worktree::diff::compare_to_index(&index, work_tree.path(), &capapilites)
        .filter_map(|change| {
            let mut change = change.unwrap();
            if let ChangeKind::Modified {
                ref mut modification, ..
            } = &mut change.kind
            {
                modification
                    .compare_data(&change.worktree_path, change.entry, &mut buf, &capapilites)
                    .unwrap();
                if modification.mode_change.is_none() && !modification.data_changed {
                    return None;
                }
            }
            Some((change.kind, change.entry.path(&index).to_owned()))
        })
        .collect()
}

#[test]
fn removed() {
    let diff = compute_diff("make_mixed_without_submodules", |path| {
        fs::remove_file(path.join("executable")).unwrap();
        fs::remove_file(path.join("dir/content")).unwrap();
        fs::remove_file(path.join("dir/sub-dir/symlink")).unwrap();
    });

    assert_eq!(
        diff,
        vec![
            (ChangeKind::Removed, BString::new(b"dir/content".to_vec())),
            (ChangeKind::Removed, BString::new(b"dir/sub-dir/symlink".to_vec())),
            (ChangeKind::Removed, BString::new(b"executable".to_vec())),
        ]
    )
}

#[test]
fn changed() {
    let diff = compute_diff("make_mixed_without_submodules", |path| {
        fs::write(path.join("dir/content"), "hello_world").unwrap();
        // write same content to this file to simulate a touch command
        fs::write(path.join("executable"), "content").unwrap();
    });

    assert_eq!(
        diff,
        vec![(
            ChangeKind::Modified {
                modification: FileModification {
                    mode_change: None,
                    stat_changed: true,
                    data_changed: true
                },
                conflict: false
            },
            BString::new(b"dir/content".to_vec())
        ),]
    )
}
