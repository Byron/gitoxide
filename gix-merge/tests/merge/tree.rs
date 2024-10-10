#[test]
fn run_baseline() -> crate::Result {
    let root = gix_testtools::scripted_fixture_read_only("tree-baseline.sh")?;
    let cases = std::fs::read_to_string(root.join("baseline.cases"))?;
    for case in baseline::Expectations::new(&root, &cases) {}

    Ok(())
}

mod baseline {
    use std::path::Path;

    pub struct Conflict;

    pub struct Expectation {
        pub odb: gix_odb::Handle,
        pub our_commit_id: gix_hash::ObjectId,
        pub their_commit_id: gix_hash::ObjectId,
        pub merge_info: Result<gix_hash::ObjectId, Conflict>,
    }

    pub struct Expectations<'a> {
        root: &'a Path,
        lines: std::str::Lines<'a>,
    }

    impl<'a> Expectations<'a> {
        pub fn new(root: &'a Path, cases: &'a str) -> Self {
            Expectations {
                root,
                lines: cases.lines(),
            }
        }
    }

    impl Iterator for Expectations<'_> {
        type Item = Expectation;

        fn next(&mut self) -> Option<Self::Item> {
            let line = self.lines.next()?;
            let mut tokens = line.split(' ');
            let (Some(subdir), Some(our_commit_id), Some(their_commit_id), Some(merge_info_filename)) =
                (tokens.next(), tokens.next(), tokens.next(), tokens.next())
            else {
                unreachable!("invalid line: {line:?}")
            };
            assert_eq!(tokens.next(), None, "unexpected trailing tokens in line {line:?}");

            let subdir = self.root.join(subdir);
            let objects = gix_odb::at(subdir.join(".git/objects")).expect("object dir exists");
            let our_commit_id = gix_hash::ObjectId::from_hex(our_commit_id.as_bytes()).unwrap();
            let their_commit_id = gix_hash::ObjectId::from_hex(their_commit_id.as_bytes()).unwrap();
            let merge_info = parse_merge_info(std::fs::read_to_string(subdir.join(merge_info_filename)).unwrap());
            Some(Expectation {
                odb: objects,
                our_commit_id,
                their_commit_id,
                merge_info,
            })
        }
    }

    fn parse_merge_info(content: String) -> Result<gix_hash::ObjectId, Conflict> {
        let mut lines = content.split('\0').filter(|t| !t.is_empty());
        let tree_id = gix_hash::ObjectId::from_hex(lines.next().unwrap().as_bytes()).unwrap();
        assert_eq!(lines.next(), None, "TODO: implement multi-line answer");
        Ok(tree_id)
    }
}
