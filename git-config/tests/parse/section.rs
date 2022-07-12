use git_config::parse::section;
use git_config::parse::Event;
use std::borrow::Cow;

pub fn header_event(name: &'static str, subsection: impl Into<Option<&'static str>>) -> Event<'static> {
    Event::SectionHeader(section::Header::new(name, subsection.into().map(Cow::Borrowed)).unwrap())
}

mod header {
    mod write_to {
        use git_config::parse::section;
        use std::borrow::Cow;

        #[test]
        fn subsection_backslashes_and_quotes_are_escaped() -> crate::Result {
            assert_eq!(
                section::Header::new("core", Cow::from(r#"a\b"#))?.to_bstring(),
                r#"[core "a\\b"]"#
            );
            assert_eq!(
                section::Header::new("core", Cow::from(r#"a:"b""#))?.to_bstring(),
                r#"[core "a:\"b\""]"#
            );
            Ok(())
        }

        #[test]
        fn everything_is_allowed() -> crate::Result {
            assert_eq!(
                section::Header::new("core", Cow::from("a/b \t\t a\\b"))?.to_bstring(),
                "[core \"a/b \t\t a\\\\b\"]"
            );
            Ok(())
        }
    }
    mod new {
        use git_config::parse::section;
        use std::borrow::Cow;

        #[test]
        fn names_must_be_mostly_ascii() {
            for name in ["🤗", "x.y", "x y", "x\ny"] {
                assert_eq!(
                    section::Header::new(name, None),
                    Err(section::header::Error::InvalidName)
                );
            }
        }

        #[test]
        fn subsections_with_newlines_and_null_bytes_are_rejected() {
            assert_eq!(
                section::Header::new("a", Cow::from("a\nb")),
                Err(section::header::Error::InvalidSubSection)
            );
            assert_eq!(
                section::Header::new("a", Cow::from("a\0b")),
                Err(section::header::Error::InvalidSubSection)
            );
        }
    }
}

mod key {
    use git_config::parse::section::Key;
    use std::cmp::Ordering;

    #[test]
    fn case_insentive_eq() {
        assert_eq!(Key::from("aBc"), Key::from("AbC"));
    }

    #[test]
    fn case_insentive_ord() {
        assert_eq!(Key::from("a").cmp(&Key::from("a")), Ordering::Equal);
        assert_eq!(Key::from("aBc").cmp(&Key::from("AbC")), Ordering::Equal);
    }

    #[test]
    fn case_insentive_hash() {
        fn calculate_hash<T: std::hash::Hash>(t: T) -> u64 {
            use std::hash::Hasher;
            let mut s = std::collections::hash_map::DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        assert_eq!(calculate_hash(Key::from("aBc")), calculate_hash(Key::from("AbC")));
    }
}
