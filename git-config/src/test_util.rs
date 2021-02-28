use std::borrow::Cow;

use crate::parser::{Event, ParsedComment, ParsedSectionHeader};

pub fn section_header_event(
    name: &str,
    subsection: impl Into<Option<(&'static str, &'static str)>>,
) -> Event<'_> {
    Event::SectionHeader(section_header(name, subsection))
}

pub fn section_header(
    name: &str,
    subsection: impl Into<Option<(&'static str, &'static str)>>,
) -> ParsedSectionHeader<'_> {
    let name = name.into();
    if let Some((separator, subsection_name)) = subsection.into() {
        ParsedSectionHeader {
            name,
            separator: Some(Cow::Borrowed(separator)),
            subsection_name: Some(Cow::Borrowed(subsection_name)),
        }
    } else {
        ParsedSectionHeader {
            name,
            separator: None,
            subsection_name: None,
        }
    }
}

pub(crate) fn name_event(name: &'static str) -> Event<'static> {
    Event::Key(Cow::Borrowed(name))
}

pub(crate) fn value_event(value: &'static str) -> Event<'static> {
    Event::Value(Cow::Borrowed(value.as_bytes()))
}

pub(crate) fn value_not_done_event(value: &'static str) -> Event<'static> {
    Event::ValueNotDone(Cow::Borrowed(value.as_bytes()))
}

pub(crate) fn value_done_event(value: &'static str) -> Event<'static> {
    Event::ValueDone(Cow::Borrowed(value.as_bytes()))
}

pub(crate) fn newline_event() -> Event<'static> {
    newline_custom_event("\n")
}

pub(crate) fn newline_custom_event(value: &'static str) -> Event<'static> {
    Event::Newline(Cow::Borrowed(value))
}

pub(crate) fn whitespace_event(value: &'static str) -> Event<'static> {
    Event::Whitespace(Cow::Borrowed(value))
}

pub(crate) fn comment_event(tag: char, msg: &'static str) -> Event<'static> {
    Event::Comment(comment(tag, msg))
}

pub(crate) fn comment(comment_tag: char, comment: &'static str) -> ParsedComment<'static> {
    ParsedComment {
        comment_tag,
        comment: Cow::Borrowed(comment.as_bytes()),
    }
}

pub(crate) fn fully_consumed<T>(t: T) -> (&'static [u8], T) {
    (&[], t)
}
