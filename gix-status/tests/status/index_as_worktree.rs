use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use bstr::BStr;
use filetime::{set_file_mtime, FileTime};
use gix_index as index;
use gix_index::Entry;
use gix_status::{
    index_as_worktree,
    index_as_worktree::{
        content::{CompareBlobs, FastEq, ReadDataOnce},
        Change, Options, Recorder,
    },
};

use crate::fixture_path;

// since tests are fixtures a bunch of stat information (like inode number)
// changes when extracting the data so we need to disable all advanced stat
// changes and only look at mtime seconds and file size to properly
// test all code paths (and to trigger racy git).
const TEST_OPTIONS: index::entry::stat::Options = index::entry::stat::Options {
    trust_ctime: false,
    check_stat: false,
    use_nsec: false,
    use_stdev: false,
};

fn fixture(name: &str, expected_status: &[(&BStr, Option<Change>, bool)]) {
    let worktree = fixture_path(name);
    let git_dir = worktree.join(".git");
    let mut index =
        gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, false, Default::default()).unwrap();
    let mut recorder = Recorder::default();
    index_as_worktree(
        &mut index,
        &worktree,
        &mut recorder,
        FastEq,
        |_, _| Ok::<_, std::convert::Infallible>(gix_object::BlobRef { data: &[] }),
        Options {
            fs: gix_fs::Capabilities::probe(&git_dir),
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
fn intent_to_add() {
    fixture(
        "status_intent_to_add",
        &[(BStr::new(b"content"), Some(Change::IntentToAdd), false)],
    );
}

#[test]
fn conflict() {
    fixture(
        "status_conflict",
        &[(
            BStr::new(b"content"),
            Some(Change::Modification {
                executable_bit_changed: false,
                content_change: Some(()),
            }),
            true,
        )],
    );
}

#[test]
fn unchanged() {
    fixture("status_unchanged", &[]);
}

#[test]
#[cfg_attr(
    windows,
    ignore = "needs work, on windows plenty of additional files are considered modified for some reason"
)]
fn modified() {
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
    // we need a writable fixture because we have to mess with `mtimes` manually, because touch -d
    // respects the locale so the test wouldn't work depending on the timezone you
    // run your test in.
    let dir = gix_testtools::scripted_fixture_writable_standalone("racy_git.sh").expect("script works");
    let worktree = dir.path();
    let git_dir = worktree.join(".git");
    let fs = gix_fs::Capabilities::probe(&git_dir);
    let mut index =
        gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, false, Default::default()).unwrap();

    #[derive(Clone)]
    struct CountCalls(Arc<AtomicUsize>, FastEq);
    impl CompareBlobs for CountCalls {
        type Output = ();

        fn compare_blobs<'a, E>(
            &mut self,
            entry: &'a Entry,
            worktree_blob_size: usize,
            worktree_blob: impl ReadDataOnce<'a, E>,
            entry_blob: impl ReadDataOnce<'a, E>,
        ) -> Result<Option<Self::Output>, E> {
            self.0.fetch_add(1, Ordering::Relaxed);
            self.1
                .compare_blobs(entry, worktree_blob_size, worktree_blob, entry_blob)
        }
    }

    // We artificially mess with the entry's `mtime` so that it's before the timestamp saved by git.
    // This would usually mean an invalid fs/invalid index file and as a result the racy git
    // mitigation doesn't work and the worktree shows up as unchanged even tough the file did
    // change.
    // This case doesn't happen in the realworld (except for file corruption) but
    // makes sure we are actually hitting the right codepath.
    index.entries_mut()[0].stat.mtime.secs = timestamp;
    set_file_mtime(worktree.join("content"), FileTime::from_unix_time(timestamp as i64, 0))
        .expect("changing filetime works");
    let mut recorder = Recorder::default();

    let count = Arc::new(AtomicUsize::new(0));
    let counter = CountCalls(count.clone(), FastEq);
    index_as_worktree(
        &mut index,
        worktree,
        &mut recorder,
        counter.clone(),
        |_, _| Err(std::io::Error::new(std::io::ErrorKind::Other, "no odb access expected")),
        Options {
            fs,
            stat: TEST_OPTIONS,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(count.load(Ordering::Relaxed), 0, "no blob content is accessed");
    assert_eq!(recorder.records, &[], "the testcase triggers racy git");

    // Now we also backdate the index timestamp to match the artificially created
    // mtime above this is now a realistic realworld race-condition which should trigger racy git
    // and cause proper output.
    index.set_timestamp(FileTime::from_unix_time(timestamp as i64, 0));
    let mut recorder = Recorder::default();
    index_as_worktree(
        &mut index,
        worktree,
        &mut recorder,
        counter,
        |_, _| Err(std::io::Error::new(std::io::ErrorKind::Other, "no odb access expected")),
        Options {
            fs,
            stat: TEST_OPTIONS,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(
        count.load(Ordering::Relaxed),
        1,
        "no we needed to access the blob content"
    );
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
