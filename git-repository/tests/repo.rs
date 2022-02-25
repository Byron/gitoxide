use git_repository::{easy::Repository, SyncRepository};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn repo(name: &str) -> crate::Result<SyncRepository> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(SyncRepository::open(repo_path)?)
}

fn repo_rw(name: &str) -> crate::Result<(SyncRepository, tempfile::TempDir)> {
    let repo_path = git_testtools::scripted_fixture_repo_writable(name)?;
    Ok((SyncRepository::discover(repo_path.path())?, repo_path))
}

fn easy_repo_rw(name: &str) -> crate::Result<(Repository, tempfile::TempDir)> {
    repo_rw(name).map(|(repo, dir)| (repo.into(), dir))
}

fn basic_repo() -> crate::Result<Repository> {
    repo("make_basic_repo.sh").map(|r| r.to_thread_local())
}

fn basic_rw_repo() -> crate::Result<(Repository, tempfile::TempDir)> {
    easy_repo_rw("make_basic_repo.sh")
}

mod discover;
mod easy;
mod init;
mod reference;
