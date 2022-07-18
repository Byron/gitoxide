use git_config::source;

#[test]
fn new_globals() {
    let config = git_config::File::new_globals().unwrap();
    assert!(config.sections().all(|section| {
        let kind = section.meta().source.kind();
        kind != source::Kind::Repository && kind != source::Kind::Override
    }));
}
