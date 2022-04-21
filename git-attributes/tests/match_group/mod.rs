mod ignore {
    use git_attributes::{Ignore, Match};

    #[test]
    fn init_from_overrides() {
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
