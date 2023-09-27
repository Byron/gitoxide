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
        traits::{CompareBlobs, FastEq, ReadDataOnce},
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

fn fixture(name: &str, expected_status: &[(&BStr, Option<Change>, bool)]) -> Outcome {
    fixture_filtered(name, &[], expected_status)
}

fn submodule_fixture(name: &str, expected_status: &[(&BStr, Option<Change>, bool)]) -> Outcome {
    fixture_filtered_detailed("status_submodule", name, &[], expected_status, false)
}

fn submodule_fixture_status(
    name: &str,
    expected_status: &[(&BStr, Option<Change>, bool)],
    submodule_dirty: bool,
) -> Outcome {
    fixture_filtered_detailed("status_submodule", name, &[], expected_status, submodule_dirty)
}

fn fixture_filtered(name: &str, pathspecs: &[&str], expected_status: &[(&BStr, Option<Change>, bool)]) -> Outcome {
    fixture_filtered_detailed(name, "", pathspecs, expected_status, false)
}

fn fixture_filtered_detailed(
    name: &str,
    subdir: &str,
    pathspecs: &[&str],
    expected_status: &[(&BStr, Option<Change>, bool)],
    submodule_dirty: bool,
) -> Outcome {
    let worktree = fixture_path(name).join(subdir);
    let git_dir = worktree.join(".git");
    let mut index =
        gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, false, Default::default()).unwrap();
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

fn records_to_tuple<'index>(
    records: impl IntoIterator<Item = Record<'index, (), ()>>,
) -> Vec<(&'index BStr, Option<Change>, bool)> {
    records
        .into_iter()
        .map(|r| (r.relative_path, r.change, r.conflict))
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
            (BStr::new(b"dir/content"), Some(Change::Removed), NO_CONFLICT),
            (BStr::new(b"dir/sub-dir/symlink"), Some(Change::Removed), NO_CONFLICT),
            (BStr::new(b"empty"), Some(Change::Removed), NO_CONFLICT),
            (BStr::new(b"executable"), Some(Change::Removed), NO_CONFLICT),
        ],
    );
    assert_eq!(
        out,
        Outcome {
            symlink_metadata_calls: 4,
            ..Default::default()
        }
    );

    let out = fixture_filtered(
        "status_removed",
        &["dir"],
        &[
            (BStr::new(b"dir/content"), Some(Change::Removed), NO_CONFLICT),
            (BStr::new(b"dir/sub-dir/symlink"), Some(Change::Removed), NO_CONFLICT),
        ],
    );
    assert_eq!(
        out,
        Outcome {
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
            &[(BStr::new(b"m1"), Some(Change::Removed), NO_CONFLICT)]
        )),
        Outcome {
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
            &[(BStr::new(b"m1"), Some(Change::Type), NO_CONFLICT)]
        )),
        Outcome {
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
            &[(BStr::new(b"m1"), Some(Change::SubmoduleModification(())), NO_CONFLICT)],
            true,
        )),
        Outcome {
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
            &[(BStr::new(b"content"), Some(Change::IntentToAdd), NO_CONFLICT)],
        ),
        Outcome {
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
                Some(Change::Modification {
                    executable_bit_changed: false,
                    content_change: Some(()),
                }),
                true,
            )],
        ),
        Outcome {
            symlink_metadata_calls: 1,
            worktree_files_read: 1,
            worktree_bytes: 51,
            ..Default::default()
        }
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
    assert_eq!(
        fixture(
            "status_changed",
            &[
                (
                    BStr::new(b"dir/content"),
                    Some(Change::Modification {
                        executable_bit_changed: true,
                        content_change: None,
                    }),
                    NO_CONFLICT,
                ),
                (
                    BStr::new(b"dir/content2"),
                    Some(Change::Modification {
                        executable_bit_changed: false,
                        content_change: Some(()),
                    }),
                    NO_CONFLICT,
                ),
                (BStr::new(b"empty"), Some(Change::Type), NO_CONFLICT),
                (
                    BStr::new(b"executable"),
                    Some(Change::Modification {
                        executable_bit_changed: true,
                        content_change: Some(()),
                    }),
                    NO_CONFLICT,
                ),
            ],
        ),
        Outcome {
            symlink_metadata_calls: 5,
            entries_updated: 1,
            worktree_files_read: 2,
            worktree_bytes: 23,
            racy_clean: 1,
            ..Default::default()
        }
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
            worktree_blob_size: usize,
            worktree_blob: impl ReadDataOnce<'a>,
            entry_blob: impl ReadDataOnce<'b>,
        ) -> Result<Option<Self::Output>, gix_status::index_as_worktree::Error> {
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
    let out = index_as_worktree(
        &mut index,
        worktree,
        &mut recorder,
        counter.clone(),
        SubmoduleStatusMock { dirty: false },
        |_, _| Err(std::io::Error::new(std::io::ErrorKind::Other, "no odb access expected")),
        &mut gix_features::progress::Discard,
        Pathspec::default(),
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
