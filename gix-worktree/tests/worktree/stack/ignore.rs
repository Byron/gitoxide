use bstr::{BStr, ByteSlice};
use gix_odb::FindExt;
use gix_worktree::{stack::state::ignore::Source, Stack};

use crate::hex_to_id;
use crate::worktree::stack::probe_case;

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
fn exclude_by_dir_is_handled_just_like_git() {
    let dir = gix_testtools::scripted_fixture_read_only_standalone("make_special_exclude_case.sh").unwrap();
    let git_dir = dir.join(".git");

    let mut buf = Vec::new();
    let case = gix_glob::pattern::Case::Sensitive;
    let state = gix_worktree::stack::State::for_add(
        Default::default(),
        gix_worktree::stack::state::Ignore::new(
            Default::default(),
            gix_ignore::Search::from_git_dir(&git_dir, None, &mut buf).unwrap(),
            None,
            Source::WorktreeThenIdMappingIfNotSkipped,
        ),
    );
    let mut cache = Stack::new(&dir, state, case, buf, Default::default());
    let baseline = std::fs::read(git_dir.parent().unwrap().join("git-check-ignore.baseline")).unwrap();
    let expectations = IgnoreExpectations {
        lines: baseline.lines(),
    };
    for (relative_entry, source_and_line) in expectations {
        let (source, line, expected_pattern) = source_and_line.expect("every value is matched");
        let relative_path = gix_path::from_byte_slice(relative_entry);
        let is_dir = dir.join(relative_path).metadata().ok().map(|m| m.is_dir());

        let platform = cache
            .at_entry(relative_entry, is_dir, |_oid, _buf| {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "unreachable"))
            })
            .unwrap();
        let match_ = platform.matching_exclude_pattern().expect("match all values");
        let _is_excluded = platform.is_excluded();
        assert_eq!(
            match_.pattern.to_string(),
            expected_pattern,
            "we perfectly agree with git"
        );
        assert_eq!(
            expected_pattern, "tld/",
            "each entry matches on the main directory exclude, ignoring negations entirely"
        );
        assert_eq!(line, 2);
        assert_eq!(source, ".gitignore");
    }
}

#[test]
fn check_against_baseline() -> crate::Result {
    let dir = gix_testtools::scripted_fixture_read_only_standalone("make_ignore_and_attributes_setup.sh")?;
    let worktree_dir = dir.join("repo");
    let git_dir = worktree_dir.join(".git");
    let mut buf = Vec::new();
    let user_exclude_path = dir.join("user.exclude");
    assert!(user_exclude_path.is_file());

    // Due to the way our setup differs from gits dynamic stack (which involves trying to read files from disk
    // by path) we can only test one case baseline, so we require multiple platforms (or filesystems) to run this.
    let case = probe_case()?;
    let mut index = gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, false, Default::default())?;
    let odb = gix_odb::at(git_dir.join("objects"))?;
    let state = gix_worktree::stack::State::for_add(
        Default::default(),
        gix_worktree::stack::state::Ignore::new(
            gix_ignore::Search::from_overrides(["!force-include"]),
            gix_ignore::Search::from_git_dir(&git_dir, Some(user_exclude_path), &mut buf)?,
            None,
            Source::WorktreeThenIdMappingIfNotSkipped,
        ),
    );
    let paths_storage = index.take_path_backing();
    let attribute_files_in_index = state.id_mappings_from_index(&index, &paths_storage, case);
    assert_eq!(
        attribute_files_in_index,
        vec![(
            "other-dir-with-ignore/.gitignore".into(),
            hex_to_id("5c7e0ed672d3d31d83a3df61f13cc8f7b22d5bfd")
        )]
    );
    let mut cache = Stack::new(&worktree_dir, state, case, buf, attribute_files_in_index);

    let baseline = std::fs::read(git_dir.parent().unwrap().join("git-check-ignore.baseline"))?;
    let expectations = IgnoreExpectations {
        lines: baseline.lines(),
    };
    for (relative_entry, source_and_line) in expectations {
        let relative_path = gix_path::from_byte_slice(relative_entry);
        let is_dir = worktree_dir.join(relative_path).metadata().ok().map(|m| m.is_dir());

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
                if m.source.map_or(false, std::path::Path::exists) {
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
                panic!("actual {actual:?} didn't match {expected:?} at '{relative_entry}'");
            }
        }
    }
    Ok(())
}
