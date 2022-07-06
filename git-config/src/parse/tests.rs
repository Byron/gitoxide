mod parse {
    use crate::parse::State;

    #[test]
    fn parser_skips_bom() {
        let bytes = b"
        [core]
            a = 1
    ";
        let bytes_with_gb18030_bom = "\u{feff}
        [core]
            a = 1
    ";

        assert_eq!(
            State::from_bytes(bytes),
            State::from_bytes(bytes_with_gb18030_bom.as_bytes())
        );
        assert_eq!(
            State::from_bytes_owned(bytes),
            State::from_bytes_owned(bytes_with_gb18030_bom.as_bytes())
        );
    }
}

#[cfg(test)]
mod error {
    use crate::parse::State;

    #[test]
    fn line_no_is_one_indexed() {
        assert_eq!(State::from_str("[hello").unwrap_err().line_number(), 1);
    }

    #[test]
    fn remaining_data_contains_bad_tokens() {
        assert_eq!(State::from_str("[hello").unwrap_err().remaining_data(), b"[hello");
    }

    #[test]
    fn to_string_truncates_extra_values() {
        assert_eq!(
            State::from_str("[1234567890").unwrap_err().to_string(),
            "Got an unexpected token on line 1 while trying to parse a section header: '[123456789' ... (1 characters omitted)"
        );
    }
}

pub(crate) mod util {
    //! This module is only included for tests, and contains common unit test helper
    //! functions.

    use std::borrow::Cow;

    use crate::parse::{Event, Key, ParsedComment, ParsedSectionHeader};

    pub fn section_header(
        name: &str,
        subsection: impl Into<Option<(&'static str, &'static str)>>,
    ) -> ParsedSectionHeader<'_> {
        let name = name.into();
        if let Some((separator, subsection_name)) = subsection.into() {
            ParsedSectionHeader {
                name,
                separator: Some(Cow::Borrowed(separator.into())),
                subsection_name: Some(Cow::Borrowed(subsection_name.into())),
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
        Event::Key(Key(Cow::Borrowed(name.into())))
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

    pub(crate) fn comment(comment_tag: char, comment: &'static str) -> ParsedComment<'static> {
        ParsedComment {
            comment_tag: comment_tag as u8,
            comment: Cow::Borrowed(comment.into()),
        }
    }

    pub(crate) const fn fully_consumed<T>(t: T) -> (&'static [u8], T) {
        (&[], t)
    }
}
