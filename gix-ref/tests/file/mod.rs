use gix_ref::file;

// TODO: when ready, add a new test entry point with a feature toggle to switch this to `gix_ref::Store`.
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
    let path = gix_testtools::scripted_fixture_read_only_standalone(name)?;
    Ok(Store::at(path.join(".git"), Default::default()))
}

pub fn store_at_with_args(name: &str, args: impl IntoIterator<Item = impl Into<String>>) -> crate::Result<Store> {
    let path = gix_testtools::scripted_fixture_read_only_with_args_standalone(name, args)?;
    Ok(Store::at(path.join(".git"), Default::default()))
}

fn store_writable(name: &str) -> crate::Result<(gix_testtools::tempfile::TempDir, Store)> {
    let dir = gix_testtools::scripted_fixture_writable_standalone(name)?;
    let git_dir = dir.path().join(".git");
    Ok((dir, Store::at(git_dir, Default::default())))
}

struct EmptyCommit;
impl gix_object::Find for EmptyCommit {
    fn try_find<'a>(
        &self,
        _id: &gix_hash::oid,
        _buffer: &'a mut Vec<u8>,
    ) -> Result<Option<gix_object::Data<'a>>, gix_object::find::Error> {
        Ok(Some(gix_object::Data {
            kind: gix_object::Kind::Commit,
            data: &[],
        }))
    }
}

mod log;
mod reference;
mod store;
pub(crate) mod transaction;
mod worktree;
