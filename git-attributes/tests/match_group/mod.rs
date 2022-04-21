mod ignore {
    use bstr::{BStr, ByteSlice};
    use git_attributes::{Ignore, Match, MatchGroup};

    struct Expectations<'a> {
        lines: bstr::Lines<'a>,
    }

    impl<'a> Iterator for Expectations<'a> {
        type Item = (&'a BStr, Option<(&'a BStr, usize)>);

        fn next(&mut self) -> Option<Self::Item> {
            let line = self.lines.next()?;
            let (left, value) = line.split_at(line.find_byte(b'\t')?);
            let value = value[1..].as_bstr();

            let source_and_line = if left == b"::" {
                None
            } else {
                let mut tokens = left.split(|b| *b == b':');
                let source = tokens.next()?.as_bstr();
                let line_number: usize = tokens.next()?.to_str_lossy().parse().ok()?;
                Some((source, line_number))
            };
            Some((value, source_and_line))
        }
    }

    #[test]
    fn from_git_dir() {
        let dir = git_testtools::scripted_fixture_repo_read_only("make_global_ignores_and_external_ignore.sh").unwrap();
        let git_dir = dir.join("repo").join(".git");
        let baseline = std::fs::read(git_dir.parent().unwrap().join("git-check-ignore.baseline")).unwrap();
        let mut buf = Vec::new();
        let _group = MatchGroup::from_git_dir(git_dir, Some(dir.join("user.exclude")), &mut buf).unwrap();
        for (_value, _source_and_line) in (Expectations {
            lines: baseline.lines(),
        }) {}
    }

    #[test]
    fn from_overrides() {
        let input = ["simple", "pattern/"];
        let group = git_attributes::MatchGroup::<Ignore>::from_overrides(input);
        assert_eq!(
            group.pattern_matching_relative_path("Simple", false, git_glob::pattern::Case::Fold),
            Some(pattern_to_match(&git_glob::parse("simple").unwrap(), 0))
        );
        assert_eq!(
            group.pattern_matching_relative_path("pattern", true, git_glob::pattern::Case::Sensitive),
            Some(pattern_to_match(&git_glob::parse("pattern/").unwrap(), 1))
        );
        assert_eq!(group.patterns.len(), 1);
        assert_eq!(
            git_attributes::PatternList::<Ignore>::from_overrides(input),
            group.patterns.into_iter().next().unwrap()
        );
    }

    fn pattern_to_match(pattern: &git_glob::Pattern, sequence_number: usize) -> Match<'_, ()> {
        Match {
            pattern,
            value: &(),
            source: None,
            sequence_number,
        }
    }
}
