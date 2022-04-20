mod ignore {
    use git_attributes::Ignore;

    #[test]
    fn init_from_overrides() {
        let input = ["simple", "pattern/"];
        let patterns = git_attributes::MatchGroup::<Ignore>::from_overrides(input).patterns;
        assert_eq!(patterns.len(), 1);
        assert_eq!(
            git_attributes::PatternList::<Ignore>::from_overrides(input),
            patterns.into_iter().nth(0).unwrap()
        );
    }
}
