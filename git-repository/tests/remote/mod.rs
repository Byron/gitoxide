use std::{borrow::Cow, path::PathBuf};
use tempfile::TempDir;

use git_repository as git;
use git_testtools::{scripted_fixture_repo_read_only, scripted_fixture_repo_writable_with_args};

pub(crate) fn repo_path(name: &str) -> PathBuf {
    let dir = scripted_fixture_repo_read_only("make_remote_repos.sh").unwrap();
    dir.join(name)
}

pub(crate) fn repo(name: &str) -> git::Repository {
    git::open_opts(repo_path(name), git::open::Options::isolated()).unwrap()
}

pub(crate) fn repo_rw(name: &str) -> (git::Repository, TempDir) {
    let dir = scripted_fixture_repo_writable_with_args(
        "make_remote_repos.sh",
        &[] as &[String],
        git_testtools::Creation::ExecuteScript,
    )
    .unwrap();
    let repo = git::open_opts(dir.path().join(name), git::open::Options::isolated()).unwrap();
    (repo, dir)
}

pub(crate) fn cow_str(s: &str) -> Cow<str> {
    Cow::Borrowed(s)
}

mod connect;
mod fetch;
mod ref_map;
