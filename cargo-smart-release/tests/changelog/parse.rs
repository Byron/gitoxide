use std::path::Path;

use cargo_smart_release::{
    changelog::{section::Segment, Section, Version},
    ChangeLog,
};

#[cfg(not(windows))]
fn fixup(v: String) -> String {
    v
}

#[cfg(windows)]
fn fixup(v: String) -> String {
    // Git checks out text files with line ending conversions, git itself will of course not put '\r\n' anywhere,
    // so that wouldn't be expected in an object and doesn't have to be parsed.
    v.replace("\r\n", "\n")
}

fn fixture(name: &str) -> std::io::Result<String> {
    let data = std::fs::read_to_string(git_testtools::fixture_path(
        Path::new("changelog").join("parse").join(name),
    ))?;
    Ok(fixup(data))
}

#[test]
fn all_unknown_in_section() {
    let fixture = fixture("known-section-unknown-content.md").unwrap();
    let log = ChangeLog::from_markdown(&fixture);
    assert_eq!(
        log.sections,
        vec![
            Section::Release {
                name: Version::Unreleased,
                removed_messages: vec![],
                date: None,
                heading_level: 3,
                version_prefix: "".into(),
                segments: vec![Segment::User {
                    markdown: "- hello ~~this is not understood~~\n* this isn't either\n\n".into()
                }],
                unknown: String::new(),
            },
            Section::Release {
                name: Version::Semantic("1.0.0".parse().unwrap()),
                removed_messages: vec![],
                date: None,
                heading_level: 4,
                version_prefix: Section::DEFAULT_PREFIX.into(),
                segments: vec![Segment::User {
                    markdown: "Some free text in a paragraph\nthat won't parse.\n".into()
                }],
                unknown: String::new(),
            }
        ]
    )
}

#[test]
fn unknown_link_and_headling() {
    let fixture = fixture("known-section-unknown-headline-with-link.md").unwrap();
    let log = ChangeLog::from_markdown(&fixture);
    assert_eq!(
        log.sections,
        vec![Section::Release {
            name: Version::Unreleased,
            removed_messages: vec![],
            date: None,
            heading_level: 4,
            version_prefix: "".into(),
            segments: vec![Segment::User {
                markdown: "##### Special\n\nHello [there][194] period.\n".into()
            }],
            unknown: String::new(),
        },]
    )
}

#[test]
fn known_and_unknown_sections_are_sorted() {
    let fixture = fixture("unknown-known-unknown-known-unsorted.md").unwrap();
    let log = ChangeLog::from_markdown(&fixture);
    assert_eq!(
        log.sections,
        vec![
            Section::Verbatim {
                text: "Hello, this is a changelog.\n\n".into(),
                generated: false
            },
            Section::Release {
                name: Version::Unreleased,
                removed_messages: vec![],
                date: None,
                heading_level: 3,
                version_prefix: "".into(),
                unknown: "".into(),
                segments: vec![Segment::User {
                    markdown: "TBD\n".into()
                }]
            },
            Section::Release {
                name: Version::Semantic(semver::Version::parse("1.0.0").unwrap()),
                removed_messages: vec![],
                date: None,
                heading_level: 3,
                version_prefix: Section::DEFAULT_PREFIX.into(),
                unknown: "".into(),
                segments: vec![
                    Segment::User {
                        markdown: "- initial release\n\n".into()
                    },
                    Segment::User {
                        markdown: "### Something in between\n\nintermezzo\n".into()
                    },
                ]
            },
        ],
    )
}
