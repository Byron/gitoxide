use std::path::Path;

use bstr::{BStr, ByteSlice};
use git_glob::pattern::Case;
use git_index::entry::Mode;
use git_odb::{pack::bundle::write::Options, FindExt};
use git_testtools::hex_to_id;
use git_worktree::fs;
use tempfile::{tempdir, TempDir};

struct IgnoreExpectations<'a> {
    lines: bstr::Lines<'a>,
}

impl<'a> Iterator for IgnoreExpectations<'a> {
    type Item = (&'a BStr, Option<(&'a BStr, usize, &'a BStr)>);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        let (left, value) = line.split_at(line.find_byte(b'\t').unwrap());
        let value = value[1..].as_bstr();

        let source_and_line = if left == b"::" {
            None
        } else {
            let mut tokens = left.split(|b| *b == b':');
            let source = tokens.next().unwrap().as_bstr();
            let line_number: usize = tokens.next().unwrap().to_str_lossy().parse().ok().unwrap();
            let pattern = tokens.next().unwrap().as_bstr();
            Some((source, line_number, pattern))
        };
        Some((value, source_and_line))
    }
}

#[test]
fn special_exclude_cases_we_handle_differently() {
    let dir = git_testtools::scripted_fixture_read_only("make_special_exclude_case.sh").unwrap();
    let git_dir = dir.join(".git");

    let mut buf = Vec::new();
    let case = git_glob::pattern::Case::Sensitive;
    let state = git_worktree::fs::cache::State::for_add(
        Default::default(),
        git_worktree::fs::cache::state::Ignore::new(
            Default::default(),
            git_attributes::MatchGroup::from_git_dir(&git_dir, None, &mut buf).unwrap(),
            None,
            case,
        ),
    );
    let mut cache = fs::Cache::new(&dir, state, case, buf, Default::default());
    let baseline = std::fs::read(git_dir.parent().unwrap().join("git-check-ignore.baseline")).unwrap();
    let expectations = IgnoreExpectations {
        lines: baseline.lines(),
    };
    for (relative_entry, source_and_line) in expectations {
        let (source, line, pattern) = source_and_line.expect("every value is matched");
        assert_eq!(
            pattern, "tld/",
            "each entry matches on the main directory exclude, ignoring negations entirely"
        );
        assert_eq!(line, 2);
        assert_eq!(source, ".gitignore");

        let relative_path = git_path::from_byte_slice(relative_entry);
        let is_dir = dir.join(&relative_path).metadata().ok().map(|m| m.is_dir());

        let platform = cache
            .at_entry(relative_entry, is_dir, |oid, buf| {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "unreachable"))
            })
            .unwrap();
        let match_ = platform.matching_exclude_pattern().expect("match all values");
        let is_excluded = platform.is_excluded();

        match relative_entry.as_ref() {
            b"tld" | b"tld/" | b"tld/file" | b"tld/sd" | b"tld/sd/" => {
                assert_eq!(match_.pattern.to_string(), "tld/");
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn check_against_baseline() -> crate::Result {
    let dir = git_testtools::scripted_fixture_read_only("make_ignore_and_attributes_setup.sh")?;
    let worktree_dir = dir.join("repo");
    let git_dir = worktree_dir.join(".git");
    let mut buf = Vec::new();
    let user_exclude_path = dir.join("user.exclude");
    assert!(user_exclude_path.is_file());

    let mut index = git_index::File::at(git_dir.join("index"), git_hash::Kind::Sha1, Default::default())?;
    let odb = git_odb::at(git_dir.join("objects"))?;
    let case = git_glob::pattern::Case::Sensitive;
    let state = git_worktree::fs::cache::State::for_add(
        Default::default(), // TODO: attribute tests
        git_worktree::fs::cache::state::Ignore::new(
            git_attributes::MatchGroup::from_overrides(vec!["!force-include"]),
            git_attributes::MatchGroup::from_git_dir(&git_dir, Some(user_exclude_path), &mut buf)?,
            None,
            case,
        ),
    );
    let paths_storage = index.take_path_backing();
    let attribute_files_in_index = state.build_attribute_list(&index, &paths_storage, case);
    assert_eq!(
        attribute_files_in_index,
        vec![(
            "other-dir-with-ignore/.gitignore".as_bytes().as_bstr(),
            hex_to_id("5c7e0ed672d3d31d83a3df61f13cc8f7b22d5bfd")
        )]
    );
    let mut cache = fs::Cache::new(&worktree_dir, state, case, buf, attribute_files_in_index);

    let baseline = std::fs::read(git_dir.parent().unwrap().join("git-check-ignore.baseline"))?;
    let expectations = IgnoreExpectations {
        lines: baseline.lines(),
    };
    for (relative_entry, source_and_line) in expectations {
        let relative_path = git_path::from_byte_slice(relative_entry);
        let is_dir = worktree_dir.join(&relative_path).metadata().ok().map(|m| m.is_dir());

        let platform = cache.at_entry(relative_entry, is_dir, |oid, buf| odb.find_blob(oid, buf))?;

        let match_ = platform.matching_exclude_pattern();
        let is_excluded = platform.is_excluded();
        match (match_, source_and_line) {
            (None, None) => {
                assert!(!is_excluded);
            }
            (Some(m), Some((source_file, line, pattern))) => {
                assert_eq!(m.pattern.to_string(), pattern);
                assert_eq!(m.sequence_number, line);
                // Paths read from the index are relative to the repo, and they don't exist locally due tot skip-worktree
                if m.source.map_or(false, |p| p.exists()) {
                    assert_eq!(
                        m.source.map(|p| p.canonicalize().unwrap()),
                        Some(worktree_dir.join(source_file.to_str_lossy().as_ref()).canonicalize()?)
                    );
                }
            }
            (Some(actual), None) if actual.pattern.is_negative() => {
                // OK: we provide negative patterns that matched on paths if there was no other match, while git doesn't.
            }
            (actual, expected) => {
                panic!(
                    "actual {:?} didn't match {:?} at '{}'",
                    actual, expected, relative_entry
                );
            }
        }
    }

    cache.set_case(Case::Fold);
    let platform = cache.at_entry("User-file-ANYWHERE", Some(false), |oid, buf| odb.find_blob(oid, buf))?;
    let m = platform.matching_exclude_pattern().expect("match");
    assert_eq!(m.pattern.text, "user-file-anywhere");
    Ok(())
}
