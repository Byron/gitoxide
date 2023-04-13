use bstr::BStr;
use filetime::{set_file_mtime, FileTime};
use gix_index as index;
use gix_worktree::fs::Capabilities;
use gix_worktree::index::status::content::FastEq;
use gix_worktree::index::status::worktree::{self, Options};
use gix_worktree::index::status::{Change, Recorder};

use crate::fixture_path;

// since tests are fixtures a bunch of stat information (like inode number)
// changes when extracting the data so we need to disable all advanced stat
// changes and only look at mtime seconds and file size to properly
// test all code paths (and to trigger racy git)
const TEST_OPTIONS: index::entry::stat::Options = index::entry::stat::Options {
    trust_ctime: false,
    check_stat: false,
    use_nsec: false,
    use_stdev: false,
};

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
            stat: TEST_OPTIONS,
            ..Options::default()
        },
    )
    .unwrap();
    recorder.records.sort_unstable_by_key(|(name, _, _)| *name);
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

#[test]
fn racy_git() {
    let timestamp = 940040400;
    // we need a writable fixture because we have to mess with mtimes manually,
    // because touch -d respects the locale so the test wouldn't work depending
    // on the timezone you run your test in
    let dir = gix_testtools::scripted_fixture_writable("racy_git.sh").expect("script works");
    let worktree = dir.path();
    let git_dir = worktree.join(".git");
    let fs = Capabilities::probe(&git_dir);
    let mut index = gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, Default::default()).unwrap();
    // we artificially mess with mtime so that it's before the timestamp
    // saved by git. This would usually mean an invalid fs/invalid index file
    // and as a result the racy git mitigation doesn't work and the worktree
    // shows up as unchanged even tough the file did change. This case
    // doesn't happen in the realworld (except for file corruption) but
    // makes sure we are actually hitting the right codepath
    index.entries[0].stat.mtime.secs = timestamp;
    set_file_mtime(worktree.join("content"), FileTime::from_unix_time(timestamp as i64, 0))
        .expect("changing filetime works");
    let mut recorder = Recorder::default();
    worktree::changes_to_obtain(
        &mut index,
        &worktree,
        &mut recorder,
        &FastEq,
        Options {
            fs,
            stat: TEST_OPTIONS,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(recorder.records, &[], "the testcase triggers racy git");

    // now we also backdate the index timestamp to match the artificially created
    // mtime above this is now a realistic realworld racecondition which
    // should trigger racy git and cause proper output
    index.set_timestamp(FileTime::from_unix_time(timestamp as i64, 0));
    let mut recorder = Recorder::default();
    worktree::changes_to_obtain(
        &mut index,
        &worktree,
        &mut recorder,
        &FastEq,
        Options {
            fs,
            stat: TEST_OPTIONS,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(
        recorder.records,
        &[(
            BStr::new(b"content"),
            Some(Change::Modification {
                executable_bit_changed: false,
                content_change: Some(()),
            }),
            false
        )],
        "racy change is correctly detected"
    );
}
