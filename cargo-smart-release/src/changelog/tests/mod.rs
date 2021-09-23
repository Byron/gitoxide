mod write_and_parse {
    use git_repository::bstr::ByteSlice;

    use crate::{changelog, changelog::Section, ChangeLog};

    #[test]
    fn all_section_types_round_trip() {
        let log = ChangeLog {
            sections: vec![
                Section::Verbatim {
                    text: "# Changelog\n\nmy very own header\n\n".into(),
                    generated: false,
                },
                Section::Release {
                    heading_level: 2,
                    date: None,
                    name: changelog::Version::Unreleased,
                },
                Section::Release {
                    heading_level: 4,
                    date: Some(time::OffsetDateTime::from_unix_timestamp(0).unwrap()),
                    name: changelog::Version::Semantic("1.0.2-beta.2".parse().unwrap()),
                },
            ],
        };
        let mut buf = Vec::<u8>::new();
        log.write_to(&mut buf).unwrap();
        let md = buf.to_str().unwrap();
        insta::assert_snapshot!(md);

        assert_eq!(
            ChangeLog::from_markdown(md),
            log,
            "we must be able to parse the exact input back"
        );
    }
}
