pub(crate) mod util {
    //! This module is only included for tests, and contains common unit test helper
    //! functions.

    use std::borrow::Cow;

    use crate::parse::{section, Comment, Event};

    pub fn into_events(events: Vec<Event<'_>>) -> section::Events<'_> {
        events.into()
    }

    pub fn section_header(
        name: &str,
        subsection: impl Into<Option<(&'static str, &'static str)>>,
    ) -> section::Header<'_> {
        let name = name.into();
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
            comment_tag: comment_tag as u8,
            comment: Cow::Borrowed(comment.into()),
        }
    }

    pub(crate) const fn fully_consumed<T>(t: T) -> (&'static [u8], T) {
        (&[], t)
    }
}
