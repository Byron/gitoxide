use git_config::parse::section;
use git_config::parse::Event;
use std::borrow::Cow;

pub fn header_event(name: &'static str, subsection: impl Into<Option<&'static str>>) -> Event<'static> {
    Event::SectionHeader(section::Header::new(name, subsection.into().map(Cow::Borrowed)).unwrap())
}

mod header {
    mod new {
        use git_config::parse::section;

        #[test]
        fn names_must_be_mostly_ascii() {
            assert_eq!(
                section::Header::new("🤗", None),
                Err(section::header::Error::InvalidName)
            );
            assert_eq!(
                section::Header::new("x.y", None),
                Err(section::header::Error::InvalidName)
            );
            assert_eq!(
                section::Header::new("x y", None),
                Err(section::header::Error::InvalidName)
            );
            assert_eq!(
                section::Header::new("x\ny", None),
                Err(section::header::Error::InvalidName)
            );
        }

        #[test]
        #[ignore]
        fn subsections_with_newlines_and_null_bytes_are_rejected() {}
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
