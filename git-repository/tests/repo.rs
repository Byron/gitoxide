use git_repository::Repository;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn repo(name: &str) -> crate::Result<Repository> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(Repository::discover(repo_path)?)
}

mod access;
mod discover;
mod object;
mod reference;
