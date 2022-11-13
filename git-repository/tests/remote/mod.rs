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

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
#[cfg_attr(not(feature = "async-network-client-async-std"), allow(unused_variables))]
pub(crate) fn spawn_daemon_if_async(name: &str) -> std::io::Result<Option<git_testtools::GitDaemon>> {
    #[cfg(not(feature = "async-network-client-async-std"))]
    {
        Ok(None)
    }
    #[cfg(feature = "async-network-client-async-std")]
    {
        git_testtools::spawn_git_daemon(repo_path(name)).map(Some)
    }
}

/// Turn `remote` into a remote that interacts with the git daemon at `daemon_url`, and `repo_path`
/// can be a specific repo, or it can be empty if the repo is hosted at the root.
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
#[cfg_attr(not(feature = "async-network-client-async-std"), allow(unused_variables))]
#[allow(dead_code)] // TODO: remove this when it's used
pub(crate) fn into_daemon_remote_if_async<'repo>(
    remote: git::Remote<'repo>,
    daemon_url: &str,
    repo_path: &str,
) -> crate::Result<git::Remote<'repo>> {
    #[cfg(not(feature = "async-network-client-async-std"))]
    {
        Ok(remote)
    }
    #[cfg(feature = "async-network-client-async-std")]
    {
        let mut new_remote = remote.repo().remote_at(format!("{}/{}", daemon_url, repo_path))?;
        for direction in [git::remote::Direction::Fetch, git::remote::Direction::Push] {
            new_remote.replace_refspecs(
                remote.refspecs(direction).iter().map(|s| s.to_ref().to_bstring()),
                direction,
            )?;
        }
        Ok(new_remote)
    }
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
