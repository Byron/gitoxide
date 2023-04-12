use bstr::BStr;
use gix_worktree::fs::{self, Capabilities};
use gix_worktree::index::status::diff::Fast;
use gix_worktree::index::status::recorder::Recorder;
use gix_worktree::index::status::worktree::{self, Options};
use gix_worktree::index::status::Status;

use crate::fixture_path;

fn fixture(name: &str, expected_status: &[(&BStr, Status, bool)]) {
    let worktree = fixture_path(name);
    let git_dir = worktree.join(".git");
    let mut index = gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, Default::default()).unwrap();
    let mut recorder = Recorder::default();
    worktree::status(
        &mut index,
        &worktree,
        &mut recorder,
        &Fast,
        Options {
            fs: Capabilities::probe(git_dir),
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(recorder.records, expected_status)
}

#[test]
fn removed() {
    fixture(
        "status_removed",
        &[
            (BStr::new(b"dir/content"), Status::Removed, false),
            (BStr::new(b"dir/sub-dir/symlink"), Status::Removed, false),
            (BStr::new(b"empty"), Status::Removed, false),
            (BStr::new(b"executable"), Status::Removed, false),
        ],
    );
}

#[test]
fn unchanged() {
    fixture("status_unchanged", &[]);
}
#[test]
fn modified() {
    // run the same status check twice to ensure that racy detection
    // doesn't change the result of the status check
    fixture(
        "status_changed",
        &[
            (
                BStr::new(b"dir/content"),
                Status::Modified {
                    executable_bit_changed: true,
                    diff: None,
                },
                false,
            ),
            (
                BStr::new(b"dir/content2"),
                Status::Modified {
                    executable_bit_changed: false,
                    diff: Some(()),
                },
                false,
            ),
            (BStr::new(b"empty"), Status::TypeChange, false),
            (
                BStr::new(b"executable"),
                Status::Modified {
                    executable_bit_changed: true,
                    diff: Some(()),
                },
                false,
            ),
        ],
    );
    fixture(
        "status_changed",
        &[
            (
                BStr::new(b"dir/content"),
                Status::Modified {
                    executable_bit_changed: true,
                    diff: None,
                },
                false,
            ),
            (
                BStr::new(b"dir/content2"),
                Status::Modified {
                    executable_bit_changed: false,
                    diff: Some(()),
                },
                false,
            ),
            (BStr::new(b"empty"), Status::TypeChange, false),
            (
                BStr::new(b"executable"),
                Status::Modified {
                    executable_bit_changed: true,
                    diff: Some(()),
                },
                false,
            ),
        ],
    );
}
