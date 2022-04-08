mod parse;
mod matching {
    use bstr::{BStr, ByteSlice};
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
        {
            let input = std::fs::read(dir.join("git-baseline.match")).unwrap();
            let mut seen = BTreeSet::default();
            for git_match in Baseline::new(&input) {
                assert!(seen.insert(git_match), "duplicate match entry: {:?}", git_match);
                assert!(
                    git_match.is_match,
                    "baseline for matches must indeed be matches - check baseline and git version: {:?}",
                    git_match
                );
                let pattern = git_glob::Pattern::from_bytes(git_match.pattern).expect("parsing works");
                assert!(pattern.matches(git_match.value))
            }
        }

        {
            let input = std::fs::read(dir.join("git-baseline.nmatch")).unwrap();
            let mut seen = BTreeSet::default();
            for git_match in Baseline::new(&input) {
                assert!(seen.insert(git_match), "duplicate match entry: {:?}", git_match);
                assert!(
                    !git_match.is_match,
                    "baseline for no-matches must indeed not be matches - check baseline and git version: {:?}",
                    git_match
                );
                let pattern = git_glob::Pattern::from_bytes(git_match.pattern).expect("parsing works");
                assert!(!pattern.matches(git_match.value))
            }
        }
    }

    #[test]
    #[ignore]
    fn check_case_insensitive() {}

    #[test]
    #[ignore]
    fn negated_patterns() {}
}
