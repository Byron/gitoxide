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

    pub type Reason = BString;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Worktree {
        pub root: PathBuf,
        pub bare: bool,
        pub locked: Option<Reason>,
        pub peeled: git_hash::ObjectId,
        pub branch: Option<BString>,
        pub prunable: Option<Reason>,
    }

    impl<'a> Iterator for Baseline<'a> {
        type Item = Worktree;

        fn next(&mut self) -> Option<Self::Item> {
            let root = git_path::from_bstr(fields(self.lines.next()?).1).into_owned();
            let mut bare = false;
            let mut branch = None;
            let mut peeled = git_hash::ObjectId::null(git_hash::Kind::Sha1);
            let mut locked = None;
            let mut prunable = None;
            for line in self.lines.by_ref() {
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
                    f if f == "prunable" => prunable = Some(value.to_owned()),
                    _ => unreachable!("unknown field: {}", field),
                }
            }
            Some(Worktree {
                root,
                bare,
                locked,
                peeled,
                branch,
                prunable,
            })
        }
    }

    fn fields(line: &[u8]) -> (&BStr, &BStr) {
        let (a, b) = line.split_at(line.find_byte(b' ').expect("at least a space"));
        (a.as_bstr(), b[1..].as_bstr())
    }
}

#[test]
#[ignore]
fn from_bare_parent_repo() {
    let dir = git_testtools::scripted_fixture_repo_read_only_with_args("make_worktree_repo.sh", ["bare"]).unwrap();
    let repo = git::open(dir.join("repo.git")).unwrap();

    run_assertions(repo, true /* bare */);
}

#[test]
#[ignore]
fn from_nonbare_parent_repo() {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_worktree_repo.sh").unwrap();
    let repo = git::open(dir.join("repo")).unwrap();

    run_assertions(repo, false /* bare */);
}

fn run_assertions(main_repo: git::Repository, should_be_bare: bool) {
    assert_eq!(main_repo.is_bare(), should_be_bare);
    let mut baseline = Baseline::collect(
        main_repo
            .work_dir()
            .map(|p| p.parent())
            .unwrap_or_else(|| main_repo.git_dir().parent())
            .expect("a temp dir as parent"),
    )
    .unwrap();
    let expected_main = baseline.remove(0);
    assert_eq!(expected_main.bare, should_be_bare);

    if !should_be_bare {
        assert_eq!(
            main_repo.work_dir().expect("non-bare").canonicalize().unwrap(),
            expected_main.root.canonicalize().unwrap()
        );
        assert_eq!(main_repo.head_id().unwrap(), expected_main.peeled);
        assert_eq!(
            main_repo.head_name().unwrap().expect("no detached head").as_bstr(),
            expected_main.branch.unwrap()
        )
    }

    let actual = main_repo.worktrees().unwrap();
    assert_eq!(actual.len(), baseline.len());
    for actual in actual {
        let base = actual.base().unwrap();
        let expected = baseline
            .iter()
            .find(|exp| exp.root == base)
            .expect("we get the same root and it matches");
        assert!(
            !expected.bare,
            "only the main worktree can be bare, and we don't see it in this loop"
        );
        assert_eq!(actual.lock_reason(), expected.locked);
        assert_eq!(actual.is_locked(), actual.lock_reason().is_some());

        dbg!(expected);
    }
}
