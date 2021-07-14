use git_ref::file;

fn store() -> crate::Result<file::Store> {
    store_at("make_ref_repository.sh")
}

pub fn store_with_packed_refs() -> crate::Result<file::Store> {
    store_at("make_packed_ref_repository.sh")
}

fn store_at(name: &str) -> crate::Result<file::Store> {
    let path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(file::Store::from(path.join(".git")))
}

fn store_writable(name: &str) -> crate::Result<(git_testtools::tempfile::TempDir, file::Store)> {
    let dir = git_testtools::scripted_fixture_repo_writable(name)?;
    let git_dir = dir.path().join(".git");
    Ok((dir, file::Store::from(git_dir)))
}

mod log;
mod reference;
mod store;
mod transaction;
