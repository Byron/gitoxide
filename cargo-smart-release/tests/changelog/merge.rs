use cargo_smart_release::{
    changelog,
    changelog::{section, Section},
    ChangeLog,
};
use time::OffsetDateTime;

#[test]
fn only_last_release_without_unreleased_section() {
    let parsed = ChangeLog {
        sections: vec![
            Section::Verbatim {
                text: "preamble".into(),
                generated: false,
            },
            Section::Release {
                heading_level: 3,
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
                name: changelog::Version::Unreleased,
                heading_level: 2,
                segments: vec![section::Segment::Clippy(section::Data::Generated(
                    section::ThanksClippy { count: 4 },
                ))],
                unknown: Default::default(),
            },
            Section::Release {
                date: date_m_d(time::Month::September, 15).into(), // generated has a date is 'correct'
                name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                heading_level: 2,
                segments: vec![section::Segment::Clippy(section::Data::Generated(
                    section::ThanksClippy { count: 3 },
                ))],
                unknown: Default::default(),
            },
            Section::Release {
                date: date_m_d(time::Month::July, 1).into(), // generated has a date
                name: changelog::Version::Semantic("0.9.0".parse().unwrap()),
                unknown: String::new(),
                heading_level: 2,
                segments: vec![section::Segment::Clippy(section::Data::Generated(
                    section::ThanksClippy { count: 2 },
                ))],
            },
            Section::Release {
                date: date_m_d(time::Month::June, 1).into(),
                name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                unknown: "undocumented".into(),
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
                    segments: vec![section::Segment::Clippy(section::Data::Generated(
                        section::ThanksClippy { count: 4 }
                    ))],
                    unknown: Default::default(),
                },
                Section::Release {
                    heading_level: 3,
                    date: Some(
                        time::Date::from_calendar_date(2021, time::Month::September, 15)
                            .unwrap()
                            .midnight()
                            .assume_utc(),
                    ),
                    name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                    segments: vec![section::Segment::Clippy(section::Data::Generated(
                        section::ThanksClippy { count: 3 }
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
                    segments: vec![section::Segment::Clippy(section::Data::Generated(
                        section::ThanksClippy { count: 2 }
                    ))],
                },
                Section::Release {
                    date: date_m_d(time::Month::June, 1).into(),
                    name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                    unknown: "undocumented".into(),
                    heading_level: 3,
                    segments: Vec::new(),
                },
            ]
        },
        "retain user content, integrate generated one, as losslessly as possible"
    );
}

#[test]
fn into_previously_generated_with_removed_statistical_segments() {
    let parsed = ChangeLog {
        sections: vec![
            Section::Verbatim {
                text: "preamble".into(),
                generated: false,
            },
            Section::Release {
                date: None,
                name: changelog::Version::Unreleased,
                heading_level: 3,
                segments: vec![section::Segment::Clippy(section::Data::Parsed)], // only clippy still available
                unknown: Default::default(),
            },
            Section::Release {
                heading_level: 3,
                date: Some(
                    time::Date::from_calendar_date(2021, time::Month::September, 15)
                        .unwrap()
                        .midnight()
                        .assume_utc(),
                ),
                name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                segments: vec![], // all segments removed
                unknown: "".into(),
            },
            Section::Release {
                date: date_m_d(time::Month::June, 1).into(),
                name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                unknown: "".into(),
                heading_level: 3,
                segments: vec![
                    section::Segment::Details(section::Data::Parsed),
                    section::Segment::Statistics(section::Data::Parsed),
                ],
            },
        ],
    };
    let clippy = section::Segment::Clippy(section::Data::Generated(section::ThanksClippy { count: 42 }));
    let statistics = section::Segment::Statistics(section::Data::Generated(section::CommitStatistics {
        count: 1,
        duration: None,
        conventional_count: 2,
        unique_issues_count: 3,
    }));
    let details = section::Segment::Details(section::Data::Generated(section::Details {
        commits_by_category: Default::default(),
    }));
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
                segments: segments.clone(),
                unknown: Default::default(),
            },
            Section::Release {
                heading_level: 3,
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
                segments: segments.clone(),
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
                    segments: vec![clippy],
                    unknown: Default::default(),
                },
                Section::Release {
                    heading_level: 3,
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
