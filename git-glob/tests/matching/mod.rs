use std::collections::BTreeSet;

use bstr::{BStr, ByteSlice};
use git_glob::{pattern, pattern::Case};

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
fn compare_baseline_with_ours() {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_baseline.sh").unwrap();
    let (mut total_matches, mut total_correct, mut panics) = (0, 0, 0);
    let mut mismatches = Vec::new();
    for (input_file, expected_matches) in &[("git-baseline.match", true), ("git-baseline.nmatch", false)] {
        let input = std::fs::read(dir.join(*input_file)).unwrap();
        let mut seen = BTreeSet::default();

        for m @ GitMatch {
            pattern,
            value,
            is_match,
        } in Baseline::new(&input)
        {
            total_matches += 1;
            assert!(seen.insert(m), "duplicate match entry: {:?}", m);
            assert_eq!(
                is_match, *expected_matches,
                "baseline for matches must indeed be {} - check baseline and git version: {:?}",
                expected_matches, m
            );
            match std::panic::catch_unwind(|| {
                let pattern = pat(pattern);
                pattern.matches_repo_relative_path(
                    value,
                    basename_start_pos(value),
                    false, // TODO: does it make sense to pretend it is a dir and see what happens?
                    pattern::Case::Sensitive,
                )
            }) {
                Ok(actual_match) => {
                    if actual_match == is_match {
                        total_correct += 1;
                    } else {
                        mismatches.push((pattern.to_owned(), value.to_owned(), is_match));
                    }
                }
                Err(_) => {
                    panics += 1;
                    continue;
                }
            };
        }
    }

    assert_eq!(
        total_correct,
        total_matches - panics,
        "We perfectly agree with git here"
    );
    assert_eq!(panics, 0);
}

#[test]
fn non_dirs_for_must_be_dir_patterns_are_ignored() {
    let pattern = pat("hello/");

    assert!(pattern.mode.contains(pattern::Mode::MUST_BE_DIR));
    assert_eq!(
        pattern.text, "hello",
        "a dir pattern doesn't actually end with the trailing slash"
    );
    let path = "hello";
    assert!(
        !pattern.matches_repo_relative_path(path, None, false /* is-dir */, Case::Sensitive),
        "non-dirs never match a dir pattern"
    );
    assert!(
        pattern.matches_repo_relative_path(path, None, true /* is-dir */, Case::Sensitive),
        "dirs can match a dir pattern with the normal rules"
    );
}

#[test]
fn basename_matches_from_end() {
    let pat = &pat("foo");
    assert!(match_file(pat, "FoO", Case::Fold));
    assert!(!match_file(pat, "FoOo", Case::Fold));
    assert!(!match_file(pat, "Foo", Case::Sensitive));
    assert!(match_file(pat, "foo", Case::Sensitive));
    assert!(!match_file(pat, "Foo", Case::Sensitive));
    assert!(!match_file(pat, "barfoo", Case::Sensitive));
}

#[test]
#[should_panic]
fn base_path_must_match_or_panic_occours_in_debug_mode() {
    let pat = pat("foo").with_base("base/");
    match_file(&pat, "other/FoO", Case::Fold);
}

#[test]
fn absolute_basename_matches_only_from_beginning() {
    let mut pattern = pat("/foo");
    let pat = &pattern;
    assert!(match_file(pat, "FoO", Case::Fold));
    assert!(!match_file(pat, "bar/Foo", Case::Fold));
    assert!(match_file(pat, "foo", Case::Sensitive));
    assert!(!match_file(pat, "Foo", Case::Sensitive));
    assert!(!match_file(pat, "bar/foo", Case::Sensitive));

    pattern = pattern.with_base("base/");
    let pat = &pattern;
    assert!(match_file(pat, "base/FoO", Case::Fold));
    assert!(!match_file(pat, "base/bar/Foo", Case::Fold));
    assert!(match_file(pat, "base/foo", Case::Sensitive));
    assert!(!match_file(pat, "base/Foo", Case::Sensitive));
    assert!(!match_file(pat, "base/bar/foo", Case::Sensitive));
}

#[test]
fn absolute_path_matches_only_from_beginning() {
    let mut pattern = pat("/bar/foo");
    let pat = &pattern;
    assert!(!match_file(pat, "FoO", Case::Fold));
    assert!(match_file(pat, "bar/Foo", Case::Fold));
    assert!(!match_file(pat, "foo", Case::Sensitive));
    assert!(match_file(pat, "bar/foo", Case::Sensitive));
    assert!(!match_file(pat, "bar/Foo", Case::Sensitive));

    pattern = pattern.with_base("base/");
    let pat = &pattern;
    assert!(!match_file(pat, "base/FoO", Case::Fold));
    assert!(match_file(pat, "base/bar/Foo", Case::Fold));
    assert!(!match_file(pat, "base/foo", Case::Sensitive));
    assert!(match_file(pat, "base/bar/foo", Case::Sensitive));
    assert!(!match_file(pat, "base/bar/Foo", Case::Sensitive));
}

#[test]
fn absolute_path_with_recursive_glob_detects_mismatches_quickly() {
    let mut pattern = pat("/bar/foo/**");
    let pat = &pattern;
    assert!(!match_file(pat, "FoO", Case::Fold));
    assert!(!match_file(pat, "bar/Fooo", Case::Fold));
    assert!(!match_file(pat, "baz/bar/Foo", Case::Fold));

    pattern = pattern.with_base("base/");
    let pat = &pattern;
    assert!(!match_file(pat, "base/FoO", Case::Fold));
    assert!(!match_file(pat, "base/bar/Fooo", Case::Fold));
    assert!(!match_file(pat, "base/baz/bar/foo", Case::Sensitive));
}

#[test]
fn absolute_path_with_recursive_glob_can_do_case_insensitive_prefix_search() {
    let mut pattern = pat("/bar/foo/**");
    let pat = &pattern;
    assert!(!match_file(pat, "bar/Foo/match", Case::Sensitive));
    assert!(match_file(pat, "bar/Foo/match", Case::Fold));

    pattern = pattern.with_base("base/");
    let pat = &pattern;
    assert!(!match_file(pat, "base/bar/Foo/match", Case::Sensitive));
    assert!(match_file(pat, "base/bar/Foo/match", Case::Fold));
}

#[test]
fn relative_path_does_not_match_from_end() {
    let pattern = &pat("bar/foo");
    assert!(!match_file(pattern, "FoO", Case::Fold));
    assert!(match_file(pattern, "bar/Foo", Case::Fold));
    assert!(!match_file(pattern, "baz/bar/Foo", Case::Fold));
    assert!(!match_file(pattern, "foo", Case::Sensitive));
    assert!(match_file(pattern, "bar/foo", Case::Sensitive));
    assert!(!match_file(pattern, "baz/bar/foo", Case::Sensitive));
    assert!(!match_file(pattern, "Baz/bar/Foo", Case::Sensitive));
}

#[test]
fn basename_glob_and_literal_is_ends_with() {
    let pattern = &pat("*foo");
    assert!(match_file(pattern, "FoO", Case::Fold));
    assert!(match_file(pattern, "BarFoO", Case::Fold));
    assert!(!match_file(pattern, "BarFoOo", Case::Fold));
    assert!(!match_file(pattern, "Foo", Case::Sensitive));
    assert!(!match_file(pattern, "BarFoo", Case::Sensitive));
    assert!(match_file(pattern, "barfoo", Case::Sensitive));
    assert!(!match_file(pattern, "barfooo", Case::Sensitive));

    assert!(match_file(pattern, "bar/foo", Case::Sensitive));
    assert!(match_file(pattern, "bar/bazfoo", Case::Sensitive));
}

#[test]
fn special_cases_from_corpus() {
    let pattern = &pat("foo*bar");
    assert!(
        !match_file(pattern, "foo/baz/bar", Case::Sensitive),
        "asterisk does not match path separators"
    );
    let pattern = &pat("*some/path/to/hello.txt");
    assert!(
        !match_file(pattern, "a/bigger/some/path/to/hello.txt", Case::Sensitive),
        "asterisk doesn't match path separators"
    );

    let pattern = &pat("/*foo.txt");
    assert!(match_file(pattern, "hello-foo.txt", Case::Sensitive));
    assert!(
        !match_file(pattern, "hello/foo.txt", Case::Sensitive),
        "absolute single asterisk doesn't match paths"
    );
}

#[test]
fn absolute_basename_glob_and_literal_is_ends_with_in_basenames() {
    let pattern = &pat("/*foo");

    assert!(match_file(pattern, "FoO", Case::Fold));
    assert!(match_file(pattern, "BarFoO", Case::Fold));
    assert!(!match_file(pattern, "BarFoOo", Case::Fold));
    assert!(!match_file(pattern, "Foo", Case::Sensitive));
    assert!(!match_file(pattern, "BarFoo", Case::Sensitive));
    assert!(match_file(pattern, "barfoo", Case::Sensitive));
    assert!(!match_file(pattern, "barfooo", Case::Sensitive));
}

#[test]
fn absolute_basename_glob_and_literal_is_glob_in_paths() {
    let pattern = &pat("/*foo");

    assert!(!match_file(pattern, "bar/foo", Case::Sensitive), "* does not match /");
    assert!(!match_file(pattern, "bar/bazfoo", Case::Sensitive));
}

#[test]
fn negated_patterns_are_handled_by_caller() {
    let pattern = &pat("!foo");
    assert!(
        match_file(pattern, "foo", Case::Sensitive),
        "negative patterns match like any other"
    );
    assert!(
        pattern.is_negative(),
        "the caller checks for the negative flag and acts accordingly"
    );
}

fn pat<'a>(pattern: impl Into<&'a BStr>) -> git_glob::Pattern {
    git_glob::Pattern::from_bytes(pattern.into()).expect("parsing works")
}

fn match_file<'a>(pattern: &git_glob::Pattern, path: impl Into<&'a BStr>, case: Case) -> bool {
    match_path(pattern, path, false, case)
}

fn match_path<'a>(pattern: &git_glob::Pattern, path: impl Into<&'a BStr>, is_dir: bool, case: Case) -> bool {
    let path = path.into();
    pattern.matches_repo_relative_path(path, basename_start_pos(path), is_dir, case)
}

fn basename_start_pos(value: &BStr) -> Option<usize> {
    value.rfind_byte(b'/').map(|pos| pos + 1)
}
