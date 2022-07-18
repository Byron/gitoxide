use crate::named_repo;
use std::path::Path;

#[test]
fn access_values() {
    let repo = named_repo("make_config_repo.sh").unwrap();
    let config = repo.config_snapshot();

    assert_eq!(config.boolean("core.bare"), Some(false));
    assert_eq!(config.boolean("a.bad-bool"), None);
    assert_eq!(config.try_boolean("core.bare"), Some(Ok(false)));
    assert!(matches!(config.try_boolean("a.bad-bool"), Some(Err(_))));

    assert_eq!(config.boolean("core.missing"), None);
    assert_eq!(config.try_boolean("core.missing"), None);

    assert_eq!(
        config
            .trusted_path("a.relative-path")
            .expect("exists")
            .expect("no error"),
        Path::new("./something")
    );
    assert_eq!(
        config
            .trusted_path("a.absolute-path")
            .expect("exists")
            .expect("no error"),
        Path::new("/etc/man.conf")
    );
    assert!(config.trusted_path("a.bad-user-path").expect("exists").is_err());
}
