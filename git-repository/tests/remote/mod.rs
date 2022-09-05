use std::{borrow::Cow, path::PathBuf};

use git_repository as git;
use git_testtools::scripted_fixture_repo_read_only;

pub(crate) fn repo_path(name: &str) -> PathBuf {
    let dir = scripted_fixture_repo_read_only("make_remote_repos.sh").unwrap();
    dir.join(name)
}

pub(crate) fn repo(name: &str) -> git::Repository {
    git::open_opts(repo_path(name), git::open::Options::isolated()).unwrap()
}

pub(crate) fn cow_str(s: &str) -> Cow<str> {
    Cow::Borrowed(s)
}

mod connect;
mod list_refs;
