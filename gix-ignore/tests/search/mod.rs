use std::io::Read;

use bstr::{BStr, ByteSlice};
use gix_glob::pattern::Case;
use gix_ignore::search::Match;

struct Expectations<'a> {
    lines: bstr::Lines<'a>,
}

impl<'a> Iterator for Expectations<'a> {
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
fn baseline_from_git_dir() -> crate::Result {
    let case = if gix_fs::Capabilities::probe("../.git".as_ref()).ignore_case {
        Case::Fold
    } else {
        Case::Sensitive
    };
    let dir = gix_testtools::scripted_fixture_read_only("make_global_and_external_and_dir_ignores.sh")?;
    let repo_dir = dir.join("repo");
    let git_dir = repo_dir.join(".git");
    let baseline = std::fs::read(git_dir.parent().unwrap().join("git-check-ignore.baseline"))?;
    let mut buf = Vec::new();
    let mut group = gix_ignore::Search::from_git_dir(&git_dir, Some(dir.join("user.exclude")), &mut buf)?;

    assert!(
        !gix_glob::search::add_patterns_file(&mut group.patterns, "not-a-file".into(), false, None, &mut buf)?,
        "missing files are no problem and cause a negative response"
    );
    assert!(
        gix_glob::search::add_patterns_file(
            &mut group.patterns,
            repo_dir.join(".gitignore"),
            true,
            repo_dir.as_path().into(),
            &mut buf
        )?,
        "existing files return true"
    );

    buf.clear();
    let ignore_file = repo_dir.join("dir-with-ignore").join(".gitignore");
    std::fs::File::open(&ignore_file)?.read_to_end(&mut buf)?;
    group.add_patterns_buffer(&buf, ignore_file, repo_dir.as_path().into());

    for (path, source_and_line) in (Expectations {
        lines: baseline.lines(),
    }) {
        let actual = group.pattern_matching_relative_path(
            path,
            repo_dir
                .join(path.to_str_lossy().as_ref())
                .metadata()
                .ok()
                .map(|m| m.is_dir()),
            case,
        );
        match (actual, source_and_line) {
            (
                Some(Match {
                    sequence_number,
                    pattern: _,
                    source,
                }),
                Some((expected_source, line, _expected_pattern)),
            ) => {
                assert_eq!(sequence_number, line, "our counting should match the one used in git");
                assert_eq!(
                    source.map(|p| p.canonicalize().unwrap()),
                    Some(repo_dir.join(expected_source.to_str_lossy().as_ref()).canonicalize()?)
                );
            }
            (None, None) => {}
            (actual, expected) => {
                panic!("{case:?}: actual {actual:?} should match {expected:?} with path '{path}'")
            }
        }
    }
    Ok(())
}

#[test]
fn from_overrides() {
    let input = ["simple", "pattern/"];
    let group = gix_ignore::Search::from_overrides(input.iter());
    assert_eq!(
        group.pattern_matching_relative_path("Simple".into(), None, gix_glob::pattern::Case::Fold),
        Some(pattern_to_match(&gix_glob::parse("simple").unwrap(), 0))
    );
    assert_eq!(
        group.pattern_matching_relative_path("pattern".into(), Some(true), gix_glob::pattern::Case::Sensitive),
        Some(pattern_to_match(&gix_glob::parse("pattern/").unwrap(), 1))
    );
    assert_eq!(group.patterns.len(), 1);
    assert_eq!(
        gix_ignore::Search::from_overrides(input).patterns[0],
        group.patterns.into_iter().next().unwrap()
    );
}

fn pattern_to_match(pattern: &gix_glob::Pattern, sequence_number: usize) -> Match<'_> {
    Match {
        pattern,
        source: None,
        sequence_number,
    }
}
