type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod write_and_parse {
    use git_repository::bstr::ByteSlice;

    use super::Result;
    use crate::{changelog, changelog::Section, ChangeLog};

    #[test]
    fn all_section_types_round_trip() -> Result {
        let mut log = ChangeLog {
            sections: vec![
                Section::Verbatim {
                    text: "# Changelog\n\nmy very own header\n\n".into(),
                    generated: false,
                },
                Section::Release {
                    heading_level: 2,
                    date: None,
                    thanks_clippy_count: 0,
                    name: changelog::Version::Unreleased,
                    unknown: "hello\nworld\n".into(),
                },
                Section::Release {
                    heading_level: 4,
                    thanks_clippy_count: 42,
                    date: Some(time::OffsetDateTime::from_unix_timestamp(0)?),
                    name: changelog::Version::Semantic("1.0.2-beta.2".parse()?),
                    unknown: String::new(),
                },
            ],
        };
        for _round in 0..2 {
            let mut buf = Vec::<u8>::new();
            log.write_to(&mut buf)?;
            let md = buf.to_str()?;
            insta::assert_snapshot!(md);

            let parsed_log = ChangeLog::from_markdown(md);
            assert_eq!(parsed_log, log, "we must be able to parse the exact input back");
            log = parsed_log;
        }
        Ok(())
    }
}

mod merge {
    use time::OffsetDateTime;

    use crate::{changelog, changelog::Section, ChangeLog};

    #[test]
    fn into_only_last_release_without_unreleased_section() {
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
                    thanks_clippy_count: 0,
                    name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                    unknown: "never changed".into(),
                },
                Section::Verbatim {
                    text: "something we couldn't parse".into(),
                    generated: false,
                },
                Section::Release {
                    heading_level: 3,
                    date: None,
                    thanks_clippy_count: 0,
                    name: changelog::Version::Semantic("0.9.0".parse().unwrap()),
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
                    thanks_clippy_count: 4,
                    unknown: Default::default(),
                },
                Section::Release {
                    date: date_m_d(time::Month::September, 15).into(), // generated has a date is 'correct'
                    name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
                    heading_level: 2,
                    thanks_clippy_count: 3,
                    unknown: Default::default(),
                },
                Section::Release {
                    date: date_m_d(time::Month::July, 1).into(), // generated has a date
                    name: changelog::Version::Semantic("0.9.0".parse().unwrap()),
                    unknown: String::new(),
                    heading_level: 2,
                    thanks_clippy_count: 2,
                },
                Section::Release {
                    date: date_m_d(time::Month::June, 1).into(),
                    name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                    unknown: "undocumented".into(),
                    heading_level: 2,
                    thanks_clippy_count: 0,
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
                        thanks_clippy_count: 4,
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
                        thanks_clippy_count: 3,
                        name: changelog::Version::Semantic("1.0.0".parse().unwrap()),
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
                        thanks_clippy_count: 2,
                    },
                    Section::Release {
                        date: date_m_d(time::Month::June, 1).into(),
                        name: changelog::Version::Semantic("0.8.0".parse().unwrap()),
                        unknown: "undocumented".into(),
                        heading_level: 3,
                        thanks_clippy_count: 0,
                    },
                ]
            },
            "retain user content, integrate generated one, as losslessly as possible"
        );
    }

    fn date_m_d(month: time::Month, day: u8) -> OffsetDateTime {
        time::Date::from_calendar_date(2021, month, day) // generated, correct date
            .unwrap()
            .midnight()
            .assume_utc()
    }
}
