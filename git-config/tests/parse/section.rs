use git_config::parse::section;
use git_config::parse::Event;
use std::borrow::Cow;

pub fn header_event(name: &str, subsection: impl Into<Option<(&'static str, &'static str)>>) -> Event<'_> {
    Event::SectionHeader(header(name, subsection))
}

pub fn header(name: &str, subsection: impl Into<Option<(&'static str, &'static str)>>) -> section::Header<'_> {
    let name = section::Name(Cow::Borrowed(name.into()));
    if let Some((separator, subsection_name)) = subsection.into() {
        section::Header {
            name,
            separator: Some(Cow::Borrowed(separator.into())),
            subsection_name: Some(Cow::Borrowed(subsection_name.into())),
        }
    } else {
        section::Header {
            name,
            separator: None,
            subsection_name: None,
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
