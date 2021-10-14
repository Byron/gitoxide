use std::{collections::BTreeMap, convert::TryFrom};

use cargo_smart_release::{
    changelog,
    changelog::{section, section::segment::conventional, Section},
    ChangeLog,
};
use git_testtools::hex_to_id;
use nom::AsBytes;

use crate::Result;

#[test]
fn conventional_write_empty_messages() -> Result {
    let first_message = hex_to_id("0000000000000000000000000000000000000001");
    let second_message = hex_to_id("0000000000000000000000000000000000000002");

    let log = ChangeLog {
        sections: vec![Section::Release {
            heading_level: 4,
            date: Some(time::OffsetDateTime::from_unix_timestamp(0)?),
            name: changelog::Version::Semantic("1.0.2-beta.2".parse()?),
            removed_messages: vec![second_message],
            segments: vec![section::Segment::Conventional(section::segment::Conventional {
                kind: "feat",
                is_breaking: true,
                removed: vec![first_message],
                messages: vec![
                    conventional::Message::User {
                        markdown: " - verbatim `whatever` the _user_ writes [hello](world)".into(),
                    },
                    conventional::Message::Generated {
                        id: hex_to_id("0000000000000000000000000000000000000003"),
                        title: "this messages comes straight from git conventional and _may_ contain markdown".into(),
                        body: Some("first line\nsecond line\n\nanother paragraph".into()),
                    },
                    conventional::Message::User {
                        markdown:
                            " - just another user message, this time\n   with multiple lines\n\n   and a new paragraph"
                                .into(),
                    },
                ],
            })],
            unknown: String::new(),
        }],
    };

    for link_mode in &[
        changelog::write::Linkables::AsText,
        changelog::write::Linkables::AsLinks {
            repository_url: git_repository::Url::try_from(b"https://github.com/user/repo".as_bytes())?.into(),
        },
    ] {
        for _round in 1..=2 {
            let mut md = String::new();
            log.write_to(&mut md, link_mode, changelog::write::Components::all())?;
            insta::assert_snapshot!(md);

            let parsed_log = ChangeLog::from_markdown(&md);
            assert_eq!(parsed_log, log, "we can parse this back losslessly");
        }
    }
    for components in &[
        changelog::write::Components::empty(),
        changelog::write::Components::all(),
    ] {
        for section in &log.sections {
            let mut buf = String::new();
            section.write_to(&mut buf, &changelog::write::Linkables::AsText, *components)?;
            insta::assert_snapshot!(buf);
        }
    }
    Ok(())
}

#[test]
fn all_section_types_round_trips_lossy() -> Result {
    let log = ChangeLog {
        sections: vec![
            Section::Verbatim {
                text: "# Changelog\n\nmy very own header\n\n".into(),
                generated: false,
            },
            Section::Release {
                heading_level: 2,
                removed_messages: vec![],
                date: None,
                name: changelog::Version::Unreleased,
                segments: Vec::new(),
                unknown: "hello\nworld\n".into(),
            },
            Section::Release {
                heading_level: 4,
                removed_messages: vec![],
                date: Some(time::OffsetDateTime::from_unix_timestamp(0)?),
                name: changelog::Version::Semantic("1.0.2-beta.2".parse()?),
                segments: vec![
                    section::Segment::User {
                        markdown: "* hello world\n\tthis\n".into(),
                    },
                    section::Segment::Clippy(section::Data::Generated(section::segment::ThanksClippy { count: 42 })),
                    section::Segment::Statistics(section::Data::Generated(section::segment::CommitStatistics {
                        count: 100,
                        duration: time::Duration::days(32).into(),
                        conventional_count: 20,
                        unique_issues: vec![
                            section::segment::details::Category::Issue("1".into()),
                            section::segment::details::Category::Uncategorized,
                            section::segment::details::Category::Issue("42".into()),
                        ],
                    })),
                    section::Segment::Details(section::Data::Generated(section::segment::Details {
                        commits_by_category: {
                            let mut h = BTreeMap::default();
                            h.insert(
                                section::segment::details::Category::Uncategorized,
                                vec![
                                    section::segment::details::Message {
                                        title: "Just the title".into(),
                                        id: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                                    },
                                    section::segment::details::Message {
                                        title: "title and body".into(),
                                        id: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5392"),
                                    },
                                ],
                            );
                            h.insert(
                                section::segment::details::Category::Issue("42".into()),
                                vec![
                                    section::segment::details::Message {
                                        title: "Just the title".into(),
                                        id: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5392"),
                                    },
                                    section::segment::details::Message {
                                        title: "another title".into(),
                                        id: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                                    },
                                ],
                            );
                            h
                        },
                    })),
                ],
                unknown: String::new(),
            },
        ],
    };

    for link_mode in &[
        changelog::write::Linkables::AsText,
        changelog::write::Linkables::AsLinks {
            repository_url: git_repository::Url::try_from(b"https://github.com/user/repo".as_bytes())?.into(),
        },
    ] {
        // NOTE: we can't run this a second time as the statistical information will be gone (it was never parsed back)
        let mut md = String::new();
        log.write_to(&mut md, link_mode, changelog::write::Components::all())?;
        insta::assert_snapshot!(md);

        let parsed_log = ChangeLog::from_markdown(&md);
        assert_eq!(parsed_log, log, "we must be able to parse the exact input back");
    }

    for components in &[
        changelog::write::Components::empty(),
        changelog::write::Components::all(),
        changelog::write::Components::DETAIL_TAGS,
    ] {
        for section in &log.sections {
            let mut buf = String::new();
            section.write_to(&mut buf, &changelog::write::Linkables::AsText, *components)?;
            insta::assert_snapshot!(buf);
        }
    }
    Ok(())
}
