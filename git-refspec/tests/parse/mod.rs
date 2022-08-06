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
                _ => {
                    eprintln!("{res:?} {err_code}");
                    mismatch += 1;
                }
            },
            Err(_) => {
                panics += 1;
            }
        }
    }
    if panics != 0 || mismatch != 0 {
        panic!(
            "Out of {} baseline entries, got {} right, ({} mismatches and {} panics)",
            count,
            count - (mismatch + panics),
            mismatch,
            panics
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

    mod fetch {
        use crate::parse::{assert_parse, b};
        use git_refspec::{Fetch, Instruction};

        #[test]
        fn empty_lhs_colon_rhs_fetches_head_to_destination() {
            assert_parse(
                ":a",
                Instruction::Fetch(Fetch::AndUpdateSingle {
                    src: b("HEAD"),
                    dst: b("a"),
                    allow_non_fast_forward: false,
                }),
            );

            assert_parse(
                "+:a",
                Instruction::Fetch(Fetch::AndUpdateSingle {
                    src: b("HEAD"),
                    dst: b("a"),
                    allow_non_fast_forward: true,
                }),
            );
        }

        #[test]
        fn colon_alone_is_for_fetching_into_fetchhead() {
            assert_parse(
                ":",
                Instruction::Fetch(Fetch::AllMatchingBranches {
                    allow_non_fast_forward: false,
                }),
            );
            assert_parse(
                "+:",
                Instruction::Fetch(Fetch::AllMatchingBranches {
                    allow_non_fast_forward: true,
                }),
            );
        }
    }

    mod push {
        use crate::parse::{assert_parse, b};
        use git_refspec::{Instruction, Mode, Push};

        #[test]
        fn colon_alone_is_for_pushing_matching_refs() {
            assert_parse(
                ":",
                Instruction::Push(Push::AllMatchingBranches {
                    allow_non_fast_forward: false,
                }),
            );
            assert_parse(
                "+:",
                Instruction::Push(Push::AllMatchingBranches {
                    allow_non_fast_forward: true,
                }),
            );
        }

        #[test]
        fn delete() {
            assert_parse(":a", Instruction::Push(Push::Delete { ref_or_pattern: b("a") }));
            let spec = assert_parse("+:a", Instruction::Push(Push::Delete { ref_or_pattern: b("a") }));
            assert_eq!(
                spec.mode(),
                Mode::Force,
                "force is set, even though it has no effect in the actual instruction"
            );
        }
    }
}

mod util {
    use git_refspec::{Instruction, Operation, RefSpecRef};

    pub fn b(input: &str) -> &bstr::BStr {
        input.into()
    }

    pub fn try_parse(spec: &str, op: Operation) -> Result<RefSpecRef<'_>, git_refspec::parse::Error> {
        git_refspec::parse(spec.into(), op)
    }

    pub fn assert_parse<'a>(spec: &'a str, expected: Instruction<'_>) -> RefSpecRef<'a> {
        let spec = try_parse(spec, expected.operation()).expect("no error");
        assert_eq!(spec.instruction(), expected);
        spec
    }
}
pub use util::*;
