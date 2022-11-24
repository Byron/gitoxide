mod section {

    mod header {
        mod unvalidated {
            use crate::parse::section::unvalidated::Key;

            #[test]
            fn section_name_only() {
                assert_eq!(
                    Key::parse("core").unwrap(),
                    Key {
                        section_name: "core",
                        subsection_name: None
                    }
                );
            }

            #[test]
            fn section_name_and_subsection() {
                assert_eq!(
                    Key::parse("core.bare").unwrap(),
                    Key {
                        section_name: "core",
                        subsection_name: Some("bare".into())
                    }
                );
            }

            #[test]
            fn section_name_and_subsection_with_separators() {
                assert_eq!(
                    Key::parse("remote.https:///home/user.git").unwrap(),
                    Key {
                        section_name: "remote",
                        subsection_name: Some("https:///home/user.git".into())
                    }
                );
            }
        }

        mod write_to {
            use std::borrow::Cow;

            use crate::parse::section;

            fn header(name: &str, subsection: impl Into<Option<(&'static str, &'static str)>>) -> section::Header<'_> {
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

            #[test]
            fn legacy_subsection_format_does_not_use_escapes() {
                let invalid = header("invalid", Some((".", "\\ \"")));
                assert_eq!(
                    invalid.to_bstring(),
                    "[invalid.\\ \"]",
                    "no escaping happens for legacy subsections"
                );
                assert!(invalid.is_legacy());
            }

            #[test]
            fn subsections_escape_two_characters_only() {
                let invalid = header("invalid", Some((" ", "\\ \"\npost newline")));
                assert_eq!(
                    invalid.to_bstring(),
                    "[invalid \"\\\\ \\\"\npost newline\"]",
                    "newlines are actually invalid in subsection, but they are possible due to unvalidated instance creation"
                );
                assert!(!invalid.is_legacy());
            }
        }
    }
}

pub(crate) mod util {
    //! This module is only included for tests, and contains common unit test helper
    //! functions.

    use std::{borrow::Cow, convert::TryFrom};

    use crate::parse::{section, Comment, Event};

    pub fn into_events(events: Vec<Event<'_>>) -> section::Events<'_> {
        events.into()
    }

    pub fn section_header(
        name: &str,
        subsection: impl Into<Option<(&'static str, &'static str)>>,
    ) -> section::Header<'_> {
        let name = section::Name::try_from(name).unwrap();
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

    pub(crate) fn name_event(name: &'static str) -> Event<'static> {
        Event::SectionKey(section::Key(Cow::Borrowed(name.into())))
    }

    pub(crate) fn value_event(value: &'static str) -> Event<'static> {
        Event::Value(Cow::Borrowed(value.into()))
    }

    pub(crate) fn value_not_done_event(value: &'static str) -> Event<'static> {
        Event::ValueNotDone(Cow::Borrowed(value.into()))
    }

    pub(crate) fn value_done_event(value: &'static str) -> Event<'static> {
        Event::ValueDone(Cow::Borrowed(value.into()))
    }

    pub(crate) fn newline_event() -> Event<'static> {
        newline_custom_event("\n")
    }

    pub(crate) fn newline_custom_event(value: &'static str) -> Event<'static> {
        Event::Newline(Cow::Borrowed(value.into()))
    }

    pub(crate) fn whitespace_event(value: &'static str) -> Event<'static> {
        Event::Whitespace(Cow::Borrowed(value.into()))
    }

    pub(crate) fn comment_event(tag: char, msg: &'static str) -> Event<'static> {
        Event::Comment(comment(tag, msg))
    }

    pub(crate) fn comment(comment_tag: char, comment: &'static str) -> Comment<'static> {
        Comment {
            tag: comment_tag as u8,
            text: Cow::Borrowed(comment.into()),
        }
    }

    pub(crate) const fn fully_consumed<T>(t: T) -> (&'static [u8], T) {
        (&[], t)
    }
}
