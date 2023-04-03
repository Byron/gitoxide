use std::convert::Infallible;

use gix::{
    bstr::BString,
    object::{blob::diff::line::Change, tree::diff::change::Event},
};
use gix_object::{bstr::ByteSlice, tree::EntryMode};

use crate::named_repo;

#[test]
fn changes_against_tree_modified() -> crate::Result {
    let repo = named_repo("make_diff_repo.sh")?;
    let from = tree_named(&repo, "@^{/c3-modification}~1");
    let to = tree_named(&repo, ":/c3-modification");
    from.changes()?
        .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
            assert_eq!(change.location, "", "without configuration the location field is empty");
            match change.event {
                Event::Modification {
                    previous_entry_mode,
                    previous_id,
                    entry_mode,
                    id,
                } => {
                    assert_eq!(previous_entry_mode, EntryMode::Blob);
                    assert_eq!(entry_mode, EntryMode::Blob);
                    assert_eq!(previous_id.object().unwrap().data.as_bstr(), "a\n");
                    assert_eq!(id.object().unwrap().data.as_bstr(), "a\na1\n");
                }
                Event::Rewrite { .. } | Event::Deletion { .. } | Event::Addition { .. } => {
                    unreachable!("only modification is expected")
                }
            };

            let diff = change.event.diff().expect("changed file").expect("objects available");
            let count = diff.line_counts();
            assert_eq!(count.insertions, 1);
            assert_eq!(count.removals, 0);
            diff.lines(|hunk| {
                match hunk {
                    Change::Deletion { .. } => unreachable!("there was no deletion"),
                    Change::Addition { lines } => assert_eq!(lines, vec!["a1".as_bytes().as_bstr()]),
                    Change::Modification { .. } => unreachable!("there was no modification"),
                };
                Ok::<_, Infallible>(())
            })
            .expect("infallible");
            Ok(Default::default())
        })?;
    Ok(())
}

#[test]
fn changes_against_tree_with_filename_tracking() -> crate::Result {
    let repo = named_repo("make_diff_repo.sh")?;
    let from = repo.empty_tree();
    let to = tree_named(&repo, ":/c1 - initial");

    let mut expected = vec!["a", "b", "c", "d"];
    from.changes()?
        .track_filename()
        .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
            expected.retain(|name| name != change.location);
            Ok(Default::default())
        })?;
    assert_eq!(expected, Vec::<&str>::new(), "all paths should have been seen");

    let mut expected = vec!["a", "b", "dir/c", "d"];
    from.changes()?
        .track_path()
        .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
            expected.retain(|name| name != change.location);
            Ok(Default::default())
        })?;
    assert_eq!(expected, Vec::<&str>::new(), "all paths should have been seen");

    let err = from
        .changes()?
        .track_path()
        .for_each_to_obtain_tree(&to, |_change| {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "custom error"))
        })
        .unwrap_err();
    assert_eq!(
        err.to_string(),
        "The user-provided callback failed",
        "custom errors made visible and not squelched"
    );
    Ok(())
}

fn tree_named(repo: &gix::Repository, rev_spec: impl AsRef<str>) -> gix::Tree {
    repo.rev_parse_single(rev_spec.as_ref())
        .unwrap()
        .object()
        .unwrap()
        .peel_to_kind(gix::object::Kind::Tree)
        .unwrap()
        .into_tree()
}

mod track_rewrites {
    use std::convert::Infallible;

    use gix::object::tree::diff::{
        change::{DiffLineStats, Event},
        rewrites::{Copies, CopySource},
        Rewrites,
    };
    use gix_ref::bstr::BStr;

    use crate::{
        object::tree::diff::{added, deleted, modified, store, tree_named},
        util::named_repo,
    };

    #[test]
    fn renames_by_identity() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        for (commit_msg, expected, assert_msg) in [
            (
                "r1-identity",
                vec![BStr::new("a"), "dir/a-moved".into()],
                "one rename and nothing else",
            ),
            (
                "r2-ambiguous",
                vec![
                    "s1".into(),
                    "b1".into(),
                    "s2".into(),
                    "b2".into(),
                    "s3".into(),
                    "z".into(),
                ],
                "multiple possible sources decide by ordering everything lexicographically",
            ),
            (
                "r4-symlinks",
                vec!["link-1".into(), "renamed-link-1".into()],
                "symlinks are only tracked by identity",
            ),
            (
                "c4 - add identical files",
                vec![],
                "not having any renames is OK as well",
            ),
            ("tc1-identity", vec![], "copy tracking is off by default"),
        ] {
            let from = tree_named(&repo, format!("@^{{/{commit_msg}}}~1"));
            let to = tree_named(&repo, format!(":/{commit_msg}"));

            for percentage in [None, Some(0.5)] {
                let mut actual = Vec::new();
                #[cfg_attr(windows, allow(unused_variables))]
                let out = from
                    .changes()?
                    .track_path()
                    .track_rewrites(
                        Rewrites {
                            percentage,
                            ..Default::default()
                        }
                        .into(),
                    )
                    .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                        if !change.event.entry_mode().is_tree() {
                            if let Event::Rewrite {
                                source_location, copy, ..
                            } = change.event
                            {
                                actual.push(source_location.to_owned());
                                actual.push(change.location.to_owned());
                                assert!(!copy);
                            }
                        }
                        Ok(Default::default())
                    })?;
                assert_eq!(actual, expected, "{assert_msg}");
                #[cfg(not(windows))]
                assert_eq!(
                    out.rewrites.expect("present as its configured").num_similarity_checks,
                    0,
                    "even though fuzzy checks are enabled, we don't end up using them"
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rename_by_similarity() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/r3-simple}~1");
        let to = tree_named(&repo, ":/r3-simple");

        for percentage in [
            None,
            Some(0.76), /*cutoff point where git stops seeing it as equal */
        ] {
            let mut actual = Vec::new();
            let mut rewrite_count = 0;
            let out = from
                .changes()?
                .track_path()
                .track_rewrites(
                    Rewrites {
                        percentage,
                        ..Default::default()
                    }
                    .into(),
                )
                .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                    if !change.event.entry_mode().is_tree() {
                        if let Event::Rewrite { .. } = change.event {
                            rewrite_count += 0;
                        } else {
                            actual.push(change.location.to_owned());
                        }
                    }
                    Ok(Default::default())
                })?;
            assert_eq!(
                actual,
                vec![BStr::new("b"), "dir/c".into(), "dir/c-moved".into()],
                "these items include no rewrite as the cut-off is chosen accordingly"
            );
            if percentage.is_some() {
                assert_eq!(
                    out.rewrites
                        .expect("always set as rewrite tracking is configured")
                        .num_similarity_checks,
                    1
                );
            }
        }

        let mut actual = Vec::new();
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    percentage: Some(0.75),
                    limit: 1, // has no effect as it's just one item here.
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        source_location, copy, ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        assert!(!copy);
                    }
                }
                Ok(Default::default())
            })?;
        assert_eq!(
            actual,
            vec![BStr::new("dir/c"), "dir/c-moved".into()],
            "it found all items at the cut-off point, similar to git"
        );
        let out = out.rewrites.expect("tracking enabled");
        assert_eq!(out.num_similarity_checks, 1);
        assert_eq!(out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 0);
        assert_eq!(out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 0);

        Ok(())
    }

    #[test]
    fn renames_by_similarity_with_limit() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/r5}~1");
        let to = tree_named(&repo, ":/r5");

        let mut actual = Vec::new();
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    limit: 1, // prevent fuzzy tracking from happening
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite { .. } = change.event {
                        unreachable!("fuzzy tracking is effecitively disabled due to limit");
                    }
                    actual.push(change.location.to_owned());
                }
                Ok(Default::default())
            })?;
        assert_eq!(
            actual,
            vec![BStr::new("f1"), "f1-renamed".into(), "f2".into(), "f2-renamed".into()],
        );
        let out = out.rewrites.expect("tracking enabled");
        assert_eq!(out.num_similarity_checks, 0);
        assert_eq!(out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 4);
        assert_eq!(out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 0);

        Ok(())
    }

    #[test]
    fn copies_by_identity() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/tc1-identity}~1");
        let to = tree_named(&repo, ":/tc1-identity");

        let mut actual = Vec::new();
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies {
                        source: CopySource::FromSetOfModifiedFiles,
                        percentage: None,
                    }),
                    limit: 1, // the limit isn't actually used for identity based checks
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        source_location, copy, ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        assert!(copy);
                    }
                }
                Ok(Default::default())
            })?;
        assert_eq!(
            actual,
            vec![
                BStr::new("base"),
                "c1".into(),
                "base".into(),
                "c2".into(),
                "base".into(),
                "dir/c3".into()
            ],
        );
        let out = out.rewrites.expect("tracking enabled");
        assert_eq!(out.num_similarity_checks, 0);
        assert_eq!(out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 0);
        assert_eq!(out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 0);

        Ok(())
    }

    #[test]
    fn copies_by_similarity() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/tc2-similarity}~1");
        let to = tree_named(&repo, ":/tc2-similarity");

        let mut actual = Vec::new();
        let mut stat = None;
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies::default()),
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        source_location,
                        copy,
                        diff,
                        ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        stat = diff;
                        assert!(copy);
                    }
                }
                Ok(Default::default())
            })?;
        assert_eq!(
            actual,
            vec![
                BStr::new("base"),
                "c4".into(),
                "base".into(),
                "c5".into(),
                "base".into(),
                "dir/c6".into()
            ],
        );
        assert_eq!(
            stat,
            Some(DiffLineStats {
                removals: 0,
                insertions: 1,
                before: 11,
                after: 12,
            }),
            "by similarity there is a diff"
        );

        let out = out.rewrites.expect("tracking enabled");
        assert_eq!(
            out.num_similarity_checks, 2,
            "two are similar, the other one is identical"
        );
        assert_eq!(out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 0);
        assert_eq!(out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 0);

        Ok(())
    }

    #[test]
    fn copies_in_entire_tree_by_similarity() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/tc3-find-harder}~1");
        let to = tree_named(&repo, ":/tc3-find-harder");

        let mut actual = Vec::new();
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies::default()),
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite { .. } = change.event {
                        unreachable!("needs --find-copies-harder to detect them here")
                    }
                    actual.push(change.location.to_owned());
                }
                Ok(Default::default())
            })?;
        assert_eq!(
            actual,
            vec![BStr::new("b"), "c6".into(), "c7".into(), "newly-added".into(),],
        );

        let out = out.rewrites.expect("tracking enabled");
        assert_eq!(
            out.num_similarity_checks, 3,
            "it does have some candidates, probably for rename tracking"
        );
        assert_eq!(
            out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 0,
            "no limit configured"
        );
        assert_eq!(out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 0);

        let mut actual = Vec::new();
        let mut stat = None;
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies {
                        source: CopySource::FromSetOfModifiedFilesAndSourceTree,
                        ..Default::default()
                    }),
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        copy,
                        diff,
                        source_location,
                        ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        stat = diff;
                        assert!(copy);
                    }
                }
                Ok(Default::default())
            })?;
        assert_eq!(
            actual,
            vec![
                BStr::new("base"),
                "c6".into(),
                "dir/c6".into(),
                "c7".into(),
                "c5".into(),
                "newly-added".into()
            ]
        );

        let out = out.rewrites.expect("tracking enabled");
        assert_eq!(
            stat,
            Some(DiffLineStats {
                removals: 0,
                insertions: 3,
                before: 12,
                after: 15,
            }),
            "by similarity there is a diff"
        );
        assert_eq!(out.num_similarity_checks, 4);
        assert_eq!(
            out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 0,
            "no limit configured"
        );
        assert_eq!(out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 0);

        Ok(())
    }

    #[test]
    fn copies_in_entire_tree_by_similarity_with_limit() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/tc3-find-harder}~1");
        let to = tree_named(&repo, ":/tc3-find-harder");

        let mut actual = Vec::new();
        let mut stat = None;
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies {
                        source: CopySource::FromSetOfModifiedFilesAndSourceTree,
                        ..Default::default()
                    }),
                    limit: 2, // similarity checks can't be made that way
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        copy,
                        diff,
                        source_location,
                        ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        stat = diff;
                        assert!(copy);
                    }
                }
                Ok(Default::default())
            })?;
        assert_eq!(
            actual,
            vec![BStr::new("base"), "c6".into(), "dir/c6".into(), "c7".into(),],
            "identification by identity, which is fast due to binary search"
        );

        let out = out.rewrites.expect("tracking enabled");
        assert_eq!(stat, None, "similarity can't run");
        assert_eq!(out.num_similarity_checks, 3);
        assert_eq!(
            out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 0,
            "no limit configured"
        );
        assert_eq!(out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 57);

        Ok(())
    }

    #[test]
    fn copies_by_similarity_with_limit() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/tc2-similarity}~1");
        let to = tree_named(&repo, ":/tc2-similarity");

        let mut actual = Vec::new();
        let mut stat = None;
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies::default()),
                    limit: 1,
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        source_location,
                        copy,
                        diff,
                        ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        stat = diff;
                        assert!(copy);
                    }
                }
                Ok(Default::default())
            })?;
        assert_eq!(
            actual,
            vec![BStr::new("base"), "c4".into()],
            "the limit prevents any similarity check from being performed, and identity fails everywhere"
        );
        assert_eq!(stat, None, "by identity there is no diff");

        let out = out.rewrites.expect("tracking enabled");
        assert_eq!(out.num_similarity_checks, 0);
        assert_eq!(out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 0);
        assert_eq!(out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 3);

        Ok(())
    }

    #[test]
    fn realistic_renames() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/r1-change}~1");
        let to = tree_named(&repo, ":/r1-change");

        let mut actual = Vec::new();
        let mut other = Vec::new();
        from.changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies::default()),
                    limit: 1,
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        source_location, copy, ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        assert!(!copy);
                    } else {
                        other.push(store(&change));
                    }
                }
                Ok(Default::default())
            })?;

        assert_eq!(actual, vec!["git-index/src/file.rs", "git-index/src/file/mod.rs"]);
        assert_eq!(
            other,
            vec![
                added("git-index/tests/index/file/access.rs"),
                modified("git-index/tests/index/file/mod.rs")
            ]
        );

        #[cfg(not(windows))]
        {
            let actual = std::fs::read_to_string(repo.work_dir().expect("non-bare").join("baseline.with-renames"))?;
            let expected = r#"commit 6974f2b5181772977a9d7d34a566414508552650
Author: author <author@example.com>
Date:   Sat Jan 1 00:00:00 2000 +0000

    r1-change

diff --git a/git-index/src/file.rs b/git-index/src/file/mod.rs
similarity index 100%
rename from git-index/src/file.rs
rename to git-index/src/file/mod.rs
diff --git a/git-index/tests/index/file/access.rs b/git-index/tests/index/file/access.rs
new file mode 100644
index 0000000..e69de29
diff --git a/git-index/tests/index/file/mod.rs b/git-index/tests/index/file/mod.rs
index e69de29..8ba3a16 100644
--- a/git-index/tests/index/file/mod.rs
+++ b/git-index/tests/index/file/mod.rs
@@ -0,0 +1 @@
+n
"#;
            assert_eq!(actual, expected);
        }

        Ok(())
    }

    #[test]
    fn realistic_renames_disabled() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/r1-change}~1");
        let to = tree_named(&repo, ":/r1-change");

        let mut actual = Vec::new();
        from.changes()?
            .track_path()
            .track_rewrites(None)
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    actual.push(store(&change));
                    if let Event::Rewrite { .. } = change.event {
                        unreachable!("it's disabled, so cannot happen")
                    }
                }
                Ok(Default::default())
            })?;

        assert_eq!(
            actual,
            vec![
                deleted("git-index/src/file.rs"),
                added("git-index/src/file/mod.rs"),
                added("git-index/tests/index/file/access.rs"),
                modified("git-index/tests/index/file/mod.rs")
            ]
        );

        #[cfg(not(windows))]
        {
            let actual = std::fs::read_to_string(repo.work_dir().expect("non-bare").join("baseline.no-renames"))?;
            let expected = r#"commit 6974f2b5181772977a9d7d34a566414508552650
Author: author <author@example.com>
Date:   Sat Jan 1 00:00:00 2000 +0000

    r1-change

diff --git a/git-index/src/file.rs b/git-index/src/file.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/git-index/src/file/mod.rs b/git-index/src/file/mod.rs
new file mode 100644
index 0000000..e69de29
diff --git a/git-index/tests/index/file/access.rs b/git-index/tests/index/file/access.rs
new file mode 100644
index 0000000..e69de29
diff --git a/git-index/tests/index/file/mod.rs b/git-index/tests/index/file/mod.rs
index e69de29..8ba3a16 100644
--- a/git-index/tests/index/file/mod.rs
+++ b/git-index/tests/index/file/mod.rs
@@ -0,0 +1 @@
+n
"#;
            assert_eq!(actual, expected);
        }

        Ok(())
    }

    #[test]
    fn realistic_renames_disabled_2() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/r2-change}~1");
        let to = tree_named(&repo, ":/r2-change");

        let mut actual = Vec::new();
        from.changes()?
            .track_path()
            .track_rewrites(None)
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    actual.push(store(&change));
                    if let Event::Rewrite { .. } = change.event {
                        unreachable!("it's disabled, so cannot happen")
                    }
                }
                Ok(Default::default())
            })?;

        #[cfg(not(windows))]
        {
            let expected = r#"commit 72de3500e1bff816e56432bee8de02946d3e784b
Author: author <author@example.com>
Date:   Sat Jan 1 00:00:00 2000 +0000

    r2-change

diff --git a/git-sec/CHANGELOG.md b/git-sec/CHANGELOG.md
deleted file mode 100644
index e69de29..0000000
diff --git a/git-sec/Cargo.toml b/git-sec/Cargo.toml
deleted file mode 100644
index e69de29..0000000
diff --git a/git-sec/src/identity.rs b/git-sec/src/identity.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/git-sec/src/lib.rs b/git-sec/src/lib.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/git-sec/src/permission.rs b/git-sec/src/permission.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/git-sec/src/trust.rs b/git-sec/src/trust.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/git-sec/tests/identity/mod.rs b/git-sec/tests/identity/mod.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/git-sec/tests/sec.rs b/git-sec/tests/sec.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/gix-sec/CHANGELOG.md b/gix-sec/CHANGELOG.md
new file mode 100644
index 0000000..e69de29
diff --git a/gix-sec/Cargo.toml b/gix-sec/Cargo.toml
new file mode 100644
index 0000000..e69de29
diff --git a/gix-sec/src/identity.rs b/gix-sec/src/identity.rs
new file mode 100644
index 0000000..e69de29
diff --git a/gix-sec/src/lib.rs b/gix-sec/src/lib.rs
new file mode 100644
index 0000000..e69de29
diff --git a/gix-sec/src/permission.rs b/gix-sec/src/permission.rs
new file mode 100644
index 0000000..e69de29
diff --git a/gix-sec/src/trust.rs b/gix-sec/src/trust.rs
new file mode 100644
index 0000000..e69de29
diff --git a/gix-sec/tests/identity/mod.rs b/gix-sec/tests/identity/mod.rs
new file mode 100644
index 0000000..e69de29
diff --git a/gix-sec/tests/sec.rs b/gix-sec/tests/sec.rs
new file mode 100644
index 0000000..e69de29
"#;
            assert_eq!(
                std::fs::read_to_string(repo.work_dir().expect("non-bare").join("baseline-2.no-renames"))?,
                expected
            );
        }

        assert_eq!(
            actual,
            vec![
                deleted("git-sec/CHANGELOG.md"),
                deleted("git-sec/Cargo.toml"),
                added("gix-sec/CHANGELOG.md"),
                added("gix-sec/Cargo.toml"),
                deleted("git-sec/src/identity.rs"),
                deleted("git-sec/src/lib.rs"),
                deleted("git-sec/src/permission.rs"),
                deleted("git-sec/src/trust.rs"),
                deleted("git-sec/tests/sec.rs"),
                added("gix-sec/src/identity.rs"),
                added("gix-sec/src/lib.rs"),
                added("gix-sec/src/permission.rs"),
                added("gix-sec/src/trust.rs"),
                added("gix-sec/tests/sec.rs"),
                deleted("git-sec/tests/identity/mod.rs"),
                added("gix-sec/tests/identity/mod.rs"),
            ]
        );

        Ok(())
    }

    #[test]
    fn realistic_renames_disabled_3() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/r3-change}~1");
        let to = tree_named(&repo, ":/r3-change");

        let mut actual = Vec::new();
        from.changes()?
            .track_path()
            .track_rewrites(None)
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    actual.push(store(&change));
                    if let Event::Rewrite { .. } = change.event {
                        unreachable!("it's disabled, so cannot happen")
                    }
                }
                Ok(Default::default())
            })?;

        #[cfg(not(windows))]
        {
            let expected = r#"commit dee00f5a20957db20d8d2e0050210716d6b44879
Author: author <author@example.com>
Date:   Sat Jan 1 00:00:00 2000 +0000

    r3-change

diff --git a/src/ein.rs b/src/ein.rs
new file mode 100644
index 0000000..e69de29
diff --git a/src/gix.rs b/src/gix.rs
new file mode 100644
index 0000000..e69de29
diff --git a/src/plumbing-cli.rs b/src/plumbing-cli.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/src/porcelain-cli.rs b/src/porcelain-cli.rs
deleted file mode 100644
index e69de29..0000000
"#;

            assert_eq!(
                std::fs::read_to_string(repo.work_dir().expect("non-bare").join("baseline-3.no-renames"))?,
                expected
            );
        }
        assert_eq!(
            actual,
            vec![
                added("src/ein.rs"),
                added("src/gix.rs"),
                deleted("src/plumbing-cli.rs"),
                deleted("src/porcelain-cli.rs"),
            ]
        );

        Ok(())
    }

    #[test]
    fn realistic_renames_3() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/r3-change}~1");
        let to = tree_named(&repo, ":/r3-change");

        let mut actual = Vec::new();
        let mut other = Vec::new();
        from.changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies::default()),
                    limit: 1,
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        source_location, copy, ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        assert!(!copy);
                    } else {
                        other.push(store(&change));
                    }
                }
                Ok(Default::default())
            })?;

        #[cfg(not(windows))]
        {
            let expected = r#"commit dee00f5a20957db20d8d2e0050210716d6b44879
Author: author <author@example.com>
Date:   Sat Jan 1 00:00:00 2000 +0000

    r3-change

diff --git a/src/plumbing-cli.rs b/src/ein.rs
similarity index 100%
rename from src/plumbing-cli.rs
rename to src/ein.rs
diff --git a/src/porcelain-cli.rs b/src/gix.rs
similarity index 100%
rename from src/porcelain-cli.rs
rename to src/gix.rs
"#;
            assert_eq!(
                std::fs::read_to_string(repo.work_dir().expect("non-bare").join("baseline-3.with-renames"))?,
                expected
            );
        }
        assert_eq!(
            actual,
            vec![
                "src/plumbing-cli.rs",
                "src/ein.rs",
                "src/porcelain-cli.rs",
                "src/gix.rs"
            ]
        );
        assert!(other.is_empty());

        Ok(())
    }

    #[test]
    fn realistic_renames_2() -> crate::Result {
        let repo = named_repo("make_diff_repo.sh")?;
        let from = tree_named(&repo, "@^{/r2-change}~1");
        let to = tree_named(&repo, ":/r2-change");

        let mut actual = Vec::new();
        from.changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies::default()),
                    limit: 1,
                    ..Default::default()
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if !change.event.entry_mode().is_tree() {
                    if let Event::Rewrite {
                        source_location, copy, ..
                    } = change.event
                    {
                        actual.push(source_location.to_owned());
                        actual.push(change.location.to_owned());
                        assert!(!copy);
                    } else {
                        unreachable!("everything is a rewrite");
                    }
                }
                Ok(Default::default())
            })?;

        #[cfg(not(windows))]
        {
            let expected = r#"commit 72de3500e1bff816e56432bee8de02946d3e784b
Author: author <author@example.com>
Date:   Sat Jan 1 00:00:00 2000 +0000

    r2-change

diff --git a/git-sec/CHANGELOG.md b/gix-sec/CHANGELOG.md
similarity index 100%
rename from git-sec/CHANGELOG.md
rename to gix-sec/CHANGELOG.md
diff --git a/git-sec/Cargo.toml b/gix-sec/Cargo.toml
similarity index 100%
rename from git-sec/Cargo.toml
rename to gix-sec/Cargo.toml
diff --git a/git-sec/src/identity.rs b/gix-sec/src/identity.rs
similarity index 100%
rename from git-sec/src/identity.rs
rename to gix-sec/src/identity.rs
diff --git a/git-sec/src/lib.rs b/gix-sec/src/lib.rs
similarity index 100%
rename from git-sec/src/lib.rs
rename to gix-sec/src/lib.rs
diff --git a/git-sec/src/permission.rs b/gix-sec/src/permission.rs
similarity index 100%
rename from git-sec/src/permission.rs
rename to gix-sec/src/permission.rs
diff --git a/git-sec/src/trust.rs b/gix-sec/src/trust.rs
similarity index 100%
rename from git-sec/src/trust.rs
rename to gix-sec/src/trust.rs
diff --git a/git-sec/tests/identity/mod.rs b/gix-sec/tests/identity/mod.rs
similarity index 100%
rename from git-sec/tests/identity/mod.rs
rename to gix-sec/tests/identity/mod.rs
diff --git a/git-sec/tests/sec.rs b/gix-sec/tests/sec.rs
similarity index 100%
rename from git-sec/tests/sec.rs
rename to gix-sec/tests/sec.rs
"#;
            assert_eq!(
                std::fs::read_to_string(repo.work_dir().expect("non-bare").join("baseline-2.with-renames"))?,
                expected
            );
        }

        assert_eq!(
            actual,
            vec![
                "git-sec/CHANGELOG.md",
                "gix-sec/CHANGELOG.md",
                "git-sec/Cargo.toml",
                "gix-sec/Cargo.toml",
                "git-sec/src/identity.rs",
                "gix-sec/src/identity.rs",
                "git-sec/src/lib.rs",
                "gix-sec/src/lib.rs",
                "git-sec/src/permission.rs",
                "gix-sec/src/permission.rs",
                "git-sec/src/trust.rs",
                "gix-sec/src/trust.rs",
                "git-sec/tests/sec.rs",
                "gix-sec/tests/sec.rs",
                "git-sec/tests/identity/mod.rs",
                "gix-sec/tests/identity/mod.rs"
            ]
        );

        Ok(())
    }
}
fn store(change: &gix::object::tree::diff::Change<'_, '_, '_>) -> (char, BString) {
    (shorthand(&change.event), change.location.to_owned())
}

fn added(path: &str) -> (char, BString) {
    ('A', path.into())
}

fn deleted(path: &str) -> (char, BString) {
    ('D', path.into())
}

fn modified(path: &str) -> (char, BString) {
    ('M', path.into())
}

fn shorthand(change: &Event) -> char {
    match change {
        Event::Addition { .. } => 'A',
        Event::Deletion { .. } => 'D',
        Event::Modification { .. } => 'M',
        Event::Rewrite { .. } => 'R',
    }
}
