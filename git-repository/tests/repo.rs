use git_repository::Repository;
use git_repository::ThreadSafeRepository;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn repo(name: &str) -> crate::Result<ThreadSafeRepository> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(ThreadSafeRepository::open(repo_path)?)
}

fn repo_rw(name: &str) -> crate::Result<(git_repository::Repository, tempfile::TempDir)> {
    let repo_path = git_testtools::scripted_fixture_repo_writable(name)?;
    Ok((
        ThreadSafeRepository::discover(repo_path.path())?.to_thread_local(),
        repo_path,
    ))
}

fn basic_repo() -> crate::Result<Repository> {
    repo("make_basic_repo.sh").map(|r| r.to_thread_local())
}

fn basic_rw_repo() -> crate::Result<(Repository, tempfile::TempDir)> {
    repo_rw("make_basic_repo.sh")
}

mod discover;
mod easy;
mod init;
mod reference;
