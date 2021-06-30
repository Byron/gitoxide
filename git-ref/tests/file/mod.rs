use git_ref::file;

fn store() -> crate::Result<file::Store> {
    let path = git_testtools::scripted_fixture_repo_read_only("make_ref_repository.sh")?;
    Ok(file::Store::from(path.join(".git")))
}

mod log;
mod reference;
mod store;
