use gix_config::parse;

#[test]
fn missing_dot_is_invalid() {
    assert_eq!(parse::key("hello".into()), None);
}

#[test]
fn section_name_and_key() {
    assert_eq!(
        parse::key("core.bare".into()),
        Some(parse::Key {
            section_name: "core",
            subsection_name: None,
            value_name: "bare"
        })
    );
}

#[test]
fn section_name_subsection_and_key() {
    assert_eq!(
        parse::key("remote.origin.url".into()),
        Some(parse::Key {
            section_name: "remote",
            subsection_name: Some("origin".into()),
            value_name: "url"
        })
    );

    assert_eq!(
        parse::key("includeIf.gitdir/i:C:\\bare.git.path".into()),
        Some(parse::Key {
            section_name: "includeIf",
            subsection_name: Some("gitdir/i:C:\\bare.git".into()),
            value_name: "path"
        })
    );
}
