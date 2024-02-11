use gix_dir::walk;
use pretty_assertions::assert_eq;

use crate::walk_utils::{
    collect, collect_filtered, entry, entry_dirstat, entry_nokind, entry_nomatch, entryps, entryps_dirstat, fixture,
    fixture_in, options, options_emit_all, try_collect, try_collect_filtered_opts, EntryExt, Options,
};
use gix_dir::entry::Kind::*;
use gix_dir::entry::PathspecMatch::*;
use gix_dir::entry::Status::*;
use gix_dir::walk::EmissionMode::*;
use gix_dir::walk::ForDeletionMode;
use gix_ignore::Kind::*;

#[test]
#[cfg_attr(windows, ignore = "symlinks the way they are organized don't yet work on windows")]
fn root_may_not_lead_through_symlinks() -> crate::Result {
    for (name, intermediate, expected) in [
        ("immediate-breakout-symlink", "", 0),
        ("breakout-symlink", "hide", 1),
        ("breakout-symlink", "hide/../hide", 1),
    ] {
        let root = fixture_in("many-symlinks", name);
        let err = try_collect(&root, |keep, ctx| {
            walk(&root.join(intermediate).join("breakout"), &root, ctx, options(), keep)
        })
        .unwrap_err();
        assert!(
            matches!(err, walk::Error::SymlinkInRoot { component_index, .. } if component_index == expected),
            "{name} should have component {expected}"
        );
    }
    Ok(())
}

#[test]
fn empty_root() -> crate::Result {
    let root = fixture("empty");
    let (out, entries) = collect(&root, |keep, ctx| walk(&root, &root, ctx, options(), keep));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 1,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(
        entries.len(),
        0,
        "by default, nothing is shown as the directory is empty"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_empty_directories: true,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 1,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry("", Untracked, EmptyDirectory),
        "this is how we can indicate the worktree is entirely untracked"
    );
    Ok(())
}

#[test]
fn complex_empty() -> crate::Result {
    let root = fixture("complex-empty");
    let (out, entries) = collect(&root, |keep, ctx| walk(&root, &root, ctx, options_emit_all(), keep));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 9,
            returned_entries: entries.len(),
            seen_entries: 5,
        }
    );
    assert_eq!(
        entries,
        &[
            entry("dirs-and-files/dir/file", Untracked, File),
            entry("dirs-and-files/sub", Untracked, EmptyDirectory),
            entry("empty-toplevel", Untracked, EmptyDirectory),
            entry("only-dirs/other", Untracked, EmptyDirectory),
            entry("only-dirs/sub/subsub", Untracked, EmptyDirectory),
        ],
        "we see each and every directory, and get it classified as empty as it's set to be emitted"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_empty_directories: false,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 9,
            returned_entries: entries.len(),
            seen_entries: 5,
        }
    );
    assert_eq!(
        entries,
        &[entry("dirs-and-files/dir/file", Untracked, File),],
        "by default, no empty directory shows up"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_empty_directories: true,
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 9,
            returned_entries: entries.len(),
            seen_entries: 9,
        }
    );
    assert_eq!(
        entries,
        &[
            entry("dirs-and-files", Untracked, Directory),
            entry("empty-toplevel", Untracked, EmptyDirectory),
            entry("only-dirs", Untracked, Directory),
        ],
        "empty directories collapse just fine"
    );
    Ok(())
}

#[test]
fn only_untracked() -> crate::Result {
    let root = fixture("only-untracked");
    let (out, entries) = collect(&root, |keep, ctx| walk(&root, &root, ctx, options(), keep));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7,
        }
    );
    assert_eq!(
        &entries,
        &[
            entry("a", Untracked, File),
            entry("b", Untracked, File),
            entry("c", Untracked, File),
            entry("d/a", Untracked, File),
            entry("d/b", Untracked, File),
            entry("d/d/a", Untracked, File),
        ]
    );

    let (out, entries) = collect_filtered(&root, |keep, ctx| walk(&root, &root, ctx, options(), keep), Some("d/*"));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7,
        }
    );
    assert_eq!(
        &entries,
        &[
            entryps("d/a", Untracked, File, WildcardMatch),
            entryps("d/b", Untracked, File, WildcardMatch),
            entryps("d/d/a", Untracked, File, WildcardMatch),
        ]
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7 + 2,
        },
        "There are 2 extra directories that we fold into, but ultimately discard"
    );
    assert_eq!(
        &entries,
        &[
            entry("a", Untracked, File),
            entry("b", Untracked, File),
            entry("c", Untracked, File),
            entry("d", Untracked, Directory),
        ]
    );
    Ok(())
}

#[test]
fn only_untracked_explicit_pathspec_selection() -> crate::Result {
    let root = fixture("only-untracked");
    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_untracked: Matching,
                    ..options()
                },
                keep,
            )
        },
        ["d/a", "d/d/a"],
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7,
        },
    );
    assert_eq!(
        &entries,
        &[
            entryps("d/a", Untracked, File, Verbatim),
            entryps("d/d/a", Untracked, File, Verbatim)
        ],
        "this works just like expected, as nothing is collapsed anyway"
    );

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_untracked: CollapseDirectory,
                    emit_pruned: true,
                    ..options()
                },
                keep,
            )
        },
        ["d/a", "d/d/a"],
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7,
        },
        "no collapsing happens"
    );
    assert_eq!(
        &entries,
        &[
            entry_nokind(".git", DotGit), 
            entry_nokind("a", Pruned), 
            entry_nokind("b", Pruned), 
            entry_nokind("c", Pruned), 
            entryps("d/a", Untracked, File, Verbatim),
            entry_nokind("d/b", Pruned),
            entryps("d/d/a", Untracked, File, Verbatim)],
        "we actually want to mention the entries that matched the pathspec precisely, so two of them would be needed here \
        while preventing the directory collapse from happening"
    );

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_untracked: CollapseDirectory,
                    ..options()
                },
                keep,
            )
        },
        Some("d/*"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7 + 2,
        },
        "collapsing happens just like Git"
    );
    assert_eq!(
        &entries,
        &[entryps("d", Untracked, Directory, WildcardMatch)],
        "wildcard matches allow collapsing directories because Git does"
    );
    Ok(())
}

#[test]
fn expendable_and_precious() {
    let root = fixture("expendable-and-precious");
    let (out, entries) = collect(&root, |keep, ctx| walk(&root, &root, ctx, options_emit_all(), keep));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 6,
            returned_entries: entries.len(),
            seen_entries: 18,
        }
    );
    assert_eq!(
        &entries,
        &[
            entry_nokind(".git", DotGit),
            entry(".gitignore", Tracked, File),
            entry("a.o", Ignored(Expendable), File),
            entry("all-expendable", Ignored(Expendable), Directory),
            entry("all-expendable-by-filematch/e.o", Ignored(Expendable), File),
            entry("all-expendable-by-filematch/f.o", Ignored(Expendable), File),
            entry("all-precious", Ignored(Precious), Directory),
            entry("all-precious-by-filematch/a.precious", Ignored(Precious), File),
            entry("all-precious-by-filematch/b.precious", Ignored(Precious), File),
            entry("mixed/b.o", Ignored(Expendable), File),
            entry("mixed/precious", Ignored(Precious), File),
            entry("precious", Ignored(Precious), File),
            entry("some-expendable/file", Tracked, File),
            entry("some-expendable/file.o", Ignored(Expendable), File),
            entry("some-expendable/new", Untracked, File),
            entry("some-precious/file", Tracked, File),
            entry("some-precious/file.precious", Ignored(Precious), File),
            entry("some-precious/new", Untracked, File),
        ],
        "listing everything is a 'matching' preset, which is among the most efficient."
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                emit_tracked: true,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 6,
            returned_entries: entries.len(),
            seen_entries: 18 + 2,
        }
    );

    assert_eq!(
        &entries,
        &[
            entry(".gitignore", Tracked, File),
            entry("a.o", Ignored(Expendable), File),
            entry("all-expendable", Ignored(Expendable), Directory),
            entry("all-expendable-by-filematch", Ignored(Expendable), Directory),
            entry("all-precious", Ignored(Precious), Directory),
            entry("all-precious-by-filematch", Ignored(Precious), Directory),
            entry("mixed/b.o", Ignored(Expendable), File),
            entry("mixed/precious", Ignored(Precious), File),
            entry("precious", Ignored(Precious), File),
            entry("some-expendable/file", Tracked, File),
            entry("some-expendable/file.o", Ignored(Expendable), File),
            entry("some-expendable/new", Untracked, File),
            entry("some-precious/file", Tracked, File),
            entry("some-precious/file.precious", Ignored(Precious), File),
            entry("some-precious/new", Untracked, File),
        ],
        "those that have tracked and ignored won't be collapsed, nor will be folders that have mixed precious and ignored files,\
        those with all files of one type will be collapsed though"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: None,
                emit_untracked: CollapseDirectory,
                emit_tracked: false,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 6,
            returned_entries: entries.len(),
            seen_entries: 16 + 2,
        }
    );

    assert_eq!(
        &entries,
        &[
            entry("some-expendable/new", Untracked, File),
            entry("some-precious/new", Untracked, File),
        ],
        "even with collapsing, once there is a tracked file in the directory, we show the untracked file directly"
    );
}

#[test]
fn subdir_untracked() -> crate::Result {
    let root = fixture("subdir-untracked");
    let (out, entries) = collect(&root, |keep, ctx| walk(&root, &root, ctx, options(), keep));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7,
        }
    );
    assert_eq!(&entries, &[entry("d/d/a", Untracked, File)]);

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| walk(&root, &root, ctx, options(), keep),
        Some("d/d/*"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7,
        },
        "pruning has no actual effect here as there is no extra directories that could be avoided"
    );
    assert_eq!(&entries, &[entryps("d/d/a", Untracked, File, WildcardMatch)]);

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 7 + 1,
        },
        "there is a folded directory we added"
    );
    assert_eq!(&entries, &[entry("d/d", Untracked, Directory)]);
    Ok(())
}

#[test]
fn only_untracked_from_subdir() -> crate::Result {
    let root = fixture("only-untracked");
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(&root.join("d").join("d"), &root, ctx, options(), keep)
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 1,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(
        &entries,
        &[entry("d/d/a", Untracked, File)],
        "even from subdirs, paths are worktree relative"
    );
    Ok(())
}

#[test]
fn untracked_and_ignored_pathspec_guidance() -> crate::Result {
    for for_deletion in [None, Some(Default::default())] {
        let root = fixture("subdir-untracked-and-ignored");
        let (out, entries) = collect_filtered(
            &root,
            |keep, ctx| {
                walk(
                    &root,
                    &root,
                    ctx,
                    walk::Options {
                        emit_ignored: Some(CollapseDirectory),
                        for_deletion,
                        ..options()
                    },
                    keep,
                )
            },
            Some("d/d/generated/b"),
        );
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 4,
                returned_entries: entries.len(),
                seen_entries: 19,
            },
        );
        assert_eq!(
            &entries,
            &[entryps("d/d/generated/b", Ignored(Expendable), File, Verbatim)],
            "pathspecs allow reaching into otherwise ignored directories, ignoring the flag to collapse"
        );
    }
    Ok(())
}

#[test]
fn untracked_and_ignored_for_deletion_negative_spec() -> crate::Result {
    let root = fixture("subdir-untracked-and-ignored");
    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(CollapseDirectory),
                    emit_untracked: CollapseDirectory,
                    emit_pruned: true,
                    for_deletion: Some(Default::default()),
                    ..options()
                },
                keep,
            )
        },
        Some(":!*generated*"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 5,
            returned_entries: entries.len(),
            seen_entries: 23,
        },
    );
    assert_eq!(
        &entries,
        &[
            entry_nokind(".git", DotGit),
            entry(".gitignore", Untracked, File),
            entry("a.o", Ignored(Expendable), File),
            entry("b.o", Ignored(Expendable), File),
            entry("c.o", Ignored(Expendable), File),
            entry("d/a.o", Ignored(Expendable), File),
            entry("d/b.o", Ignored(Expendable), File),
            entry("d/d/a", Untracked, File),
            entry("d/d/a.o", Ignored(Expendable), File),
            entry("d/d/b.o", Ignored(Expendable), File),
            entryps("d/d/generated", Ignored(Expendable), Directory, Excluded),
            entryps("d/generated", Ignored(Expendable), Directory, Excluded),
            entryps("generated", Ignored(Expendable), Directory, Excluded),
            entry("objs", Ignored(Expendable), Directory),
        ],
        "'generated' folders are excluded, and collapsing is done where possible. \
         Note that Git wants to incorrectly delete `d/d` as it doesn't see the excluded \
         ignored file inside, which would incorrectly delete something the users didn't want deleted."
    );
    Ok(())
}

#[test]
fn untracked_and_ignored() -> crate::Result {
    let root = fixture("subdir-untracked-and-ignored");
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(Matching),
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 5,
            returned_entries: entries.len(),
            seen_entries: 21,
        },
        "some untracked ones are hidden by default"
    );
    assert_eq!(
        &entries,
        &[
            entry(".gitignore", Untracked, File),
            entry("a.o", Ignored(Expendable), File),
            entry("b.o", Ignored(Expendable), File),
            entry("c.o", Ignored(Expendable), File),
            entry("d/a.o", Ignored(Expendable), File),
            entry("d/b.o", Ignored(Expendable), File),
            entry("d/d/a", Untracked, File),
            entry("d/d/a.o", Ignored(Expendable), File),
            entry("d/d/b.o", Ignored(Expendable), File),
            entry("d/d/generated", Ignored(Expendable), Directory),
            entry("d/generated", Ignored(Expendable), Directory),
            entry("generated", Ignored(Expendable), Directory),
            entry("objs/a.o", Ignored(Expendable), File),
            entry("objs/b.o", Ignored(Expendable), File),
            entry("objs/sub/other.o", Ignored(Expendable), File),
        ]
    );

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_pruned: true,
                    ..options()
                },
                keep,
            )
        },
        Some("**/a*"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 5,
            returned_entries: entries.len(),
            seen_entries: 21,
        },
        "basically the same result…"
    );

    assert_eq!(
        &entries,
        &[
            entry_nokind(".git", DotGit),
            entry_nomatch(".gitignore", Pruned, File),
            entryps("d/d/a", Untracked, File, WildcardMatch),
        ],
        "…but with different classification as the ignore file is pruned so it's not untracked anymore"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: None,
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 5,
            returned_entries: entries.len(),
            seen_entries: 21 + 1,
        },
        "we still encounter the same amount of entries, and 1 folded directory"
    );
    assert_eq!(
        &entries,
        &[entry(".gitignore", Untracked, File), entry("d/d", Untracked, Directory)],
        "aggregation kicks in here"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 5,
            returned_entries: entries.len(),
            seen_entries: 21 + 2,
        },
        "some untracked ones are hidden by default, folded directories"
    );
    assert_eq!(
        &entries,
        &[
            entry(".gitignore", Untracked, File),
            entry("a.o", Ignored(Expendable), File),
            entry("b.o", Ignored(Expendable), File),
            entry("c.o", Ignored(Expendable), File),
            entry("d/a.o", Ignored(Expendable), File),
            entry("d/b.o", Ignored(Expendable), File),
            entry("d/d/a", Untracked, File),
            entry("d/d/a.o", Ignored(Expendable), File),
            entry("d/d/b.o", Ignored(Expendable), File),
            entry("d/d/generated", Ignored(Expendable), Directory),
            entry("d/generated", Ignored(Expendable), Directory),
            entry("generated", Ignored(Expendable), Directory),
            entry("objs", Ignored(Expendable), Directory),
        ],
        "objects are aggregated"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 5,
            returned_entries: entries.len(),
            seen_entries: 21 + 3,
        },
        "some untracked ones are hidden by default, and folded directories"
    );
    assert_eq!(
        &entries,
        &[
            entry(".gitignore", Untracked, File),
            entry("a.o", Ignored(Expendable), File),
            entry("b.o", Ignored(Expendable), File),
            entry("c.o", Ignored(Expendable), File),
            entry("d/a.o", Ignored(Expendable), File),
            entry("d/b.o", Ignored(Expendable), File),
            entry("d/d", Untracked, Directory),
            entryps_dirstat("d/d/a.o", Ignored(Expendable), File, Always, Untracked),
            entryps_dirstat("d/d/b.o", Ignored(Expendable), File, Always, Untracked),
            entryps_dirstat("d/d/generated", Ignored(Expendable), Directory, Always, Untracked),
            entry("d/generated", Ignored(Expendable), Directory),
            entry("generated", Ignored(Expendable), Directory),
            entry("objs", Ignored(Expendable), Directory),
        ],
        "ignored ones are aggregated, and we get the same effect as with `git status --ignored` - collapsing of untracked happens\
        and we still list the ignored files that were inside.\
        Also note the entries that would be dropped in case of `git clean` are marked with `entry_dirstat`, which would display what's\
        done differently."
    );
    Ok(())
}

#[test]
fn untracked_and_ignored_collapse_handling_mixed() -> crate::Result {
    let root = fixture("subdir-untracked-and-ignored");
    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(CollapseDirectory),
                    emit_untracked: CollapseDirectory,
                    for_deletion: None,
                    ..options()
                },
                keep,
            )
        },
        Some("d/d/b.o"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 19,
        },
    );

    assert_eq!(
        &entries,
        &[entryps("d/d/b.o", Ignored(Expendable), File, Verbatim)],
        "when files are selected individually, they are never collapsed"
    );

    for (spec, pathspec_match) in [("d/d/*", WildcardMatch), ("d/d", Prefix), ("d/d/", Prefix)] {
        let (out, entries) = collect_filtered(
            &root,
            |keep, ctx| {
                walk(
                    &root,
                    &root,
                    ctx,
                    walk::Options {
                        emit_ignored: Some(CollapseDirectory),
                        emit_untracked: CollapseDirectory,
                        for_deletion: None,
                        ..options()
                    },
                    keep,
                )
            },
            Some(spec),
        );
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 4,
                returned_entries: entries.len(),
                seen_entries: 21,
            },
        );

        assert_eq!(
            &entries,
            &[
                entryps("d/d", Untracked, Directory, pathspec_match),
                entryps_dirstat("d/d/a.o", Ignored(Expendable), File, pathspec_match, Untracked),
                entryps_dirstat("d/d/b.o", Ignored(Expendable), File, pathspec_match, Untracked),
                entryps_dirstat(
                    "d/d/generated",
                    Ignored(Expendable),
                    Directory,
                    pathspec_match,
                    Untracked
                ),
            ],
            "with wildcard matches, it's OK to collapse though"
        );
    }
    Ok(())
}

#[test]
fn untracked_and_ignored_collapse_handling_for_deletion_with_wildcards() -> crate::Result {
    let root = fixture("subdir-untracked-and-ignored");
    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(CollapseDirectory),
                    emit_untracked: CollapseDirectory,
                    for_deletion: Some(Default::default()),
                    ..options()
                },
                keep,
            )
        },
        Some("*.o"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 8,
            returned_entries: entries.len(),
            seen_entries: 26
        },
    );
    assert_eq!(
        &entries,
        &[
            entryps("a.o", Ignored(Expendable), File, WildcardMatch),
            entryps("b.o", Ignored(Expendable), File, WildcardMatch),
            entryps("c.o", Ignored(Expendable), File, WildcardMatch),
            entryps("d/a.o", Ignored(Expendable), File, WildcardMatch),
            entryps("d/b.o", Ignored(Expendable), File, WildcardMatch),
            entryps("d/d/a.o", Ignored(Expendable), File, WildcardMatch,),
            entryps("d/d/b.o", Ignored(Expendable), File, WildcardMatch,),
            entryps("generated/a.o", Ignored(Expendable), File, WildcardMatch),
            entryps("objs", Ignored(Expendable), Directory, WildcardMatch),
        ],
        "when using wildcards like these, we actually want to see only the suffixed items even if they all match, like Git does. \
        However, we have no way to differentiate `*` from `*.o`, in which case Git decides to delete the directory instead of its \
        contents, so it's not perfect there either. \
        Thus we stick to the rule: if everything in the directory is going to be deleted, we delete the whole directory."
    );

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(CollapseDirectory),
                    emit_untracked: CollapseDirectory,
                    for_deletion: Some(Default::default()),
                    ..options()
                },
                keep,
            )
        },
        Some("*"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 8,
            returned_entries: entries.len(),
            seen_entries: 28
        },
    );
    assert_eq!(
        &entries,
        &[
            entryps(".gitignore", Untracked, File, WildcardMatch),
            entryps("a.o", Ignored(Expendable), File, WildcardMatch),
            entryps("b.o", Ignored(Expendable), File, WildcardMatch),
            entryps("c.o", Ignored(Expendable), File, WildcardMatch),
            entryps("d/a.o", Ignored(Expendable), File, WildcardMatch),
            entryps("d/b.o", Ignored(Expendable), File, WildcardMatch),
            entryps("d/d", Untracked, Directory, WildcardMatch,),
            entryps_dirstat("d/d/a.o", Ignored(Expendable), File, WildcardMatch, Untracked),
            entryps_dirstat("d/d/b.o", Ignored(Expendable), File, WildcardMatch, Untracked),
            entryps_dirstat(
                "d/d/generated",
                Ignored(Expendable),
                Directory,
                WildcardMatch,
                Untracked
            ),
            entryps("d/generated", Ignored(Expendable), Directory, WildcardMatch),
            entryps("generated", Ignored(Expendable), Directory, WildcardMatch),
            entryps("objs", Ignored(Expendable), Directory, WildcardMatch),
        ],
        "In this case, Git is doing exactly the same"
    );
    Ok(())
}

#[test]
fn untracked_and_ignored_collapse_handling_for_deletion_with_prefix_wildcards() -> crate::Result {
    let root = fixture("subdir-untracked-and-ignored");
    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(CollapseDirectory),
                    emit_untracked: CollapseDirectory,
                    for_deletion: Some(Default::default()),
                    ..options()
                },
                keep,
            )
        },
        Some("generated/*.o"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 12,
        },
    );
    assert_eq!(
        &entries,
        &[entryps("generated/a.o", Ignored(Expendable), File, WildcardMatch)],
        "this is the same result as '*.o', but limited to a subdirectory"
    );
    Ok(())
}

#[test]
fn untracked_and_ignored_collapse_handling_for_deletion_mixed() -> crate::Result {
    let root = fixture("subdir-untracked-and-ignored");
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: None,
                emit_untracked: CollapseDirectory,
                for_deletion: Some(Default::default()),
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 5,
            returned_entries: entries.len(),
            seen_entries: 21,
        },
    );

    assert_eq!(
        &entries,
        &[entry(".gitignore", Untracked, File), entry("d/d/a", Untracked, File)],
        "without ignored files, we only see untracked ones, without a chance to collapse. This actually is something Git fails to do."
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                emit_untracked: CollapseDirectory,
                for_deletion: Some(Default::default()),
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 5,
            returned_entries: entries.len(),
            seen_entries: 24,
        },
    );

    assert_eq!(
        &entries,
        &[
            entry(".gitignore", Untracked, File),
            entry("a.o", Ignored(Expendable), File),
            entry("b.o", Ignored(Expendable), File),
            entry("c.o", Ignored(Expendable), File),
            entry("d/a.o", Ignored(Expendable), File),
            entry("d/b.o", Ignored(Expendable), File),
            entry("d/d", Untracked, Directory),
            entryps_dirstat("d/d/a.o", Ignored(Expendable), File, Always, Untracked),
            entryps_dirstat("d/d/b.o", Ignored(Expendable), File, Always, Untracked),
            entryps_dirstat("d/d/generated", Ignored(Expendable), Directory, Always, Untracked),
            entry("d/generated", Ignored(Expendable), Directory),
            entry("generated", Ignored(Expendable), Directory),
            entry("objs", Ignored(Expendable), Directory),
        ],
        "with ignored files, we can collapse untracked and ignored like before"
    );

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(CollapseDirectory),
                    emit_untracked: CollapseDirectory,
                    for_deletion: Some(Default::default()),
                    ..options()
                },
                keep,
            )
        },
        Some("d/d/*"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 4,
            returned_entries: entries.len(),
            seen_entries: 21,
        },
    );

    assert_eq!(
        &entries,
        &[
            entryps("d/d", Untracked, Directory, WildcardMatch),
            entryps_dirstat("d/d/a.o", Ignored(Expendable), File, WildcardMatch, Untracked),
            entryps_dirstat("d/d/b.o", Ignored(Expendable), File, WildcardMatch, Untracked),
            entryps_dirstat(
                "d/d/generated",
                Ignored(Expendable),
                Directory,
                WildcardMatch,
                Untracked
            ),
        ],
        "everything is filtered down to the pathspec, otherwise it's like before. Not how all-matching collapses"
    );

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(CollapseDirectory),
                    emit_untracked: CollapseDirectory,
                    emit_tracked: true,
                    for_deletion: Some(Default::default()),
                    ..options()
                },
                keep,
            )
        },
        Some("d/d/*.o"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 4,
            returned_entries: entries.len(),
            seen_entries: 20,
        },
    );

    assert_eq!(
        &entries,
        &[
            entryps("d/d/a.o", Ignored(Expendable), File, WildcardMatch),
            entryps("d/d/b.o", Ignored(Expendable), File, WildcardMatch),
        ],
        "If the wildcard doesn't match everything, it can't be collapsed"
    );

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(CollapseDirectory),
                    emit_untracked: CollapseDirectory,
                    for_deletion: Some(Default::default()),
                    ..options()
                },
                keep,
            )
        },
        Some("d/d/"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 4,
            returned_entries: entries.len(),
            seen_entries: 21,
        },
    );

    assert_eq!(
        &entries,
        &[
            entryps("d/d", Untracked, Directory, Prefix),
            entryps_dirstat("d/d/a.o", Ignored(Expendable), File, Prefix, Untracked),
            entryps_dirstat("d/d/b.o", Ignored(Expendable), File, Prefix, Untracked),
            entryps_dirstat("d/d/generated", Ignored(Expendable), Directory, Prefix, Untracked),
        ],
        "a prefix match works similarly, while also listing the dropped content for good measure"
    );

    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: None,
                    emit_untracked: CollapseDirectory,
                    for_deletion: Some(Default::default()),
                    ..options()
                },
                keep,
            )
        },
        Some("d/d/"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 4,
            returned_entries: entries.len(),
            seen_entries: 19,
        },
    );

    assert_eq!(
        &entries,
        &[entryps("d/d/a", Untracked, File, Prefix)],
        "a prefix match works similarly"
    );
    Ok(())
}

#[test]
fn precious_are_not_expendable() {
    let root = fixture("untracked-and-precious");
    let (_out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(Matching),
                emit_untracked: Matching,
                ..options_emit_all()
            },
            keep,
        )
    });
    assert_eq!(
        &entries,
        &[
            entry_nokind(".git", DotGit),
            entry(".gitignore", Tracked, File),
            entry("a.o", Ignored(Expendable), File),
            entry("d/a", Tracked, File),
            entry("d/a.o", Ignored(Expendable), File),
            entry("d/b", Tracked, File),
            entry("d/b.o", Ignored(Expendable), File),
            entry("d/d/a.precious", Ignored(Precious), File),
            entry("d/d/new", Untracked, File),
        ],
        "just to have an overview"
    );
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 10,
        },
    );

    assert_eq!(
        &entries,
        &[
            entry("a.o", Ignored(Expendable), File),
            entry("d/a.o", Ignored(Expendable), File),
            entry("d/b.o", Ignored(Expendable), File),
            entry("d/d", Untracked, Directory),
            entryps_dirstat("d/d/a.precious", Ignored(Precious), File, Always, Untracked),
        ],
        "by default precious files are treated no differently than expendable files, which is fine\
            unless you want to delete `d/d`. Then we shouldn't ever see `d/d` and have to deal with \
            a collapsed precious file."
    );

    for (equivalent_pathspec, expected_match) in [("d/*", WildcardMatch), ("d/", Prefix), ("d", Prefix)] {
        let (out, entries) = collect_filtered(
            &root,
            |keep, ctx| {
                walk(
                    &root,
                    &root,
                    ctx,
                    walk::Options {
                        emit_ignored: Some(CollapseDirectory),
                        emit_untracked: CollapseDirectory,
                        ..options()
                    },
                    keep,
                )
            },
            Some(equivalent_pathspec),
        );
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 3,
                returned_entries: entries.len(),
                seen_entries: 10,
            },
            "{equivalent_pathspec}: should yield same result"
        );

        assert_eq!(
            &entries,
            &[
                entryps("d/a.o", Ignored(Expendable), File, expected_match),
                entryps("d/b.o", Ignored(Expendable), File, expected_match),
                entryps("d/d", Untracked, Directory, expected_match),
                entryps_dirstat("d/d/a.precious", Ignored(Precious), File, expected_match, Untracked),
            ],
            "'{equivalent_pathspec}' should yield the same entries - note how collapsed directories inherit the pathspec"
        );
    }

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                emit_untracked: CollapseDirectory,
                for_deletion: Some(Default::default()),
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 3,
            returned_entries: entries.len(),
            seen_entries: 9,
        },
    );

    assert_eq!(
        &entries,
        &[
            entry("a.o", Ignored(Expendable), File),
            entry("d/a.o", Ignored(Expendable), File),
            entry("d/b.o", Ignored(Expendable), File),
            entry("d/d/a.precious", Ignored(Precious), File),
            entryps("d/d/new", Untracked, File, Always),
        ],
        "If collapses are for deletion, we don't treat precious files like expendable/ignored anymore so they show up individually \
        and prevent collapsing into a folder in the first place"
    );
}

#[test]
#[cfg_attr(
    not(target_vendor = "apple"),
    ignore = "Needs filesystem that folds unicode composition"
)]
fn decomposed_unicode_in_directory_is_returned_precomposed() -> crate::Result {
    let root = gix_testtools::tempfile::TempDir::new()?;

    let decomposed = "a\u{308}";
    let precomposed = "ä";
    std::fs::create_dir(root.path().join(decomposed))?;
    std::fs::write(root.path().join(decomposed).join(decomposed), [])?;

    let (out, entries) = collect(root.path(), |keep, ctx| {
        walk(
            root.path(),
            root.path(),
            ctx,
            walk::Options {
                precompose_unicode: true,
                ..options()
            },
            keep,
        )
    });

    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry(format!("{precomposed}/{precomposed}").as_str(), Untracked, File),
        "even root paths are returned precomposed then"
    );

    let (_out, entries) = collect(root.path(), |keep, ctx| {
        walk(
            &root.path().join(decomposed),
            root.path(),
            ctx,
            walk::Options {
                precompose_unicode: false,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry(format!("{decomposed}/{decomposed}").as_str(), Untracked, File),
        "if disabled, it stays decomposed as provided"
    );
    Ok(())
}

#[test]
fn root_must_be_in_worktree() -> crate::Result {
    let err = try_collect("worktree root does not matter here".as_ref(), |keep, ctx| {
        walk(
            "traversal".as_ref(),
            "unrelated-worktree".as_ref(),
            ctx,
            options(),
            keep,
        )
    })
    .unwrap_err();
    assert!(matches!(err, walk::Error::RootNotInWorktree { .. }));
    Ok(())
}

#[test]
#[cfg_attr(windows, ignore = "symlinks the way they are organized don't yet work on windows")]
fn worktree_root_can_be_symlink() -> crate::Result {
    let root = fixture_in("many-symlinks", "symlink-to-breakout-symlink");
    let (out, entries) = collect(&root, |keep, ctx| walk(&root.join("file"), &root, ctx, options(), keep));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry("file", Untracked, File),
        "it allows symlinks for the worktree itself"
    );
    Ok(())
}

#[test]
fn root_may_not_go_through_dot_git() -> crate::Result {
    let root = fixture("with-nested-dot-git");
    for dir in ["", "subdir"] {
        let (out, entries) = collect(&root, |keep, ctx| {
            walk(
                &root.join("dir").join(".git").join(dir),
                &root,
                ctx,
                options_emit_all(),
                keep,
            )
        });
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 0,
                returned_entries: entries.len(),
                seen_entries: 1,
            }
        );
        assert_eq!(entries.len(), 1, "no traversal happened as root passes though .git");
        assert_eq!(&entries[0], &entry_nomatch("dir/.git", DotGit, Directory));
    }
    Ok(())
}

#[test]
fn root_enters_directory_with_dot_git_in_reconfigured_worktree_tracked() -> crate::Result {
    let root = fixture("nonstandard-worktree");
    let (out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root.join("dir-with-dot-git").join("inside"),
                &root,
                ctx,
                walk::Options {
                    emit_tracked: true,
                    ..options()
                },
                keep,
            )
        },
        None::<&str>,
        Options::git_dir("dir-with-dot-git/.git"),
    )?;

    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );

    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry("dir-with-dot-git/inside", Tracked, File),
        "everything is tracked, so it won't try to detect git repositories anyway"
    );

    let (out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root.join("dir-with-dot-git").join("inside"),
                &root,
                ctx,
                walk::Options {
                    emit_tracked: false,
                    ..options()
                },
                keep,
            )
        },
        None::<&str>,
        Options::git_dir("dir-with-dot-git/.git"),
    )?;

    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: 0,
            seen_entries: 1,
        }
    );

    assert!(entries.is_empty());
    Ok(())
}

#[test]
fn root_enters_directory_with_dot_git_in_reconfigured_worktree_untracked() -> crate::Result {
    let root = fixture("nonstandard-worktree-untracked");
    let (_out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root.join("dir-with-dot-git").join("inside"),
                &root,
                ctx,
                options(),
                keep,
            )
        },
        None::<&str>,
        Options::git_dir("dir-with-dot-git/.git"),
    )?;
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry("dir-with-dot-git/inside", Untracked, File),
        "it can enter a dir and treat it as normal even if own .git is inside,\
         which otherwise would be a repository"
    );
    Ok(())
}

#[test]
fn root_may_not_go_through_nested_repository_unless_enabled() -> crate::Result {
    let root = fixture("nested-repository");
    let walk_root = root.join("nested").join("file");
    let (_out, entries) = collect(&root, |keep, ctx| {
        walk(
            &walk_root,
            &root,
            ctx,
            walk::Options {
                recurse_repositories: true,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry("nested/file", Untracked, File),
        "it happily enters the repository and lists the file"
    );

    let (out, entries) = collect(&root, |keep, ctx| walk(&walk_root, &root, ctx, options(), keep));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry("nested", Untracked, Repository),
        "thus it ends in the directory that is a repository"
    );
    Ok(())
}

#[test]
fn root_may_not_go_through_submodule() -> crate::Result {
    let root = fixture("with-submodule");
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root.join("submodule").join("dir").join("file"),
            &root,
            ctx,
            options_emit_all(),
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1, "it refuses to start traversal in a submodule");
    assert_eq!(
        &entries[0],
        &entry("submodule", Tracked, Repository),
        "thus it ends in the directory that is the submodule"
    );
    Ok(())
}

#[test]
fn walk_with_submodule() -> crate::Result {
    let root = fixture("with-submodule");
    let (out, entries) = collect(&root, |keep, ctx| walk(&root, &root, ctx, options_emit_all(), keep));
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 4,
        }
    );
    assert_eq!(
        entries,
        [
            entry_nokind(".git", DotGit),
            entry(".gitmodules", Tracked, File),
            entry("dir/file", Tracked, File),
            entry("submodule", Tracked, Repository)
        ],
        "thus it ends in the directory that is the submodule"
    );
    Ok(())
}

#[test]
fn root_that_is_tracked_file_is_returned() -> crate::Result {
    let root = fixture("dir-with-tracked-file");
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(&root.join("dir").join("file"), &root, ctx, options_emit_all(), keep)
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );

    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry("dir/file", Tracked, File),
        "a tracked file as root just returns that file (even though no iteration is possible)"
    );
    Ok(())
}

#[test]
fn root_that_is_untracked_file_is_returned() -> crate::Result {
    let root = fixture("dir-with-file");
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(&root.join("dir").join("file"), &root, ctx, options(), keep)
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );

    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry("dir/file", Untracked, File),
        "an untracked file as root just returns that file (even though no iteration is possible)"
    );
    Ok(())
}

#[test]
fn top_level_root_that_is_a_file() {
    let root = fixture("just-a-file");
    let err = try_collect(&root, |keep, ctx| walk(&root, &root, ctx, options(), keep)).unwrap_err();
    assert!(matches!(err, walk::Error::WorktreeRootIsFile { .. }));
}

#[test]
fn root_can_be_pruned_early_with_pathspec() -> crate::Result {
    let root = fixture("dir-with-file");
    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| walk(&root.join("dir"), &root, ctx, options_emit_all(), keep),
        Some("no-match/"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1);

    assert_eq!(
        &entries[0],
        &entry_nomatch("dir", Pruned, Directory),
        "the pathspec didn't match the root, early abort"
    );
    Ok(())
}

#[test]
fn file_root_is_shown_if_pathspec_matches_exactly() -> crate::Result {
    let root = fixture("dir-with-file");
    let (out, entries) = collect_filtered(
        &root,
        |keep, ctx| walk(&root.join("dir").join("file"), &root, ctx, options(), keep),
        Some("*dir/*"),
    );
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1);

    assert_eq!(
        &entries[0],
        &entryps("dir/file", Untracked, File, WildcardMatch),
        "the pathspec matched the root precisely"
    );
    Ok(())
}

#[test]
fn root_that_is_tracked_and_ignored_is_considered_tracked() -> crate::Result {
    let root = fixture("tracked-is-ignored");
    let walk_root = "dir/file";
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(&root.join(walk_root), &root, ctx, options_emit_all(), keep)
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1);

    assert_eq!(
        &entries[0],
        &entry(walk_root, Tracked, File),
        "tracking is checked first, so we can safe exclude checks for most entries"
    );
    Ok(())
}

#[test]
fn root_with_dir_that_is_tracked_and_ignored() -> crate::Result {
    let root = fixture("tracked-is-ignored");
    for emission in [Matching, CollapseDirectory] {
        let (out, entries) = collect(&root, |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_ignored: Some(emission),
                    emit_tracked: true,
                    emit_untracked: emission,
                    ..options_emit_all()
                },
                keep,
            )
        });
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 2,
                returned_entries: entries.len(),
                seen_entries: 3,
            }
        );
        assert_eq!(entries.len(), 3);

        assert_eq!(
            entries,
            [
                entry_nokind(".git", DotGit),
                entry(".gitignore", Tracked, File),
                entry("dir/file", Tracked, File)
            ],
            "'tracked' is the overriding property here, so we even enter ignored directories if they have tracked contents,\
            otherwise we might permanently miss new untracked files in there. Emission mode has no effect"
        );
    }

    Ok(())
}

#[test]
fn empty_and_nested_untracked() -> crate::Result {
    let root = fixture("empty-and-untracked-dir");
    for for_deletion in [None, Some(Default::default())] {
        let (out, entries) = collect(&root, |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_untracked: Matching,
                    for_deletion,
                    emit_empty_directories: true,
                    ..options()
                },
                keep,
            )
        });
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 3,
                returned_entries: entries.len(),
                seen_entries: 2,
            }
        );

        assert_eq!(
            entries,
            [
                entry("empty", Untracked, EmptyDirectory),
                entry("untracked/file", Untracked, File)
            ],
            "we find all untracked entries, no matter the deletion mode"
        );
        let (out, entries) = collect(&root, |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_untracked: CollapseDirectory,
                    emit_empty_directories: true,
                    for_deletion,
                    ..options()
                },
                keep,
            )
        });
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 3,
                returned_entries: entries.len(),
                seen_entries: 3,
            }
        );

        assert_eq!(
            entries,
            [
                entry("empty", Untracked, EmptyDirectory),
                entry("untracked", Untracked, Directory)
            ],
            "we find all untracked directories, no matter the deletion mode"
        );
    }
    Ok(())
}

#[test]
fn root_that_is_ignored_is_listed_for_files_and_directories() -> crate::Result {
    let root = fixture("ignored-dir");
    for walk_root in ["dir", "dir/file"] {
        for emission in [Matching, CollapseDirectory] {
            let (out, entries) = collect(&root, |keep, ctx| {
                walk(
                    &root.join(walk_root),
                    &root,
                    ctx,
                    walk::Options {
                        emit_ignored: Some(emission),
                        ..options()
                    },
                    keep,
                )
            });
            assert_eq!(
                out,
                walk::Outcome {
                    read_dir_calls: 0,
                    returned_entries: entries.len(),
                    seen_entries: 1,
                }
            );
            assert_eq!(entries.len(), 1);

            assert_eq!(
                &entries[0],
                &entry("dir", Ignored(Expendable), Directory),
                "excluded directories or files that walkdir are listed without further recursion"
            );
        }
    }
    Ok(())
}

#[test]
fn nested_bare_repos_in_ignored_directories() -> crate::Result {
    let root = fixture("ignored-dir-with-nested-bare-repository");
    let (_out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                for_deletion: Some(Default::default()),
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    // NOTE: do not use `_out` as `.git` directory contents can change, it's controlled by Git, causing flakiness.

    assert_eq!(
        entries,
        [
            entry(".gitignore", Untracked, File),
            entry("bare", Untracked, Directory),
            entry("dir", Ignored(Expendable), Directory),
        ],
        "by default, only the directory is listed and recursion is stopped there, as it matches the ignore directives. \
        Note the nested bare repository isn't seen, while the bare repository is just collapsed, and not detected as repository"
    );

    let (_out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                for_deletion: Some(ForDeletionMode::FindNonBareRepositoriesInIgnoredDirectories),
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });

    assert_eq!(
        entries,
        [
            entry(".gitignore", Untracked, File),
            entry("bare", Untracked, Directory),
            entry("dir", Ignored(Expendable), Directory),
        ],
        "When looking for non-bare repositories, we won't find bare ones, they just disappear as ignored collapsed directories"
    );

    let (_out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                for_deletion: Some(ForDeletionMode::FindRepositoriesInIgnoredDirectories),
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });

    assert_eq!(
        entries,
        [
            entry(".gitignore", Untracked, File),
            entry("bare", Untracked, Directory),
            entry("dir/file", Ignored(Expendable), File),
            entry("dir/subdir/nested-bare", Ignored(Expendable), Repository),
        ],
        "Only in this mode we are able to find them, but it's expensive"
    );
    Ok(())
}

#[test]
fn nested_repos_in_untracked_directories() -> crate::Result {
    let root = fixture("untracked-hidden-bare");
    let (_out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    // NOTE: do not use `_out` as `.git` directory contents can change, it's controlled by Git, causing flakiness.

    assert_eq!(
        entries,
        [entry("subdir", Untracked, Directory)],
        "by default, the subdir is collapsed and we don't see the contained repository as it doesn't get classified"
    );

    let (_out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_untracked: CollapseDirectory,
                classify_untracked_bare_repositories: true,
                ..options()
            },
            keep,
        )
    });

    assert_eq!(
        entries,
        [
            entry("subdir/file", Untracked, File),
            entry("subdir/hidden-bare", Untracked, Repository)
        ],
        "With this flag we are able to find the bare repository"
    );

    Ok(())
}

#[test]
fn nested_repos_in_ignored_directories() -> crate::Result {
    let root = fixture("ignored-dir-with-nested-repository");
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(Matching),
                for_deletion: Some(Default::default()),
                emit_untracked: CollapseDirectory,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 4,
        }
    );

    assert_eq!(
        entries,
        [
            entry("dir", Ignored(Expendable), Directory),
            entry("objs/a.o", Ignored(Expendable), File),
        ],
        "by default, only the directory is listed and recursion is stopped there, as it matches the ignore directives."
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(Matching),
                emit_untracked: CollapseDirectory,
                for_deletion: Some(ForDeletionMode::FindNonBareRepositoriesInIgnoredDirectories),
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 4,
            returned_entries: entries.len(),
            seen_entries: 6,
        }
    );

    assert_eq!(
        entries,
        [
            entry("dir/file", Ignored(Expendable), File),
            entry("dir/subdir/a", Ignored(Expendable), File),
            entry("dir/subdir/nested", Ignored(Expendable), Repository),
            entry("objs/a.o", Ignored(Expendable), File)
        ],
        "in this mode, we will list repositories nested in ignored directories separately"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                emit_untracked: CollapseDirectory,
                for_deletion: Some(ForDeletionMode::FindNonBareRepositoriesInIgnoredDirectories),
                ..options()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 4,
            returned_entries: entries.len(),
            seen_entries: 7,
        }
    );

    assert_eq!(
        entries,
        [
            entry("dir/file", Ignored(Expendable), File),
            entry("dir/subdir/a", Ignored(Expendable), File),
            entry("dir/subdir/nested", Ignored(Expendable), Repository),
            entry("objs", Ignored(Expendable), Directory),
        ],
        "finally, we can't fold if there are any nested repositories. Note how the folding isn't affected in unrelated directories"
    );
    Ok(())
}

#[test]
#[cfg_attr(
    not(target_vendor = "apple"),
    ignore = "Needs filesystem that folds unicode composition"
)]
fn decomposed_unicode_in_root_is_returned_precomposed() -> crate::Result {
    let root = gix_testtools::tempfile::TempDir::new()?;

    let decomposed = "a\u{308}";
    let precomposed = "ä";
    std::fs::write(root.path().join(decomposed), [])?;

    let (out, entries) = collect(root.path(), |keep, ctx| {
        walk(
            &root.path().join(decomposed),
            root.path(),
            ctx,
            walk::Options {
                precompose_unicode: true,
                ..options()
            },
            keep,
        )
    });

    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 0,
            returned_entries: entries.len(),
            seen_entries: 1,
        }
    );
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry(precomposed, Untracked, File),
        "even root paths are returned precomposed then"
    );

    let (_out, entries) = collect(root.path(), |keep, ctx| {
        walk(
            &root.path().join(decomposed),
            root.path(),
            ctx,
            walk::Options {
                precompose_unicode: false,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(entries.len(), 1);
    assert_eq!(
        &entries[0],
        &entry(decomposed, Untracked, File),
        "if disabled, it stays decomposed as provided"
    );
    Ok(())
}

#[test]
fn untracked_and_ignored_collapse_mix() {
    let root = fixture("untracked-and-ignored-for-collapse");
    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(CollapseDirectory),
                emit_untracked: Matching,
                ..options_emit_all()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 4,
            returned_entries: entries.len(),
            seen_entries: 7,
        }
    );
    assert_eq!(
        entries,
        [
            entry(".gitignore", Untracked, File),
            entry("ignored", Ignored(Expendable), Directory),
            entry("ignored-inside", Ignored(Expendable), Directory),
            entry("mixed/c", Untracked, File),
            entry("mixed/c.o", Ignored(Expendable), File),
            entry("untracked/a", Untracked, File),
        ],
        "ignored collapses separately from untracked"
    );

    let (out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root,
            &root,
            ctx,
            walk::Options {
                emit_ignored: Some(Matching),
                emit_untracked: CollapseDirectory,
                ..options_emit_all()
            },
            keep,
        )
    });
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 4,
            returned_entries: entries.len(),
            seen_entries: 8,
        }
    );
    assert_eq!(
        entries,
        [
            entry(".gitignore", Untracked, File),
            entry("ignored", Ignored(Expendable), Directory),
            entry("ignored-inside/d.o", Ignored(Expendable), File),
            entry("mixed", Untracked, Directory),
            entry_dirstat("mixed/c.o", Ignored(Expendable), File, Untracked),
            entry("untracked", Untracked, Directory),
        ],
        "untracked collapses separately from ignored, but note that matching directories are still emitted, i.e. ignored/"
    );
}

#[test]
fn root_cannot_pass_through_case_altered_capital_dot_git_if_case_insensitive() {
    let root = fixture("with-nested-capitalized-dot-git");
    for dir in ["", "subdir"] {
        let (out, entries) = collect(&root, |keep, ctx| {
            walk(
                &root.join("dir").join(".GIT").join(dir),
                &root,
                ctx,
                walk::Options {
                    ignore_case: true,
                    ..options_emit_all()
                },
                keep,
            )
        });
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 0,
                returned_entries: entries.len(),
                seen_entries: 1,
            }
        );
        assert_eq!(entries.len(), 1, "no traversal happened as root passes though .git");
        assert_eq!(
            &entries[0],
            &entry_nomatch("dir/.GIT", DotGit, Directory),
            "it compares in a case-insensitive fashion"
        );
    }

    let (_out, entries) = collect(&root, |keep, ctx| {
        walk(
            &root.join("dir").join(".GIT").join("config"),
            &root,
            ctx,
            walk::Options {
                ignore_case: false,
                ..options()
            },
            keep,
        )
    });
    assert_eq!(entries.len(), 1,);
    assert_eq!(
        &entries[0],
        &entry("dir/.GIT/config", Untracked, File),
        "it passes right through what now seems like any other directory"
    );
}

#[test]
fn partial_checkout_cone_and_non_one() -> crate::Result {
    for fixture_name in ["partial-checkout-cone-mode", "partial-checkout-non-cone"] {
        let root = fixture(fixture_name);
        let not_in_cone_but_created_locally_by_hand = "d/file-created-manually";
        let (out, entries) = collect(&root, |keep, ctx| {
            walk(
                &root.join(not_in_cone_but_created_locally_by_hand),
                &root,
                ctx,
                options_emit_all(),
                keep,
            )
        });
        assert_eq!(
            out,
            walk::Outcome {
                read_dir_calls: 0,
                returned_entries: entries.len(),
                seen_entries: 1,
            }
        );
        assert_eq!(entries.len(), 1);

        assert_eq!(
            &entries[0],
            &entry("d", TrackedExcluded, Directory),
            "{fixture_name}: we avoid entering excluded sparse-checkout directories even if they are present on disk,\
            no matter with cone or without."
        );
    }
    Ok(())
}

#[test]
fn type_mismatch() {
    let root = fixture("type-mismatch");
    let (out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_tracked: true,
                    emit_untracked: Matching,
                    ..options()
                },
                keep,
            )
        },
        None::<&str>,
        Options {
            fresh_index: false,
            ..Default::default()
        },
    )
    .expect("success");
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 3,
        }
    );
    assert_eq!(entries.len(), 2);

    assert_eq!(
        entries,
        [
            entry("dir-is-file", Untracked, File).with_index_kind(Directory),
            entry("file-is-dir/b", Untracked, File)
        ],
        "as long as the index doesn't claim otherwise (i.e. uptodate) it will handle these changes correctly. \
         Also, `dir-is-file` is tracked as directory, but not as file.\
         The typechange is visible only when there is an entry in the index, of course"
    );

    let (out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_tracked: true,
                    emit_untracked: CollapseDirectory,
                    ..options()
                },
                keep,
            )
        },
        None::<&str>,
        Options {
            fresh_index: false,
            ..Default::default()
        },
    )
    .expect("success");
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 3 + 1,
        }
    );
    assert_eq!(entries.len(), 2);

    assert_eq!(
        entries,
        [
            entry("dir-is-file", Untracked, File).with_index_kind(Directory),
            entry("file-is-dir", Untracked, Directory).with_index_kind(File)
        ],
        "collapsing works as well, and we allow to see the typechange"
    );
}

#[test]
fn type_mismatch_ignore_case() {
    let root = fixture("type-mismatch-icase");
    let (out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_tracked: true,
                    emit_untracked: Matching,
                    ignore_case: true,
                    ..options()
                },
                keep,
            )
        },
        None::<&str>,
        Options {
            fresh_index: false,
            ..Default::default()
        },
    )
    .expect("success");
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 3,
        }
    );
    assert_eq!(
        entries,
        [
            entry("Dir-is-File", Untracked, File).with_index_kind(Directory),
            entry("File-is-Dir/b", Untracked, File)
        ],
        "this is the same as in the non-icase version, which means that icase lookup works"
    );

    let (out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_tracked: true,
                    emit_untracked: CollapseDirectory,
                    ignore_case: true,
                    ..options()
                },
                keep,
            )
        },
        None::<&str>,
        Options {
            fresh_index: false,
            ..Default::default()
        },
    )
    .expect("success");
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 3 + 1,
        }
    );
    assert_eq!(
        entries,
        [
            entry("Dir-is-File", Untracked, File).with_index_kind(Directory),
            entry("File-is-Dir", Untracked, Directory).with_index_kind(File)
        ],
        "this is the same as in the non-icase version, which means that icase lookup works"
    );
}

#[test]
fn type_mismatch_ignore_case_clash_dir_is_file() {
    let root = fixture("type-mismatch-icase-clash-dir-is-file");
    let (out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_tracked: true,
                    emit_untracked: Matching,
                    ignore_case: true,
                    ..options()
                },
                keep,
            )
        },
        None::<&str>,
        Options {
            fresh_index: false,
            ..Default::default()
        },
    )
    .expect("success");
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 1,
            returned_entries: entries.len(),
            seen_entries: 2,
        }
    );
    assert_eq!(
        entries,
        [entry("d", Tracked, File)],
        "file `d` exists on disk and it is found as well. This is just because we prefer finding files over dirs, coincidence"
    );
}

#[test]
fn type_mismatch_ignore_case_clash_file_is_dir() {
    let root = fixture("type-mismatch-icase-clash-file-is-dir");
    let (out, entries) = try_collect_filtered_opts(
        &root,
        |keep, ctx| {
            walk(
                &root,
                &root,
                ctx,
                walk::Options {
                    emit_tracked: true,
                    emit_untracked: CollapseDirectory,
                    ignore_case: true,
                    ..options()
                },
                keep,
            )
        },
        None::<&str>,
        Options {
            fresh_index: false,
            ..Default::default()
        },
    )
    .expect("success");
    assert_eq!(
        out,
        walk::Outcome {
            read_dir_calls: 2,
            returned_entries: entries.len(),
            seen_entries: 2,
        }
    );
    assert_eq!(
        entries,
        [entry("D/a", Tracked, File)],
        "`D` exists on disk as directory, and we manage to to find it in in the index, hence no collapsing happens.\
         If there was no special handling for this, it would have found the file (`d` in the index, icase), which would have been wrong."
    );
}
