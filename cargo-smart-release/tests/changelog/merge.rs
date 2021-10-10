use cargo_smart_release::{
    changelog,
    changelog::{section, Section},
    ChangeLog,
};
use git_testtools::hex_to_id;
use time::OffsetDateTime;

#[test]
fn sections() {
    let parsed = ChangeLog {
        sections: vec![
            Section::Verbatim {
                text: "preamble".into(),
                generated: false,
            },
            Section::Release {
                heading_level: 3,
                removed_messages: vec![],
                date: Some(
                    time::Date::from_calendar_date(2021, time::Month::September, 14)
                        .unwrap()
                        .midnight()
                        .assume_utc(),
                ),
                name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                segments: Vec::new(),
                unknown: "never changed".into(),
            },
            Section::Verbatim {
                text: "something we couldn't parse".into(),
                generated: false,
            },
            Section::Release {
                heading_level: 3,
                removed_messages: vec![],
                date: None,
                name: changelog::Version::Semantic("0.9.0".parse().unwrap()),
                segments: Vec::new(),
                unknown: String::new(),
            },
        ],
    };
    let generated = ChangeLog {
        sections: vec![
            Section::Verbatim {
                text: "header".into(),
                generated: true,
            },
            Section::Release {
                date: None,
                removed_messages: vec![],
                name: changelog::Version::Unreleased,
                heading_level: 2,
                segments: vec![section::Segment::Clippy(section::Data::Generated(
                    section::segment::ThanksClippy { count: 4 },
                ))],
                unknown: Default::default(),
            },
            Section::Release {
                date: date_m_d(time::Month::September, 15).into(), // generated has a date is 'correct'
                name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                removed_messages: vec![],
                heading_level: 2,
                segments: vec![section::Segment::Clippy(section::Data::Generated(
                    section::segment::ThanksClippy { count: 3 },
                ))],
                unknown: Default::default(),
            },
            Section::Release {
                date: date_m_d(time::Month::July, 1).into(), // generated has a date
                name: changelog::Version::Semantic("0.9.0".parse().unwrap()),
                unknown: String::new(),
                removed_messages: vec![],
                heading_level: 2,
                segments: vec![section::Segment::Clippy(section::Data::Generated(
                    section::segment::ThanksClippy { count: 2 },
                ))],
            },
            Section::Release {
                date: date_m_d(time::Month::June, 1).into(),
                name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                unknown: "undocumented".into(),
                removed_messages: vec![],
                heading_level: 2,
                segments: Vec::new(),
            },
        ],
    };

    let merged = parsed.merge_generated(generated);
    assert_eq!(
        merged,
        ChangeLog {
            sections: vec![
                Section::Verbatim {
                    text: "preamble".into(),
                    generated: false,
                },
                Section::Release {
                    date: None,
                    name: changelog::Version::Unreleased,
                    heading_level: 3,
                    removed_messages: vec![],
                    segments: vec![section::Segment::Clippy(section::Data::Generated(
                        section::segment::ThanksClippy { count: 4 }
                    ))],
                    unknown: Default::default(),
                },
                Section::Release {
                    heading_level: 3,
                    removed_messages: vec![],
                    date: Some(
                        time::Date::from_calendar_date(2021, time::Month::September, 15)
                            .unwrap()
                            .midnight()
                            .assume_utc(),
                    ),
                    name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                    segments: vec![section::Segment::Clippy(section::Data::Generated(
                        section::segment::ThanksClippy { count: 3 }
                    ))],
                    unknown: "never changed".into(),
                },
                Section::Verbatim {
                    text: "something we couldn't parse".into(),
                    generated: false,
                },
                Section::Release {
                    date: date_m_d(time::Month::July, 1).into(),
                    name: changelog::Version::Semantic("0.9.0".parse().unwrap()),
                    unknown: String::new(),
                    heading_level: 3,
                    removed_messages: vec![],
                    segments: vec![section::Segment::Clippy(section::Data::Generated(
                        section::segment::ThanksClippy { count: 2 }
                    ))],
                },
                Section::Release {
                    date: date_m_d(time::Month::June, 1).into(),
                    name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                    unknown: "undocumented".into(),
                    heading_level: 3,
                    removed_messages: vec![],
                    segments: Vec::new(),
                },
            ]
        },
        "retain user content, integrate generated one, as losslessly as possible"
    );
}

#[test]
fn segments() {
    let removed_message_id = hex_to_id("0000000000000000000000000000000000000001");
    let changed_message_id = hex_to_id("0000000000000000000000000000000000000002");
    let parsed = ChangeLog {
        sections: vec![
            Section::Verbatim {
                text: "preamble".into(),
                generated: false,
            },
            Section::Release {
                date: date_m_d(time::Month::January, 1).into(),
                name: changelog::Version::Semantic("0.7.0".parse().unwrap()),
                unknown: "".into(),
                heading_level: 3,
                removed_messages: vec![],
                segments: vec![
                    section::Segment::Conventional(section::segment::Conventional {
                        kind: "feat",
                        is_breaking: false,
                        removed: vec![removed_message_id],
                        messages: vec![
                            section::segment::conventional::Message::User {
                                markdown: "user text".into(),
                            },
                            section::segment::conventional::Message::Generated {
                                id: changed_message_id,
                                title: "content changed by user".to_string(),
                                body: None,
                            },
                        ],
                    }), // conventional is present and prevents new conventionals from showing up
                    section::Segment::Clippy(section::Data::Parsed), // statistical items prevent others from showing up
                ],
            },
            Section::Release {
                date: None,
                name: changelog::Version::Unreleased,
                heading_level: 3,
                removed_messages: vec![],
                segments: vec![section::Segment::Clippy(section::Data::Parsed)], // only clippy still available
                unknown: Default::default(),
            },
            Section::Release {
                heading_level: 3,
                removed_messages: vec![],
                date: Some(
                    time::Date::from_calendar_date(2021, time::Month::September, 15)
                        .unwrap()
                        .midnight()
                        .assume_utc(),
                ),
                name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                segments: vec![section::Segment::User {
                    markdown: "user generated".into(),
                }], // all segments removed, but user segment present
                unknown: "".into(),
            },
            Section::Release {
                date: date_m_d(time::Month::June, 1).into(),
                name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                unknown: "".into(),
                heading_level: 3,
                removed_messages: vec![],
                segments: vec![
                    section::Segment::Details(section::Data::Parsed),
                    section::Segment::Statistics(section::Data::Parsed),
                ],
            },
        ],
    };
    let clippy = section::Segment::Clippy(section::Data::Generated(section::segment::ThanksClippy { count: 42 }));
    let statistics = section::Segment::Statistics(section::Data::Generated(section::segment::CommitStatistics {
        count: 1,
        duration: None,
        conventional_count: 2,
        unique_issues: vec![],
    }));
    let details = section::Segment::Details(section::Data::Generated(section::segment::Details {
        commits_by_category: Default::default(),
    }));
    let added_message_id = hex_to_id("0000000000000000000000000000000000000003");
    let feat_conventional = section::Segment::Conventional(section::segment::Conventional {
        kind: "feat",
        is_breaking: false,
        removed: vec![],
        messages: vec![
            section::segment::conventional::Message::Generated {
                id: removed_message_id,
                title: "something removed".to_string(),
                body: None,
            },
            section::segment::conventional::Message::Generated {
                id: changed_message_id,
                title: "something added/changed".to_string(),
                body: None,
            },
            section::segment::conventional::Message::Generated {
                id: added_message_id,
                title: "to be inserted after user message".to_string(),
                body: None,
            },
        ],
    });
    let segments = vec![statistics.clone(), clippy.clone(), details.clone()];
    let generated = ChangeLog {
        sections: vec![
            Section::Verbatim {
                text: "custom header".into(),
                generated: true,
            },
            Section::Release {
                date: None,
                name: changelog::Version::Unreleased,
                heading_level: 3,
                removed_messages: vec![],
                segments: segments.clone(),
                unknown: Default::default(),
            },
            Section::Release {
                heading_level: 3,
                removed_messages: vec![],
                date: Some(
                    time::Date::from_calendar_date(2021, time::Month::September, 15)
                        .unwrap()
                        .midnight()
                        .assume_utc(),
                ),
                name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                segments: segments.clone(),
                unknown: "".into(),
            },
            Section::Release {
                date: date_m_d(time::Month::June, 1).into(),
                name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                unknown: "".into(),
                heading_level: 3,
                removed_messages: vec![],
                segments: segments.clone(),
            },
            Section::Release {
                date: date_m_d(time::Month::January, 1).into(),
                name: changelog::Version::Semantic("0.7.0".parse().unwrap()),
                unknown: "".into(),
                heading_level: 3,
                removed_messages: vec![],
                segments: {
                    let mut v = segments.clone();
                    v.push(feat_conventional);
                    v
                },
            },
        ],
    };
    let merged = parsed.merge_generated(generated);
    assert_eq!(
        merged,
        ChangeLog {
            sections: vec![
                Section::Verbatim {
                    text: "preamble".into(),
                    generated: false,
                },
                Section::Release {
                    date: date_m_d(time::Month::January, 1).into(),
                    name: changelog::Version::Semantic("0.7.0".parse().unwrap()),
                    unknown: "".into(),
                    heading_level: 3,
                    removed_messages: vec![],
                    segments: vec![
                        section::Segment::Conventional(section::segment::Conventional {
                            kind: "feat",
                            is_breaking: false,
                            removed: vec![removed_message_id],
                            messages: vec![
                                section::segment::conventional::Message::User {
                                    markdown: "user text".into(),
                                },
                                section::segment::conventional::Message::Generated {
                                    id: added_message_id,
                                    title: "to be inserted after user message".to_string(),
                                    body: None
                                }, // new messages are inserted after user content
                                section::segment::conventional::Message::Generated {
                                    id: changed_message_id,
                                    title: "content changed by user".to_string(),
                                    body: None
                                }, // changed user content is preserved, don't overwrite, ever
                            ],
                        }), // conventional is present and prevents new conventionals from showing up, they have messages merged though
                        clippy.clone(), // statistical items prevent others from showing up
                    ],
                }, // merge does not change the order of sections
                Section::Release {
                    date: None,
                    name: changelog::Version::Unreleased,
                    heading_level: 3,
                    removed_messages: vec![],
                    segments: vec![clippy],
                    unknown: Default::default(),
                },
                Section::Release {
                    heading_level: 3,
                    removed_messages: vec![],
                    date: Some(
                        time::Date::from_calendar_date(2021, time::Month::September, 15)
                            .unwrap()
                            .midnight()
                            .assume_utc(),
                    ),
                    name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                    segments: {
                        let mut s = segments;
                        s.insert(
                            0,
                            section::Segment::User {
                                markdown: "user generated".into(),
                            },
                        );
                        s
                    },
                    unknown: "".into(),
                },
                Section::Release {
                    date: date_m_d(time::Month::June, 1).into(),
                    name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                    unknown: "".into(),
                    heading_level: 3,
                    removed_messages: vec![],
                    segments: vec![details, statistics],
                },
            ],
        }
    )
}

fn date_m_d(month: time::Month, day: u8) -> OffsetDateTime {
    time::Date::from_calendar_date(2021, month, day) // generated, correct date
        .unwrap()
        .midnight()
        .assume_utc()
}
