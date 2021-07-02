use git_ref::file;

fn store() -> crate::Result<file::Store> {
    let path = git_testtools::scripted_fixture_repo_read_only("make_ref_repository.sh")?;
    Ok(file::Store::from(path.join(".git")))
}

fn store_writable() -> crate::Result<(git_testtools::tempfile::TempDir, file::Store)> {
    let dir = git_testtools::scripted_fixture_repo_writable("make_ref_repository.sh")?;
    Ok((dir, file::Store::from(dir.path().join(".git"))))
}

mod log;
mod reference;
mod store;
mod transaction;
