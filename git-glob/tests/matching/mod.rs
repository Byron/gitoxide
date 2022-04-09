use bstr::{BStr, ByteSlice};
use git_glob::pattern;
use std::collections::BTreeSet;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct GitMatch<'a> {
    pattern: &'a BStr,
    value: &'a BStr,
    /// True if git could match `value` with `pattern`
    is_match: bool,
}

pub struct Baseline<'a> {
    inner: bstr::Lines<'a>,
}

impl<'a> Iterator for Baseline<'a> {
    type Item = GitMatch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tokens = self.inner.next()?.splitn(2, |b| *b == b' ');
        let pattern = tokens.next().expect("pattern").as_bstr();
        let value = tokens.next().expect("value").as_bstr().trim_start().as_bstr();

        let git_match = self.inner.next()?;
        let is_match = !git_match.starts_with(b"::\t");
        Some(GitMatch {
            pattern,
            value,
            is_match,
        })
    }
}

impl<'a> Baseline<'a> {
    fn new(input: &'a [u8]) -> Self {
        Baseline {
            inner: input.as_bstr().lines(),
        }
    }
}

#[test]
#[ignore]
fn compare_baseline_with_ours() {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_baseline.sh").unwrap();
    for (input_file, expected_matches) in &[("git-baseline.match", true), ("git-baseline.nmatch", false)] {
        let input = std::fs::read(dir.join(*input_file)).unwrap();
        let mut seen = BTreeSet::default();

        for m @ GitMatch {
            pattern,
            value,
            is_match,
        } in Baseline::new(&input)
        {
            assert!(seen.insert(m), "duplicate match entry: {:?}", m);
            assert_eq!(
                is_match, *expected_matches,
                "baseline for matches must indeed be {} - check baseline and git version: {:?}",
                expected_matches, m
            );
            let pattern = git_glob::Pattern::from_bytes(pattern).expect("parsing works");
            assert_eq!(
                pattern.matches_path(
                    value,
                    value.rfind_byte(b'/').map(|pos| pos + 1),
                    false, // TODO: does it make sense to pretent it is a dir and see what happens?
                    pattern::Case::Sensitive
                ),
                is_match
            )
        }
    }
}

#[test]
#[ignore]
fn check_case_insensitive() {}

#[test]
#[ignore]
fn negated_patterns() {}
