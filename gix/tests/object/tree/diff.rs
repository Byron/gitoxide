use std::convert::Infallible;

use gix::object::{blob::diff::lines, tree::diff::Change};
use gix_object::{bstr::ByteSlice, tree::EntryKind};

use crate::named_repo;

#[test]
fn changes_against_tree_modified() -> crate::Result {
    let repo = named_repo("make_diff_repo.sh")?;
    let from = tree_named(&repo, "@^{/c3-modification}~1");
    let to = tree_named(&repo, ":/c3-modification");
    let mut cache = repo.diff_resource_cache(gix_diff::blob::pipeline::Mode::ToGit, Default::default())?;

    let expected_modifications = [
        (EntryKind::Blob, "a\n", EntryKind::Blob, "a\na1\n"),
        (EntryKind::Tree, "", EntryKind::Tree, ""),
        (EntryKind::Blob, "dir/c\n", EntryKind::Blob, "dir/c\ndir/c1\n"),
    ];
    let mut i = 0;

    from.changes()?
        .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
            let (expected_previous_entry_mode, expected_previous_data, expected_entry_mode, expected_data) =
                expected_modifications[i];

            assert!(
                !change.location().is_empty(),
                "without configuration the location field is set"
            );
            match change {
                Change::Modification {
                    previous_entry_mode,
                    previous_id,
                    entry_mode,
                    id,
                    ..
                } => {
                    assert_eq!(previous_entry_mode.kind(), expected_previous_entry_mode);
                    assert_eq!(entry_mode.kind(), expected_entry_mode);

                    if matches!(entry_mode.kind(), EntryKind::Tree) {
                        i += 1;
                        return Ok(Default::default());
                    }

                    assert_eq!(previous_id.object().unwrap().data.as_bstr(), expected_previous_data);
                    assert_eq!(id.object().unwrap().data.as_bstr(), expected_data);
                }
                Change::Rewrite { .. } | Change::Deletion { .. } | Change::Addition { .. } => {
                    unreachable!("only modification is expected")
                }
            };

            let mut diff = change.diff(&mut cache).expect("objects available");
            let count = diff.line_counts().expect("no diff error").expect("no binary blobs");
            assert_eq!(count.insertions, 1);
            assert_eq!(count.removals, 0);
            diff.lines(|hunk| {
                match hunk {
                    lines::Change::Deletion { .. } => unreachable!("there was no deletion"),
                    lines::Change::Addition { lines } => assert_eq!(
                        lines,
                        vec![expected_data[expected_previous_data.len()..].as_bytes().as_bstr()]
                    ),
                    lines::Change::Modification { .. } => unreachable!("there was no modification"),
                };
                Ok::<_, Infallible>(())
            })
            .expect("infallible");

            i += 1;
            Ok(Default::default())
        })?;
    assert_eq!(i, 3);

    assert_eq!(
        from.changes()?.stats(&to)?,
        gix::object::tree::diff::Stats {
            lines_added: 2,
            lines_removed: 0,
            files_changed: 2,
        },
        "two files with one added line each"
    );

    Ok(())
}

mod track_rewrites {
    use std::collections::HashMap;
    use std::convert::Infallible;

    use gix::{
        diff::{
            rewrites::{Copies, CopySource},
            Rewrites,
        },
        object::tree::diff::Change,
    };
    use gix_ref::bstr::BStr;

    use crate::object::tree::diff::tree_named;
    use crate::util::named_subrepo_opts;

    #[test]
    #[cfg_attr(
        windows,
        ignore = "Fails on some Window systems, like the fixture doesn't get set up correctly."
    )]
    fn jj_realistic_needs_to_be_more_clever() -> crate::Result {
        let repo = named_subrepo_opts("make_diff_repos.sh", "jj-trackcopy-1", gix::open::Options::isolated())?;

        let mut expected = HashMap::<&BStr, (&BStr, u32)>::new();
        expected.insert(
            "cli/src/commands/file/chmod.rs".into(),
            ("cli/src/commands/chmod.rs".into(), 90),
        );
        expected.insert(
            "cli/src/commands/file/print.rs".into(),
            ("cli/src/commands/cat.rs".into(), 90),
        );
        expected.insert(
            "cli/tests/test_file_chmod_command.rs".into(),
            ("cli/tests/test_chmod_command.rs".into(), 88),
        );
        expected.insert(
            "cli/tests/test_file_print_command.rs".into(),
            ("cli/tests/test_cat_command.rs".into(), 77),
        );

        let from = tree_named(&repo, "@~1");
        let to = tree_named(&repo, "@");
        let out = from
            .changes()?
            .track_path()
            .track_rewrites(
                Rewrites {
                    copies: Some(Copies {
                        source: CopySource::FromSetOfModifiedFiles,
                        percentage: Some(0.5),
                    }),
                    limit: 1000,
                    percentage: Some(0.5),
                }
                .into(),
            )
            .for_each_to_obtain_tree(&to, |change| -> Result<_, Infallible> {
                if let Change::Rewrite {
                    source_location,
                    diff: Some(diff),
                    location,
                    ..
                } = change
                {
                    // Round to percentage points to avoid floating point error.
                    let similarity = (diff.similarity * 100.0) as u32;
                    let v = expected.remove(location);
                    assert_eq!(v, Some((source_location, similarity)));
                }
                Ok(Default::default())
            })?;

        assert_eq!(expected, HashMap::new());
        let out = out.expect("tracking enabled");
        assert_eq!(
            out.num_similarity_checks, 21,
            "this probably increases once the algorithm improves"
        );
        assert_eq!(
            out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit, 0,
            "limit disabled"
        );
        assert_eq!(
            out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit, 0,
            "limit disabled"
        );

        Ok(())
    }
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
