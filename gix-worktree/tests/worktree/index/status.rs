use bstr::BStr;
use gix_worktree::fs::{self, Capabilities};
use gix_worktree::index::status::recorder::worktree::Recorder;
use gix_worktree::index::status::visit::{worktree, ModeChange, Modification};
use gix_worktree::index::status::{worktree::Options, IndexStatus};

use crate::fixture_path;

fn fixture(name: &str, expected_status: &[(&BStr, worktree::Status, bool)]) {
    let worktree = fixture_path(name);
    let git_dir = worktree.join(".git");
    let index = gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, Default::default()).unwrap();
    let capabilities = fs::Capabilities::probe(&git_dir);
    let mut recorder = Recorder::new(&capabilities, &index);
    IndexStatus::from(&index).of_worktree(
        &worktree,
        &mut recorder,
        Options {
            fs: Capabilities::probe(git_dir),
            ..Options::default()
        },
    );
    // disable stat changed since this is not quite deterministic (can have false positives)
    for (_, change, _) in &mut recorder.records {
        if let worktree::Status::Modified(modification) = change {
            modification.stat_changed = false
        }
    }
    assert_eq!(recorder.records, expected_status)
}

#[test]
fn removed() {
    fixture(
        "status_removed",
        &[
            (BStr::new(b"dir/content"), worktree::Status::Removed, false),
            (BStr::new(b"dir/sub-dir/symlink"), worktree::Status::Removed, false),
            (BStr::new(b"empty"), worktree::Status::Removed, false),
            (BStr::new(b"executable"), worktree::Status::Removed, false),
        ],
    );
}

#[test]
fn unchanged() {
    fixture("status_unchanged", &[]);
}
#[test]
fn modified() {
    fixture(
        "status_changed",
        &[
            (
                BStr::new(b"dir/content"),
                worktree::Status::Modified(Modification {
                    mode_change: Some(ModeChange::ExecutableChange),
                    stat_changed: false,
                    data_changed: false,
                }),
                false,
            ),
            (
                BStr::new(b"dir/content2"),
                worktree::Status::Modified(Modification {
                    mode_change: Some(ModeChange::ExecutableChange),
                    stat_changed: false,
                    data_changed: true,
                }),
                false,
            ),
            (
                BStr::new(b"empty"),
                worktree::Status::Modified(Modification {
                    mode_change: Some(ModeChange::TypeChange),
                    stat_changed: false,
                    data_changed: true,
                }),
                false,
            ),
        ],
    );
}
