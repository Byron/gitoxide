mod write {
    use git_repository::bstr::ByteSlice;

    use crate::{changelog, changelog::Section, ChangeLog};

    #[test]
    fn all_section_types() {
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
                    date: Some(time::OffsetDateTime::from_unix_timestamp(123456).unwrap()),
                    name: changelog::Version::Semantic("1.0.2-beta.2".parse().unwrap()),
                },
            ],
        };
        let mut buf = Vec::<u8>::new();
        log.write_to(&mut buf).unwrap();
        insta::assert_snapshot!(buf.to_str().unwrap());
    }
}
