use std::sync::atomic::AtomicBool;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use bstr::BStr;
use filetime::{set_file_mtime, FileTime};
use gix_index as index;
use gix_index::Entry;
use gix_status::index_as_worktree::traits::SubmoduleStatus;
use gix_status::index_as_worktree::{Outcome, Record};
use gix_status::{
    index_as_worktree,
    index_as_worktree::{
        traits::{CompareBlobs, FastEq, ReadData},
        Change as WorktreeChange, Options, Recorder,
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

type Change = WorktreeChange<(), ()>;
type Expectation<'a> = (&'a BStr, usize, Option<Change>, bool);

fn fixture(name: &str, expected_status: &[Expectation<'_>]) -> Outcome {
    fixture_filtered(name, &[], expected_status)
}

fn fixture_with_index(
    name: &str,
    prepare_index: impl FnMut(&mut gix_index::State),
    expected_status: &[Expectation<'_>],
) -> Outcome {
    fixture_filtered_detailed(name, "", &[], expected_status, prepare_index, false)
}

fn submodule_fixture(name: &str, expected_status: &[Expectation<'_>]) -> Outcome {
    fixture_filtered_detailed("status_submodule", name, &[], expected_status, |_| {}, false)
}

fn submodule_fixture_status(name: &str, expected_status: &[Expectation<'_>], submodule_dirty: bool) -> Outcome {
    fixture_filtered_detailed("status_submodule", name, &[], expected_status, |_| {}, submodule_dirty)
}

fn fixture_filtered(name: &str, pathspecs: &[&str], expected_status: &[Expectation<'_>]) -> Outcome {
    fixture_filtered_detailed(name, "", pathspecs, expected_status, |_| {}, false)
}

fn fixture_filtered_detailed(
    name: &str,
    subdir: &str,
    pathspecs: &[&str],
    expected_status: &[Expectation<'_>],
    mut prepare_index: impl FnMut(&mut gix_index::State),
    submodule_dirty: bool,
) -> Outcome {
    let worktree = fixture_path(name).join(subdir);
    let git_dir = worktree.join(".git");
    let mut index =
        gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, false, Default::default()).unwrap();
    prepare_index(&mut index);
    let mut recorder = Recorder::default();
    let search = gix_pathspec::Search::from_specs(to_pathspecs(pathspecs), None, std::path::Path::new(""))
        .expect("valid specs can be normalized");
    let outcome = index_as_worktree(
        &mut index,
        &worktree,
        &mut recorder,
        FastEq,
        SubmoduleStatusMock { dirty: submodule_dirty },
        |_, _| Ok::<_, std::convert::Infallible>(gix_object::BlobRef { data: &[] }),
        &mut gix_features::progress::Discard,
        Pathspec(search),
        Default::default(),
        &AtomicBool::default(),
        Options {
            fs: gix_fs::Capabilities::probe(&git_dir),
            stat: TEST_OPTIONS,
            ..Options::default()
        },
    )
    .unwrap();
    recorder.records.sort_unstable_by_key(|r| r.relative_path);
    assert_eq!(records_to_tuple(recorder.records), expected_status);
    outcome
}

fn records_to_tuple<'index>(records: impl IntoIterator<Item = Record<'index, (), ()>>) -> Vec<Expectation<'index>> {
    records
        .into_iter()
        .map(|r| (r.relative_path, r.entry_index, r.change, r.conflict))
        .collect()
}

#[derive(Clone)]
struct SubmoduleStatusMock {
    dirty: bool,
}

impl SubmoduleStatus for SubmoduleStatusMock {
    type Output = ();
    type Error = std::convert::Infallible;

    fn status(&mut self, _entry: &Entry, _rela_path: &BStr) -> Result<Option<Self::Output>, Self::Error> {
        Ok(self.dirty.then_some(()))
    }
}

fn to_pathspecs(input: &[&str]) -> Vec<gix_pathspec::Pattern> {
    input
        .iter()
        .map(|pattern| gix_pathspec::parse(pattern.as_bytes(), Default::default()).expect("known to be valid"))
        .collect()
}

#[test]
fn removed() {
    let out = fixture(
        "status_removed",
        &[
            (BStr::new(b"dir/content"), 0, Some(Change::Removed), NO_CONFLICT),
            (BStr::new(b"dir/sub-dir/symlink"), 1, Some(Change::Removed), NO_CONFLICT),
            (BStr::new(b"empty"), 2, Some(Change::Removed), NO_CONFLICT),
            (BStr::new(b"executable"), 3, Some(Change::Removed), NO_CONFLICT),
        ],
    );
    assert_eq!(
        out,
        Outcome {
            entries_to_process: 4,
            entries_processed: 4,
            symlink_metadata_calls: 4,
            ..Default::default()
        }
    );

    let out = fixture_filtered(
        "status_removed",
        &["dir"],
        &[
            (BStr::new(b"dir/content"), 0, Some(Change::Removed), NO_CONFLICT),
            (BStr::new(b"dir/sub-dir/symlink"), 1, Some(Change::Removed), NO_CONFLICT),
        ],
    );
    assert_eq!(
        out,
        Outcome {
            entries_to_process: 2,
            entries_processed: 2,
            entries_skipped_by_common_prefix: 2,
            symlink_metadata_calls: 2,
            ..Default::default()
        }
    );
}

#[test]
fn subomdule_nochange() {
    assert_eq!(
        ignore_racyclean(submodule_fixture("no-change", &[])),
        Outcome {
            entries_to_process: 2,
            entries_processed: 2,
            entries_updated: 1,
            symlink_metadata_calls: 2,
            worktree_bytes: 46,
            worktree_files_read: 1,
            ..Default::default()
        }
    );
}

#[test]
fn subomdule_deleted_dir() {
    assert_eq!(
        ignore_racyclean(submodule_fixture(
            "deleted-dir",
            &[(BStr::new(b"m1"), 1, Some(Change::Removed), NO_CONFLICT)]
        )),
        Outcome {
            entries_to_process: 2,
            entries_processed: 2,
            entries_updated: 1,
            symlink_metadata_calls: 2,
            worktree_files_read: 1,
            worktree_bytes: 46,
            ..Default::default()
        }
    );
}

#[test]
fn subomdule_typechange() {
    assert_eq!(
        ignore_racyclean(submodule_fixture(
            "type-change",
            &[(BStr::new(b"m1"), 1, Some(Change::Type), NO_CONFLICT)]
        )),
        Outcome {
            entries_to_process: 2,
            entries_processed: 2,
            entries_updated: 1,
            symlink_metadata_calls: 2,
            worktree_files_read: 1,
            worktree_bytes: 46,
            ..Default::default()
        }
    )
}

#[test]
fn subomdule_empty_dir_no_change() {
    assert_eq!(
        ignore_racyclean(submodule_fixture("empty-dir-no-change", &[])),
        Outcome {
            entries_to_process: 2,
            entries_processed: 2,
            entries_updated: 1,
            symlink_metadata_calls: 2,
            worktree_files_read: 1,
            worktree_bytes: 46,
            ..Default::default()
        }
    );
}

#[test]
fn subomdule_empty_dir_no_change_is_passed_to_submodule_handler() {
    assert_eq!(
        ignore_racyclean(submodule_fixture_status(
            "empty-dir-no-change",
            &[(
                BStr::new(b"m1"),
                1,
                Some(Change::SubmoduleModification(())),
                NO_CONFLICT
            )],
            true,
        )),
        Outcome {
            entries_to_process: 2,
            entries_processed: 2,
            entries_updated: 1,
            symlink_metadata_calls: 2,
            worktree_files_read: 1,
            worktree_bytes: 46,
            ..Default::default()
        }
    );
}

#[test]
fn intent_to_add() {
    assert_eq!(
        fixture(
            "status_intent_to_add",
            &[(BStr::new(b"content"), 0, Some(Change::IntentToAdd), NO_CONFLICT)],
        ),
        Outcome {
            entries_to_process: 1,
            entries_processed: 1,
            symlink_metadata_calls: 1,
            ..Default::default()
        }
    );
}

#[test]
fn conflict() {
    assert_eq!(
        fixture(
            "status_conflict",
            &[(
                BStr::new(b"content"),
                0,
                Some(Change::Modification {
                    executable_bit_changed: false,
                    content_change: Some(()),
                }),
                true,
            )],
        ),
        Outcome {
            entries_to_process: 3,
            entries_processed: 3,
            symlink_metadata_calls: 1,
            worktree_files_read: 1,
            worktree_bytes: 51,
            ..Default::default()
        }
    );
}

#[test]
fn submodule_conflict() {
    assert_eq!(
        submodule_fixture("conflict", &[(BStr::new(b"m1"), 1, None, true)]),
        Outcome {
            entries_to_process: 3,
            entries_processed: 3,
            symlink_metadata_calls: 2,
            ..Default::default()
        },
        "submodule status is still called for OUR side of an entry, but never for THEIRS"
    );
}

#[test]
fn unchanged() {
    fixture("status_unchanged", &[]);
}

#[test]
fn refresh() {
    let expected_outcome = Outcome {
        entries_to_process: 5,
        entries_processed: 5,
        symlink_metadata_calls: 5,
        entries_updated: if cfg!(windows) { 2 } else { 1 },
        worktree_files_read: 4,
        worktree_bytes: if cfg!(windows) { 41 } else { 38 },
        ..Default::default()
    };
    assert_eq!(
        fixture_with_index(
            "status_changed",
            |index| { index.entries_mut().iter_mut().for_each(|e| e.stat = Default::default()) },
            #[cfg(not(windows))]
            &[
                (
                    BStr::new(b"dir/content"),
                    0,
                    Some(Change::Modification {
                        executable_bit_changed: true,
                        content_change: None,
                    }),
                    NO_CONFLICT,
                ),
                (
                    BStr::new(b"dir/content2"),
                    1,
                    Some(Change::Modification {
                        executable_bit_changed: false,
                        content_change: Some(()),
                    }),
                    NO_CONFLICT,
                ),
                (BStr::new(b"empty"), 3, Some(Change::Type), NO_CONFLICT),
                (
                    BStr::new(b"executable"),
                    4,
                    Some(Change::Modification {
                        executable_bit_changed: true,
                        content_change: Some(()),
                    }),
                    NO_CONFLICT,
                ),
            ],
            #[cfg(windows)]
            &[
                (
                    BStr::new("dir/content2"),
                    1,
                    Some(Change::Modification {
                        executable_bit_changed: false,
                        content_change: Some(())
                    }),
                    NO_CONFLICT
                ),
                (
                    BStr::new("empty"),
                    3,
                    Some(Change::Modification {
                        executable_bit_changed: false,
                        content_change: Some(())
                    }),
                    NO_CONFLICT
                ),
                (
                    BStr::new("executable"),
                    4,
                    Some(Change::Modification {
                        executable_bit_changed: false,
                        content_change: Some(())
                    }),
                    NO_CONFLICT
                )
            ],
        ),
        expected_outcome,
    );
}

#[test]
fn modified() {
    let expected_outcome = Outcome {
        entries_to_process: 5,
        entries_processed: 5,
        symlink_metadata_calls: 5,
        ..Default::default()
    };
    let actual_outcome = fixture(
        "status_changed",
        #[cfg(not(windows))]
        &[
            (
                BStr::new(b"dir/content"),
                0,
                Some(Change::Modification {
                    executable_bit_changed: true,
                    content_change: None,
                }),
                NO_CONFLICT,
            ),
            (
                BStr::new(b"dir/content2"),
                1,
                Some(Change::Modification {
                    executable_bit_changed: false,
                    content_change: Some(()),
                }),
                NO_CONFLICT,
            ),
            (BStr::new(b"empty"), 3, Some(Change::Type), NO_CONFLICT),
            (
                BStr::new(b"executable"),
                4,
                Some(Change::Modification {
                    executable_bit_changed: true,
                    content_change: Some(()),
                }),
                NO_CONFLICT,
            ),
        ],
        #[cfg(windows)]
        &[
            (
                BStr::new("dir/content2"),
                1,
                Some(Change::Modification {
                    executable_bit_changed: false,
                    content_change: Some(()),
                }),
                NO_CONFLICT,
            ),
            (
                BStr::new("empty"),
                3,
                Some(Change::Modification {
                    executable_bit_changed: false,
                    content_change: Some(()),
                }),
                NO_CONFLICT,
            ),
            (
                BStr::new("executable"),
                4,
                Some(Change::Modification {
                    executable_bit_changed: false,
                    content_change: Some(()),
                }),
                NO_CONFLICT,
            ),
        ],
    );
    assert_eq!(
        ignore_worktree_stats(ignore_updated(ignore_racyclean(actual_outcome))),
        expected_outcome,
    );
}

const NO_CONFLICT: bool = false;

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

        fn compare_blobs<'a, 'b>(
            &mut self,
            entry: &Entry,
            worktree_file_size: u64,
            data: impl ReadData<'a>,
            buf: &mut Vec<u8>,
        ) -> Result<Option<Self::Output>, gix_status::index_as_worktree::Error> {
            self.0.fetch_add(1, Ordering::Relaxed);
            self.1.compare_blobs(entry, worktree_file_size, data, buf)
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
    let out = index_as_worktree(
        &mut index,
        worktree,
        &mut recorder,
        counter.clone(),
        SubmoduleStatusMock { dirty: false },
        |_, _| Err(std::io::Error::new(std::io::ErrorKind::Other, "no odb access expected")),
        &mut gix_features::progress::Discard,
        Pathspec::default(),
        Default::default(),
        &AtomicBool::default(),
        Options {
            fs,
            stat: TEST_OPTIONS,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(
        out,
        Outcome {
            entries_to_process: 1,
            entries_processed: 1,
            symlink_metadata_calls: 1,
            ..Default::default()
        }
    );
    assert_eq!(count.load(Ordering::Relaxed), 0, "no blob content is accessed");
    assert_eq!(
        records_to_tuple(recorder.records),
        &[],
        "the testcase triggers racy git"
    );

    // Now we also backdate the index timestamp to match the artificially created
    // mtime above this is now a realistic realworld race-condition which should trigger racy git
    // and cause proper output.
    index.set_timestamp(FileTime::from_unix_time(timestamp as i64, 0));
    let mut recorder = Recorder::default();
    let out = index_as_worktree(
        &mut index,
        worktree,
        &mut recorder,
        counter,
        SubmoduleStatusMock { dirty: false },
        |_, _| Err(std::io::Error::new(std::io::ErrorKind::Other, "no odb access expected")),
        &mut gix_features::progress::Discard,
        Pathspec::default(),
        Default::default(),
        &AtomicBool::default(),
        Options {
            fs,
            stat: TEST_OPTIONS,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(
        out,
        Outcome {
            entries_to_process: 1,
            entries_processed: 1,
            symlink_metadata_calls: 1,
            racy_clean: 1,
            worktree_bytes: 3,
            worktree_files_read: 1,
            ..Default::default()
        }
    );
    assert_eq!(
        count.load(Ordering::Relaxed),
        1,
        "no we needed to access the blob content"
    );
    assert_eq!(
        records_to_tuple(recorder.records),
        &[(
            BStr::new(b"content"),
            0,
            Some(Change::Modification {
                executable_bit_changed: false,
                content_change: Some(()),
            }),
            false
        )],
        "racy change is correctly detected"
    );
}

#[derive(Clone)]
struct Pathspec(gix_pathspec::Search);

impl Default for Pathspec {
    fn default() -> Self {
        let search = gix_pathspec::Search::from_specs(to_pathspecs(&[]), None, std::path::Path::new(""))
            .expect("empty is always valid");
        Self(search)
    }
}

impl gix_status::Pathspec for Pathspec {
    fn common_prefix(&self) -> &BStr {
        self.0.common_prefix()
    }

    fn is_included(&mut self, relative_path: &BStr, is_dir: Option<bool>) -> bool {
        self.0
            .pattern_matching_relative_path(relative_path, is_dir, &mut |_, _, _, _| {
                unreachable!("we don't use attributes in our pathspecs")
            })
            .map_or(false, |m| !m.is_excluded())
    }
}

// This can easily happen in some fixtures, which can cause flakyness. It's time-dependent after all.
fn ignore_racyclean(mut out: Outcome) -> Outcome {
    out.racy_clean = 0;
    out
}

fn ignore_updated(mut out: Outcome) -> Outcome {
    out.entries_updated = 0;
    out
}

fn ignore_worktree_stats(mut out: Outcome) -> Outcome {
    out.worktree_bytes = 0;
    out.worktree_files_read = 0;
    out
}
