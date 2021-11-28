pub fn store_at(name: &str) -> crate::Result<git_ref::Store> {
    let path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(git_ref::Store::at(
        path.join(".git"),
        git_ref::store::WriteReflog::Normal,
    )?)
}

#[test]
#[cfg(feature = "internal-testing-git-features-parallel")]
fn is_send_and_sync() {
    pub fn store_with_packed_refs() -> crate::Result<git_ref::Store> {
        store_at("make_packed_ref_repository.sh")
    }
    fn assert_type<T: Send + Sync>(_t: T) {}
    let store = store_with_packed_refs().unwrap();
    assert_type(&store);
    assert_type(store);
}
