use git_repository as git;
use git_testtools::scripted_fixture_repo_read_only;

pub(crate) fn repo(name: &str) -> git::Repository {
    let dir = scripted_fixture_repo_read_only("make_remote_repos.sh").unwrap();
    git::open_opts(dir.join(name), git::open::Options::isolated()).unwrap()
}
