use git_testtools::scripted_fixture_repo_read_only;

#[test]
#[ignore]
fn baseline() {
    let _dir = scripted_fixture_repo_read_only("make_baseline.sh").unwrap();
}

mod invalid {
    use git_refspec::{parse, parse::Error, Operation};

    #[test]
    #[ignore]
    fn empty() {
        assert!(matches!(parse("".into(), Operation::Fetch).unwrap_err(), Error::Empty));
        assert!(matches!(parse("".into(), Operation::Push).unwrap_err(), Error::Empty));
    }

    #[test]
    #[ignore]
    fn negative_with_destination() {
        assert!(matches!(
            parse("^a:b".into(), Operation::Fetch).unwrap_err(),
            Error::NegativeWithDestination
        ));
        assert!(matches!(
            parse("a:b".into(), Operation::Fetch).unwrap_err(),
            Error::NegativeWithDestination
        ));
    }

    mod fetch {}

    mod push {}
}
