use git_ref::bstr;
use git_repository as git;

mod with_core_worktree_config {
    use std::io::BufRead;

    use git_repository as git;

    use crate::repository::worktree::Baseline;

    #[test]
    fn relative() -> crate::Result {
        for (name, is_relative) in [("absolute-worktree", false), ("relative-worktree", true)] {
            let repo = repo(name);

            if is_relative {
                assert_eq!(
                    repo.work_dir().unwrap(),
                    repo.git_dir().parent().unwrap().parent().unwrap().join("worktree"),
                    "work_dir is set to core.worktree config value, relative paths are appended to `git_dir() and made absolute`"
                );
            } else {
                assert_eq!(
                    repo.work_dir().unwrap(),
                    git_path::realpath(repo.git_dir().parent().unwrap().parent().unwrap().join("worktree"))?,
                    "absolute workdirs are left untouched"
                );
            }

            assert_eq!(
                repo.worktree().expect("present").base(),
                repo.work_dir().unwrap(),
                "current worktree is based on work-tree dir"
            );

            let baseline = Baseline::collect(repo.git_dir())?;
            assert_eq!(baseline.len(), 1, "git lists the main worktree");
            assert_eq!(
                baseline[0].root,
                git_path::realpath(repo.git_dir().parent().unwrap())?,
                "git lists the original worktree, to which we have no access anymore"
            );
            assert_eq!(
                repo.worktrees()?.len(),
                0,
                "we only list linked worktrees, and there are none"
            );
            assert_eq!(
                repo.index()?.entries().len(),
                count_deleted(repo.git_dir()),
                "git considers all worktree entries missing as the overridden worktree is an empty dir"
            );
            assert_eq!(repo.index()?.entries().len(), 3, "just to be sure");
        }
        Ok(())
    }

    #[test]
    fn non_existing_relative() {
        let repo = repo("relative-nonexisting-worktree");
        assert_eq!(
            count_deleted(repo.git_dir()),
            0,
            "git can't chdir into missing worktrees, has no error handling there"
        );

        assert_eq!(
            repo.work_dir(),
            repo.git_dir().parent(),
            "we just ignore missing configured worktree dirs and fall back to the default one"
        )
    }

    #[test]
    fn relative_file() {
        let repo = repo("relative-worktree-file");
        assert_eq!(count_deleted(repo.git_dir()), 0, "git can't chdir into a file");

        assert_eq!(
            repo.work_dir(),
            repo.git_dir().parent(),
            "we just ignore missing configured worktree dirs and fall back to the default one"
        )
    }

    #[test]
    fn bare_relative() -> crate::Result {
        let repo = repo("bare-relative-worktree");

        assert_eq!(
            count_deleted(repo.git_dir()),
            0,
            "git refuses to mix bare with core.worktree"
        );
        assert!(
            repo.work_dir().is_none(),
            "we simply don't load core.worktree in bare repos either to match this behaviour"
        );
        Ok(())
    }

    fn repo(name: &str) -> git::Repository {
        let dir = git_testtools::scripted_fixture_read_only("make_core_worktree_repo.sh").unwrap();
        git::open_opts(dir.join(name), crate::restricted()).unwrap()
    }

    fn count_deleted(git_dir: &std::path::Path) -> usize {
        std::fs::read(git_dir.join("status.baseline"))
            .unwrap()
            .lines()
            .filter_map(Result::ok)
            .filter(|line| line.contains(" D "))
            .count()
    }
}

struct Baseline<'a> {
    lines: bstr::Lines<'a>,
}

mod baseline {
    use std::path::{Path, PathBuf};

    use git_object::bstr::BStr;
    use git_repository::bstr::{BString, ByteSlice};

    use super::Baseline;

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
fn from_bare_parent_repo() {
    if git_testtools::should_skip_as_git_version_is_smaller_than(2, 31, 0) {
        return;
    }
    let dir = git_testtools::scripted_fixture_read_only_with_args("make_worktree_repo.sh", ["bare"]).unwrap();
    let repo = git::open(dir.join("repo.git")).unwrap();

    run_assertions(repo, true /* bare */);
}

#[test]
fn from_nonbare_parent_repo() {
    if git_testtools::should_skip_as_git_version_is_smaller_than(2, 31, 0) {
        return;
    }
    let dir = git_testtools::scripted_fixture_read_only("make_worktree_repo.sh").unwrap();
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

    if should_be_bare {
        assert!(main_repo.worktree().is_none());
    } else {
        assert_eq!(
            main_repo.work_dir().expect("non-bare").canonicalize().unwrap(),
            expected_main.root.canonicalize().unwrap()
        );
        assert_eq!(main_repo.head_id().unwrap(), expected_main.peeled);
        assert_eq!(
            main_repo.head_name().unwrap().expect("no detached head").as_bstr(),
            expected_main.branch.unwrap()
        );
        let worktree = main_repo.worktree().expect("not bare");
        assert!(
            worktree.lock_reason().is_none(),
            "main worktrees, bare or not, are never locked"
        );
        assert!(!worktree.is_locked());
        assert!(worktree.is_main());
    }
    assert_eq!(main_repo.main_repo().unwrap(), main_repo, "main repo stays main repo");

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
        let proxy_lock_reason = actual.lock_reason();
        assert_eq!(proxy_lock_reason, expected.locked);
        let proxy_is_locked = actual.is_locked();
        assert_eq!(proxy_is_locked, proxy_lock_reason.is_some());
        // TODO: check id of expected worktree, but need access to .gitdir from worktree base
        let proxy_id = actual.id().to_owned();
        assert_eq!(
            base.is_dir(),
            expected.prunable.is_none(),
            "in our case prunable repos have no worktree base"
        );

        let repo = if base.is_dir() {
            let repo = actual.into_repo().unwrap();
            assert_eq!(
                &git::open(base).unwrap(),
                &repo,
                "repos are considered the same no matter if opened from worktree or from git dir"
            );
            repo
        } else {
            assert!(
                matches!(
                    actual.clone().into_repo(),
                    Err(git::worktree::proxy::into_repo::Error::MissingWorktree { .. })
                ),
                "missing bases are detected"
            );
            actual.into_repo_with_possibly_inaccessible_worktree().unwrap()
        };
        let worktree = repo.worktree().expect("linked worktrees have at least a base path");
        assert!(!worktree.is_main());
        assert_eq!(worktree.lock_reason(), proxy_lock_reason);
        assert_eq!(worktree.is_locked(), proxy_is_locked);
        assert_eq!(worktree.id(), Some(proxy_id.as_ref()));
        assert_eq!(
            repo.main_repo().unwrap(),
            main_repo,
            "main repo from worktree repo is the actual main repo"
        );
    }
}
