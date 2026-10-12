use std::{fmt::Write, fs, path::Path};

use bstr::ByteSlice;
use gix_error::{ResultExt, bail, message};
use gix_glob::pattern::Case;
use gix_index::entry::Flags;
use gix_testtools::TestResult;

#[test]
fn patterns_match_git_baseline() -> TestResult {
    let fixture = gix_testtools::scripted_fixture_read_only("make_sparse_baseline.sh")?;
    let git_version = fs::read_to_string(fixture.join("git-version"))?;
    let scenarios = fs::read_to_string(fixture.join("scenarios"))?;
    let mut failures = String::new();
    let mut count = 0;
    for name in scenarios.lines() {
        count += 1;
        let dir = fixture.join(name);
        if let Err(err) = check_scenario(&dir) {
            writeln!(
                failures,
                "
Scenario: {name}
Fixture: {}
{err:?}",
                dir.display()
            )?;
        }
    }
    assert_eq!(count, 18, "the fixture must retain all original sparse scenarios");
    assert!(
        failures.is_empty(),
        "sparse patterns differ from {}:
{failures}",
        git_version.trim()
    );
    Ok(())
}

fn check_scenario(dir: &Path) -> TestResult {
    let description = fs::read_to_string(dir.join("description"))?;
    let config = fs::read_to_string(dir.join("config"))?;
    let mut settings = config.lines();
    let cone: bool = settings.next().expect("fixture records cone mode").parse()?;
    let ignore_case: bool = settings.next().expect("fixture records ignorecase").parse()?;
    assert!(
        settings.next().is_none(),
        "fixture config contains exactly two settings"
    );
    let source = dir.join("patterns");
    let patterns = fs::read(&source)?;
    let status = fs::read_to_string(dir.join("status"))?;
    let method = fs::read_to_string(dir.join("method"))?;
    let stderr = fs::read(dir.join("stderr"))?;
    let mut context = format!(
        "{}
cone={cone}, ignorecase={ignore_case}
Git method: {}, read-tree status: {}
Git stderr:
{}
Patterns: {:?}",
        description.trim(),
        method.trim(),
        status.trim(),
        stderr.as_bstr(),
        patterns.as_bstr()
    );
    if method.trim() == "reapply" {
        writeln!(
            context,
            "
Git reapply status: {}
Git reapply stderr:
{}",
            fs::read_to_string(dir.join("reapply.status"))?.trim(),
            fs::read(dir.join("reapply.stderr"))?.as_bstr()
        )?;
    }
    let object_hash = gix_testtools::object_hash();
    let mut actual = gix_index::File::at(dir.join("index.before"), object_hash, false, Default::default())?;
    let expected = gix_index::File::at(dir.join("index.after"), object_hash, false, Default::default())?;
    for entry in actual.entries_mut() {
        assert!(
            !entry.flags.contains(Flags::SKIP_WORKTREE),
            "baseline input must be a full, initially non-sparse index: {context}"
        );
        // UPTODATE is transient and cannot be recorded in Git's on-disk index.
        entry.flags.insert(Flags::UPTODATE);
    }
    gix_worktree::sparse::apply_patterns(
        &mut actual,
        &patterns,
        &source,
        cone,
        if ignore_case { Case::Fold } else { Case::Sensitive },
    )
    .or_raise(|| message!("Could not apply patterns: {context}"))?;

    let mut differences = String::new();
    if actual.entries().len() != expected.entries().len() {
        writeln!(
            differences,
            "entry count: gix={}, Git={}",
            actual.entries().len(),
            expected.entries().len()
        )?;
    }
    for (actual_entry, expected_entry) in actual.entries().iter().zip(expected.entries()) {
        let actual_path = actual_entry.path(&actual);
        let expected_path = expected_entry.path(&expected);
        let expected_flags = expected_entry.flags | Flags::UPTODATE;
        if actual_path != expected_path
            || actual_entry.mode != expected_entry.mode
            || actual_entry.id != expected_entry.id
            || actual_entry.flags != expected_flags
        {
            writeln!(
                differences,
                "path {actual_path:?}:
  gix: mode={:?}, id={}, flags={:?}
  Git: path={expected_path:?}, mode={:?}, id={}, flags={expected_flags:?}",
                actual_entry.mode, actual_entry.id, actual_entry.flags, expected_entry.mode, expected_entry.id,
            )?;
        }
        if actual_entry.flags.contains(Flags::SKIP_WORKTREE) != actual_entry.flags.contains(Flags::EXTENDED) {
            writeln!(differences, "path {actual_path:?}: SKIP_WORKTREE requires EXTENDED")?;
        }
    }

    let baseline = fs::read(dir.join("baseline"))?;
    let actual_listing = listing(&actual);
    if actual_listing != baseline {
        writeln!(
            differences,
            "git ls-files -t (H=included, S=skipped):
  gix: {:?}
  Git: {:?}",
            actual_listing
                .split(|byte| *byte == 0)
                .map(ByteSlice::as_bstr)
                .collect::<Vec<_>>(),
            baseline
                .split(|byte| *byte == 0)
                .map(ByteSlice::as_bstr)
                .collect::<Vec<_>>(),
        )?;
    }
    if !differences.is_empty() {
        bail!(message!(
            "{context}
Differences:
{differences}"
        ));
    }
    Ok(())
}

fn listing(index: &gix_index::State) -> Vec<u8> {
    let mut out = Vec::new();
    for entry in index.entries() {
        out.extend_from_slice(if entry.flags.contains(Flags::SKIP_WORKTREE) {
            b"S "
        } else {
            b"H "
        });
        out.extend_from_slice(entry.path(index));
        out.push(0);
    }
    out
}

mod apply_patterns {
    use std::path::Path;

    use gix_glob::pattern::Case;
    use gix_index::entry::Flags;
    use gix_testtools::TestResult;
    use gix_worktree::sparse::apply_patterns;

    #[test]
    fn non_cone_directory_patterns_include_gitlinks_on_every_host() -> TestResult {
        use gix_index::entry::Mode;

        let hash = gix_hash::Kind::shortest();
        let mut index = gix_index::State::new(hash);
        let initial = Flags::ASSUME_VALID | Flags::UPTODATE;
        for (path, mode) in [
            ("file", Mode::FILE),
            ("executable", Mode::FILE_EXECUTABLE),
            ("link", Mode::SYMLINK),
            ("module", Mode::COMMIT),
            ("nested/file", Mode::FILE),
        ] {
            index.dangerously_push_entry(Default::default(), hash.null(), initial, mode, path.into());
        }
        for patterns in [b"/*/\n".as_slice(), b"/*\n!/module/\n"] {
            let mut actual = index.clone();
            apply_patterns(
                &mut actual,
                patterns,
                Path::new("sparse-checkout"),
                false,
                Case::Sensitive,
            )?;
            for entry in actual.entries() {
                let included = if patterns == b"/*/\n" {
                    entry.mode == Mode::COMMIT || entry.path(&actual) == "nested/file"
                } else {
                    entry.mode != Mode::COMMIT
                };
                assert_eq!(
                    entry.flags,
                    if included {
                        initial
                    } else {
                        initial | Flags::SKIP_WORKTREE | Flags::EXTENDED
                    },
                    "{}: directory-only patterns include or exclude absent gitlinks on every host",
                    entry.path(&actual)
                );
            }
        }
        Ok(())
    }

    #[test]
    fn cone_cache_resets_at_directory_boundaries_and_on_out_of_order_paths() -> TestResult {
        assert_membership(
            b"/a/\n",
            true,
            Case::Sensitive,
            &[
                ("a/b/c/file", true),
                ("a/file", true),
                ("a/bb/file", true),
                ("a-b/file", false),
                ("z/file", false),
                ("a/b/d/file", true),
                ("root", true),
            ],
        )
    }

    #[test]
    fn folded_cone_cache_handles_byte_sorted_paths_with_different_folded_order() -> TestResult {
        assert_membership(
            b"/a/\n",
            true,
            Case::Fold,
            &[("A/x/file", true), ("B/x/file", false), ("a/y/file", true)],
        )
    }

    #[test]
    fn explicit_non_cone_file_matches_do_not_poison_cached_ancestors() -> TestResult {
        assert_membership(
            b"/a/
!/a/excluded/
/a/excluded/keep
/a/excluded/reincluded/
!/a/excluded/reincluded/drop
",
            false,
            Case::Sensitive,
            &[
                ("a/excluded/keep", true),
                ("a/excluded/other", false),
                ("a/excluded/reincluded/drop", false),
                ("a/excluded/reincluded/keep", true),
                ("a/file", true),
                ("ab/file", false),
                ("a/excluded/another", false),
            ],
        )
    }

    #[test]
    fn non_cone_nearest_ancestor_wins_even_if_its_pattern_is_earlier() -> TestResult {
        assert_membership(
            b"/a/b/
!/a/
",
            false,
            Case::Sensitive,
            &[("a/b/file", true), ("a/b/other", true), ("a/other", false)],
        )
    }

    #[test]
    fn full_cone_fast_path_requires_valid_cone_patterns_and_preserves_flags() -> TestResult {
        let paths = [("nested/file", true), ("root", true)];
        assert_membership(b"/*\n", true, Case::Sensitive, &paths)?;
        assert_membership(
            b"/*
!/nested/
",
            true,
            Case::Sensitive,
            &[("nested/file", false), ("root", true)],
        )
    }

    fn assert_membership(patterns: &[u8], cone: bool, case: Case, paths: &[(&str, bool)]) -> TestResult {
        let hash = gix_hash::Kind::shortest();
        let mut index = gix_index::State::new(hash);
        let initial = Flags::ASSUME_VALID | Flags::UPTODATE;
        for &(path, _) in paths {
            index.dangerously_push_entry(
                Default::default(),
                hash.null(),
                initial,
                gix_index::entry::Mode::FILE,
                path.into(),
            );
        }
        apply_patterns(&mut index, patterns, Path::new("sparse-checkout"), cone, case)?;
        for (entry, &(path, included)) in index.entries().iter().zip(paths) {
            assert_eq!(entry.path(&index), path, "matching must not reorder index entries");
            assert_eq!(
                entry.flags,
                if included {
                    initial
                } else {
                    initial | Flags::SKIP_WORKTREE | Flags::EXTENDED
                },
                "{path}: only excluded entries gain sparse flags; unrelated flags are preserved"
            );
        }
        Ok(())
    }
}
