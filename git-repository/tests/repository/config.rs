use crate::basic_rw_repo;

#[test]
fn access_values() {
    let (repo, _dir) = basic_rw_repo().unwrap();
    let config = repo.config_snapshot();

    assert_eq!(config.boolean("core.bare"), Some(false));
    assert_eq!(config.boolean("core.missing"), None);
}
