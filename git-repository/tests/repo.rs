use git_repository::{Easy, Repository};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn repo(name: &str) -> crate::Result<Repository> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(Repository::discover(repo_path)?)
}

fn basic_rw_repo() -> crate::Result<(Easy, tempfile::TempDir)> {
    let repo_path = git_testtools::scripted_fixture_repo_writable("make_basic_repo.sh")?;
    Ok((Repository::open(repo_path.path())?.into(), repo_path))
}

mod commit;
mod discover;
mod easy;
mod init;
mod reference;
