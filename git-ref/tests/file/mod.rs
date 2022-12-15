use git_ref::file;

// TODO: when ready, add a new test entry point with a feature toggle to switch this to `git_ref::Store`.
//       That way all tests can run against the new general store to validate its truly working.
//       The same can be done when RefTable is available, and its corresponding tests.
pub type Store = file::Store;

fn store() -> crate::Result<Store> {
    store_at("make_ref_repository.sh")
}

pub fn store_with_packed_refs() -> crate::Result<Store> {
    store_at("make_packed_ref_repository.sh")
}

pub fn store_at(name: &str) -> crate::Result<Store> {
    let path = git_testtools::scripted_fixture_read_only(name)?;
    Ok(Store::at(
        path.join(".git"),
        git_ref::store::WriteReflog::Normal,
        git_hash::Kind::Sha1,
    ))
}

fn store_writable(name: &str) -> crate::Result<(git_testtools::tempfile::TempDir, Store)> {
    let dir = git_testtools::scripted_fixture_writable(name)?;
    let git_dir = dir.path().join(".git");
    Ok((
        dir,
        Store::at(git_dir, git_ref::store::WriteReflog::Normal, git_hash::Kind::Sha1),
    ))
}

mod log;
mod reference;
mod store;
pub(crate) mod transaction;
mod worktree;
