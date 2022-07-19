use git_config::source;
use serial_test::serial;

#[test]
fn new_globals() {
    let config = git_config::File::new_globals().unwrap();
    assert!(config.sections().all(|section| {
        let kind = section.meta().source.kind();
        kind != source::Kind::Repository && kind != source::Kind::Override
    }));
}

#[test]
#[serial]
fn new_environment_overrides() {
    let config = git_config::File::new_environment_overrides().unwrap();
    assert!(config.is_void());
}
