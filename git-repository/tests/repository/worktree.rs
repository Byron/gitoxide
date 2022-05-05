use git_ref::bstr;
use git_repository as git;

struct Baseline<'a> {
    lines: bstr::Lines<'a>,
}

mod baseline {
    use super::Baseline;
    use git_object::bstr::BStr;
    use git_repository::bstr::{BString, ByteSlice};
    use std::path::{Path, PathBuf};

    impl<'a> Baseline<'a> {
        pub fn collect(dir: impl AsRef<Path>) -> std::io::Result<Vec<Worktree>> {
            let content = std::fs::read(dir.as_ref().join("worktree-list.baseline"))?;
            Ok(Baseline { lines: content.lines() }.collect())
        }
    }

    #[derive(Debug)]
    pub struct Worktree {
        root: PathBuf,
        bare: bool,
        locked: Option<BString>,
        peeled: git_hash::ObjectId,
        branch: Option<BString>,
    }

    impl<'a> Iterator for Baseline<'a> {
        type Item = Worktree;

        fn next(&mut self) -> Option<Self::Item> {
            let root = git_path::from_bstr(fields(self.lines.next()?).1).into_owned();
            let mut bare = false;
            let mut branch = None;
            let mut peeled = git_hash::ObjectId::null(git_hash::Kind::Sha1);
            let mut locked = None;
            while let Some(line) = self.lines.next() {
                if line.is_empty() {
                    break;
                }
                if line == b"bare" {
                    bare = true;
                    continue;
                } else if line == b"detached" {
                    continue;
                }
                let (field, value) = fields(line);
                match field {
                    f if f == "HEAD" => peeled = git_hash::ObjectId::from_hex(value).expect("valid hash"),
                    f if f == "branch" => branch = Some(value.to_owned()),
                    f if f == "locked" => locked = Some(value.to_owned()),
                    _ => unreachable!("unknown field: {}", field),
                }
            }
            Some(Worktree {
                root,
                bare,
                locked,
                peeled,
                branch,
            })
        }
    }

    fn fields(line: &[u8]) -> (&BStr, &BStr) {
        let (a, b) = line.split_at(line.find_byte(b' ').expect("at least a space"));
        (a.as_bstr(), b[1..].as_bstr())
    }
}

#[test]
fn from_bare_parent_repo() {
    let dir = git_testtools::scripted_fixture_repo_read_only_with_args("make_worktree_repo.sh", ["bare"]).unwrap();
    let repo = git::open(dir.join("repo.git")).unwrap();

    assert!(repo.is_bare());
    dbg!(Baseline::collect(dir));
}

#[test]
fn from_nonbare_parent_repo() {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_worktree_repo.sh").unwrap();
    let repo = git::open(dir.join("repo")).unwrap();

    assert!(!repo.is_bare());
    dbg!(Baseline::collect(dir));
}
