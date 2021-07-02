use git_ref::file;

fn store() -> crate::Result<file::Store> {
    let path = git_testtools::scripted_fixture_repo_read_only("make_ref_repository.sh")?;
    Ok(file::Store::from(path.join(".git")))
}

// TODO: actually use this in transaction testing
fn _store_writable() -> crate::Result<(git_testtools::tempfile::TempDir, file::Store)> {
    let dir = git_testtools::scripted_fixture_repo_writable("make_ref_repository.sh")?;
    let git_dir = dir.path().join(".git");
    Ok((dir, file::Store::from(git_dir)))
}

mod log;
mod reference;
mod store;
mod transaction;
