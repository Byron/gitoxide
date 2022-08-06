use bstr::ByteSlice;
use git_refspec::Operation;
use git_testtools::scripted_fixture_repo_read_only;
use std::panic::catch_unwind;

#[test]
#[should_panic]
fn baseline() {
    let dir = scripted_fixture_repo_read_only("make_baseline.sh").unwrap();
    let baseline = std::fs::read(dir.join("baseline.git")).unwrap();
    let mut lines = baseline.lines();
    let mut panics = 0;
    let mut mismatch = 0;
    let mut count = 0;
    while let Some(kind_spec) = lines.next() {
        count += 1;
        let (kind, spec) = kind_spec.split_at(kind_spec.find_byte(b' ').expect("space between kind and spec"));
        let err_code: usize = lines
            .next()
            .expect("err code")
            .to_str()
            .unwrap()
            .parse()
            .expect("number");
        let op = match kind {
            b"fetch" => Operation::Fetch,
            b"push" => Operation::Push,
            _ => unreachable!("{} unexpected", kind.as_bstr()),
        };
        let res = catch_unwind(|| try_parse(spec.to_str().unwrap(), op));
        match res {
            Ok(res) => match (res.is_ok(), err_code == 0) {
                (true, true) | (false, false) => {}
                _ => mismatch += 1,
            },
            Err(_) => {
                panics += 1;
            }
        }
    }
    if panics != 0 || mismatch != 0 {
        panic!(
            "Out of {} baseline entries, got {} mismatches and {} panics",
            count, mismatch, panics
        );
    }
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

    mod push {
        use crate::parse::assert_parse;
        use git_refspec::{Mode, Operation};

        #[test]
        #[ignore]
        fn colon_alone_is_for_pushing_matching_refs() {
            assert_parse(":", Operation::Push, None, None, Mode::Normal);
        }
    }
}

mod util {
    use git_refspec::{Mode, Operation, RefSpecRef};

    pub fn try_parse(spec: &str, op: Operation) -> Result<RefSpecRef<'_>, git_refspec::parse::Error> {
        git_refspec::parse(spec.into(), op)
    }

    pub fn assert_parse(spec: &str, op: Operation, _src: Option<&str>, _dest: Option<&str>, mode: Mode) {
        let spec = try_parse(spec, op).expect("no error");
        assert_eq!(spec.mode(), mode);
    }
}
pub use util::*;
