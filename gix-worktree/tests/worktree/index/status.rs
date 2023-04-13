use bstr::BStr;
use gix_worktree::fs::{self, Capabilities};
use gix_worktree::index::status::content::FastEq;
use gix_worktree::index::status::worktree::{self, Options};
use gix_worktree::index::status::{Change, Recorder};

use crate::fixture_path;

fn fixture(name: &str, expected_status: &[(&BStr, Option<Change>, bool)]) {
    let worktree = fixture_path(name);
    let git_dir = worktree.join(".git");
    let mut index = gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, Default::default()).unwrap();
    let mut recorder = Recorder::default();
    worktree::changes_to_obtain(
        &mut index,
        &worktree,
        &mut recorder,
        &FastEq,
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
            (BStr::new(b"dir/content"), Some(Change::Removed), false),
            (BStr::new(b"dir/sub-dir/symlink"), Some(Change::Removed), false),
            (BStr::new(b"empty"), Some(Change::Removed), false),
            (BStr::new(b"executable"), Some(Change::Removed), false),
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
                Some(Change::Modification {
                    executable_bit_changed: true,
                    content_change: None,
                }),
                false,
            ),
            (
                BStr::new(b"dir/content2"),
                Some(Change::Modification {
                    executable_bit_changed: false,
                    content_change: Some(()),
                }),
                false,
            ),
            (BStr::new(b"empty"), Some(Change::Type), false),
            (
                BStr::new(b"executable"),
                Some(Change::Modification {
                    executable_bit_changed: true,
                    content_change: Some(()),
                }),
                false,
            ),
        ],
    );
    fixture(
        "status_changed",
        &[
            (
                BStr::new(b"dir/content"),
                Some(Change::Modification {
                    executable_bit_changed: true,
                    content_change: None,
                }),
                false,
            ),
            (
                BStr::new(b"dir/content2"),
                Some(Change::Modification {
                    executable_bit_changed: false,
                    content_change: Some(()),
                }),
                false,
            ),
            (BStr::new(b"empty"), Some(Change::Type), false),
            (
                BStr::new(b"executable"),
                Some(Change::Modification {
                    executable_bit_changed: true,
                    content_change: Some(()),
                }),
                false,
            ),
        ],
    );
}
