use cargo_smart_release::changelog::{Section, Version};
use cargo_smart_release::ChangeLog;

#[test]
fn all_unknown_in_section() {
    let log = ChangeLog::from_markdown(include_str!("fixtures/known-section-unknown-content.md"));
    assert_eq!(
        log.sections,
        vec![
            Section::Release {
                name: Version::Unreleased,
                date: None,
                heading_level: 3,
                thanks_clippy_count: 0,
                unknown: "hello ~~this is not understood~~this isn't either".into()
            },
            Section::Release {
                name: Version::Semantic("1.0.0".parse().unwrap()),
                date: None,
                heading_level: 4,
                thanks_clippy_count: 0,
                unknown: "Some free text in a paragraphthat won't parse.".into()
            }
        ]
    )
}
