use git_testtools::scripted_fixture_repo_read_only;

#[test]
#[ignore]
fn baseline() {
    let _dir = scripted_fixture_repo_read_only("make_baseline.sh").unwrap();
}

mod invalid {
    use crate::parse::try_parse;
    use git_refspec::{parse::Error, Operation};

    #[test]
    fn empty() {
        assert!(matches!(try_parse("", Operation::Fetch).unwrap_err(), Error::Empty));
        assert!(matches!(try_parse("", Operation::Push).unwrap_err(), Error::Empty));
    }

    #[test]
    fn negative_with_destination() {
        for op in [Operation::Fetch, Operation::Push] {
            for spec in ["^a:b", "^a:", "^:", "^:b"] {
                assert!(matches!(
                    try_parse(spec, op).unwrap_err(),
                    Error::NegativeWithDestination
                ));
            }
        }
    }

    mod fetch {}

    mod push {}
}

mod util {
    use git_refspec::{Operation, RefSpecRef};

    pub fn try_parse(spec: &str, op: Operation) -> Result<RefSpecRef<'_>, git_refspec::parse::Error> {
        git_refspec::parse(spec.into(), op)
    }
}
pub use util::*;
