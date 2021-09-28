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
                date: None,
                heading_level: 3,
                segments: vec![Segment::User {
                    text: "- hello ~~this is not understood~~\n* this isn't either\n\n".into()
                }],
                unknown: String::new(),
            },
            Section::Release {
                name: Version::Semantic("1.0.0".parse().unwrap()),
                date: None,
                heading_level: 4,
                segments: vec![Segment::User {
                    text: "Some free text in a paragraph\nthat won't parse.\n".into()
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
            date: None,
            heading_level: 4,
            segments: vec![Segment::User {
                text: "##### Special\n\nHello [there][194] period.\n".into()
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
                date: None,
                heading_level: 3,
                unknown: "".into(),
                segments: vec![Segment::User { text: "TBD\n".into() }]
            },
            Section::Release {
                name: Version::Semantic(semver::Version::parse("1.0.0").unwrap()),
                date: None,
                heading_level: 3,
                unknown: "".into(),
                segments: vec![
                    Segment::User {
                        text: "- initial release\n\n".into()
                    },
                    Segment::User {
                        text: "### Something inbetween\n\nintermezzo\n".into()
                    },
                ]
            },
        ],
    )
}
