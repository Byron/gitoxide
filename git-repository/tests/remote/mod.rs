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

pub(crate) fn into_daemon_remote<'repo>(
    remote: git::Remote<'repo>,
    daemon_url: &str,
    repo_path: &str,
) -> crate::Result<git::Remote<'repo>> {
    let mut new_remote = remote.repo().remote_at(format!("{}/{}", daemon_url, repo_path))?;
    for direction in [git::remote::Direction::Fetch, git::remote::Direction::Push] {
        new_remote.replace_refspecs(
            remote.refspecs(direction).iter().map(|s| s.to_ref().to_bstring()),
            direction,
        )?;
    }
    Ok(new_remote)
}

pub(crate) fn cow_str(s: &str) -> Cow<str> {
    Cow::Borrowed(s)
}

mod connect;
mod fetch;
mod ref_map;
mod save;
mod name {
    use git_repository as git;
    #[test]
    fn empty_is_invalid() {
        assert!(git::remote::name::validated("").is_err());
    }
}
