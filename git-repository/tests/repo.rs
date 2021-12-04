use git_repository::easy::Handle;
use git_repository::Repository;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn repo(name: &str) -> crate::Result<Repository> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(Repository::open(repo_path)?)
}

fn repo_rw(name: &str) -> crate::Result<(Repository, tempfile::TempDir)> {
    let repo_path = git_testtools::scripted_fixture_repo_writable(name)?;
    Ok((Repository::discover(repo_path.path())?, repo_path))
}

fn easy_repo_rw(name: &str) -> crate::Result<(Handle, tempfile::TempDir)> {
    repo_rw(name).map(|(repo, dir)| (repo.into(), dir))
}

fn basic_repo() -> crate::Result<Handle> {
    repo("make_basic_repo.sh").map(|r| r.to_easy())
}

fn basic_rw_repo() -> crate::Result<(Handle, tempfile::TempDir)> {
    easy_repo_rw("make_basic_repo.sh")
}

mod discover;
mod easy;
mod init;
mod reference;
