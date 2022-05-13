use std::path::PathBuf;

fn dir(packed: bool) -> PathBuf {
    let name = "make_worktree_repo.sh";
    if packed {
        git_testtools::scripted_fixture_repo_read_only_with_args(name, Some("packed"))
    } else {
        git_testtools::scripted_fixture_repo_read_only(name)
    }
    .unwrap()
}

fn main_store(packed: bool) -> git_ref::file::Store {
    let dir = dir(packed);
    git_ref::file::Store::at(dir.join("repo").join(".git"), Default::default(), Default::default())
}

fn worktree_store(packed: bool) -> git_ref::file::Store {
    let dir = dir(packed);
    let (git_dir, _work_tree) = git_discover::upwards(dir.join("w1"))
        .unwrap()
        .0
        .into_repository_and_work_tree_directories();
    let common_dir = git_dir.join("../..");
    git_ref::file::Store::for_linked_worktree(git_dir, common_dir, Default::default(), Default::default())
}

#[test]
fn with_common_dir() {
    for packed in [false, true] {
        let _store = worktree_store(packed);
    }
}

#[test]
fn with_git_dir() {
    for packed in [false, true] {
        let _store = main_store(packed);
    }
}
