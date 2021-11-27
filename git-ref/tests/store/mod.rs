use git_features::threading::OwnShared;
use std::convert::TryFrom;

pub fn store_at(name: &str) -> crate::Result<OwnShared<git_ref::Store>> {
    let path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(OwnShared::new(git_ref::Store::try_from(path.join(".git"))?))
}

#[test]
fn is_send_and_sync() {
    fn assert_type<T: Send + Sync>(_t: T) {}
    let store = store_at("make_packed_ref_repository.sh").unwrap();
    let store = OwnShared::try_unwrap(store).ok().expect("single ownership");
    assert_type(&store);
    assert_type(store);
}

mod handle {}
