//! This module handles parsing a `git-config`. Generally speaking, you want to
//! use a higher abstraction unless you have some explicit reason to work with
//! events instead.
//!
//! The general workflow for interacting with this is to use one of the
//! `parse_from_*` function variants. These will return a [`Parser`] on success,
//! which can be converted into an [`Event`] iterator. The [`Parser`] also has
//! additional methods for accessing leading comments or events by section.

use nom::bytes::complete::{escaped, tag, take_till, take_while};
use nom::character::complete::{char, none_of, one_of};
use nom::character::{is_newline, is_space};
use nom::combinator::{map, opt};
use nom::error::{Error as NomError, ErrorKind};
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult;
use nom::{branch::alt, multi::many0};
use std::iter::FusedIterator;

/// Syntactic events that occurs in the config.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Event<'a> {
    /// A comment with a comment tag and the comment itself. Note that the
    /// comment itself may contain additional whitespace and comment markers
    /// at the beginning.
    Comment(ParsedComment<'a>),
    /// A section header containing the section name and a subsection, if it
    /// exists.
    SectionHeader(ParsedSectionHeader<'a>),
    /// A name to a value in a section.
    Key(&'a str),
    /// A completed value. This may be any string, including the empty string,
    /// if an implicit boolean value is used. Note that these values may contain
    /// spaces and any special character. This value is also unprocessed, so it
    /// it may contain double quotes that should be replaced.
    Value(&'a str),
    /// Represents any token used to signify a new line character. On Unix
    /// platforms, this is typically just `\n`, but can be any valid newline
    /// sequence.
    Newline(&'a str),
    /// Any value that isn't completed. This occurs when the value is continued
    /// onto the next line. A Newline event is guaranteed after, followed by
    /// either a ValueDone, a Whitespace, or another ValueNotDone.
    ValueNotDone(&'a str),
    /// The last line of a value which was continued onto another line.
    ValueDone(&'a str),
    /// A continuous section of insignificant whitespace. Values with internal
    /// spaces will not be separated by this event.
    Whitespace(&'a str),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ParsedSection<'a> {
    pub section_header: ParsedSectionHeader<'a>,
    pub events: Vec<Event<'a>>,
}

/// A parsed section header, containing a name and optionally a subsection name.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ParsedSectionHeader<'a> {
    pub name: &'a str,
    /// The separator used to determine if the section contains a subsection.
    /// This is either a period `.` or a string of whitespace. Note that
    /// reconstruction of subsection format is dependent on this value. If this
    /// is all whitespace, then the subsection name needs to be surrounded by
    /// quotes to have perfect reconstruction.
    pub separator: Option<&'a str>,
    /// The subsection name without quotes if any exist.
    pub subsection_name: Option<&'a str>,
}

/// A parsed comment event containing the comment marker and comment.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ParsedComment<'a> {
    pub comment_tag: char,
    pub comment: &'a str,
}

#[derive(PartialEq, Debug)]
pub enum ParserError<'a> {
    InvalidInput(nom::Err<NomError<&'a str>>),
    ConfigHasExtraData(&'a str),
}

#[doc(hidden)]
impl<'a> From<nom::Err<NomError<&'a str>>> for ParserError<'a> {
    fn from(e: nom::Err<NomError<&'a str>>) -> Self {
        Self::InvalidInput(e)
    }
}

/// A zero-copy `git-config` file parser.
///
/// This is parser is considered a perfect parser, where a `git-config` file
/// can be identically reconstructed from the events emitted from this parser.
/// Events emitted from this parser are bound to the lifetime of the provided
/// `str` as this parser performs no copies from the input.
///
/// # Differences between a `.ini` parser
///
/// While the `git-config` format closely resembles the [`.ini` file format],
/// there are subtle differences that make them incompatible. For one, the file
/// format is not well defined, and there exists no formal specification to
/// adhere to. Thus, attempting to use an `.ini` parser on a `git-config` file
/// may successfully parse invalid configuration files.
///
/// For concrete examples, some notable differences are:
/// - `git-config` sections permit subsections via either a quoted string
/// (`[some-section "subsection"]`) or via the deprecated dot notation
/// (`[some-section.subsection]`). Successful parsing these section names is not
/// well defined in typical `.ini` parsers. This parser will handle these cases
/// perfectly.
/// - Comment markers are not strictly defined either. This parser will always
/// and only handle a semicolon or octothorpe (also known as a hash or number
/// sign).
/// - Global properties may be allowed in `.ini` parsers, but is strictly
/// disallowed by this parser.
/// - Only `\t`, `\n`, `\b` `\\` are valid escape characters.
/// - Quoted and semi-quoted values will be parsed (but quotes will be included
/// in event outputs). An example of a semi-quoted value is `5"hello world"`,
/// which should be interpreted as `5hello world`.
/// - Line continuations via a `\` character is supported.
/// - Whitespace handling similarly follows the `git-config` specification as
/// closely as possible, where excess whitespace after a non-quoted value are
/// trimmed, and line continuations onto a new line with excess spaces are kept.
/// - Only equal signs (optionally padded by spaces) are valid name/value
/// delimiters.
///
/// Note that that things such as case-sensitivity or duplicate sections are
/// _not_ handled. This parser is a low level _syntactic_ interpreter (as a
/// parser should be), and higher level wrappers around this parser (which may
/// or may not be zero-copy) should handle _semantic_ values. This also means
/// that string-like values are not interpreted. For example, `hello"world"`
/// would be read at a high level as `helloworld` but this parser will return
/// the former instead, with the extra quotes. This is because it is not the
/// responsibility of the parser to interpret these values, and doing so would
/// necessarily require a copy, which this parser avoids.
///
/// # Trait Implementations
///
/// - This struct does _not_ implement [`FromStr`] due to lifetime
/// constraints implied on the required `from_str` method, but instead provides
/// [`Parser::from_str`].
///
/// [`.ini` file format]: https://en.wikipedia.org/wiki/INI_file
/// [`git`'s documentation]: https://git-scm.com/docs/git-config#_configuration_file
/// [`FromStr`]: std::str::FromStr
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Parser<'a> {
    init_comments: Vec<ParsedComment<'a>>,
    sections: Vec<ParsedSection<'a>>,
}

impl<'a> Parser<'a> {
    /// Attempt to zero-copy parse the provided `&str`. On success, returns a
    /// [`Parser`] that provides methods to accessing leading comments and sections
    /// of a `git-config` file and can be converted into an iterator of [`Event`]
    /// for higher level processing.
    ///
    /// This function is identical to [`parse_from_str`].
    ///
    /// # Errors
    ///
    /// Returns an error if the string provided is not a valid file, or we have
    /// non-section data.
    pub fn from_str(s: &'a str) -> Result<Self, ParserError> {
        parse_from_str(s)
    }

    /// Returns the leading comments (any comments before a section) from the
    /// parser. Consider [`Parser::take_leading_comments`] if you need an owned
    /// copy only once.
    pub fn leading_comments(&self) -> &[ParsedComment<'a>] {
        &self.init_comments
    }

    /// Takes the leading comments (any comments before a section) from the
    /// parser. Subsequent calls will return an empty vec. Consider
    /// [`Parser::leading_comments`] if you only need a reference to the comments.
    pub fn take_leading_comments(&mut self) -> Vec<ParsedComment<'a>> {
        let mut to_return = vec![];
        std::mem::swap(&mut self.init_comments, &mut to_return);
        to_return
    }

    pub fn sections(&self) -> &[ParsedSection<'a>] {
        &self.sections
    }

    pub fn take_sections(&mut self) -> Vec<ParsedSection<'a>> {
        let mut to_return = vec![];
        std::mem::swap(&mut self.sections, &mut to_return);
        to_return
    }

    pub fn into_vec(self) -> Vec<Event<'a>> {
        self.into_iter().collect()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Event<'a>> + FusedIterator {
        let section_iter = self
            .sections
            .into_iter()
            .map(|section| {
                vec![Event::SectionHeader(section.section_header)]
                    .into_iter()
                    .chain(section.events)
            })
            .flatten();
        self.init_comments
            .into_iter()
            .map(Event::Comment)
            .chain(section_iter)
    }
}

/// Attempt to zero-copy parse the provided `&str`. On success, returns a
/// [`Parser`] that provides methods to accessing leading comments and sections
/// of a `git-config` file and can be converted into an iterator of [`Event`]
/// for higher level processing.
///
/// # Errors
///
/// Returns an error if the string provided is not a valid file, or we have
/// non-section data.
pub fn parse_from_str(input: &str) -> Result<Parser<'_>, ParserError> {
    let (i, comments) = many0(comment)(input)?;
    let (i, sections) = many1(section)(i)?;

    if !i.is_empty() {
        return Err(ParserError::ConfigHasExtraData(i));
    }

    Ok(Parser {
        init_comments: comments,
        sections,
    })
}

fn comment<'a>(i: &'a str) -> IResult<&'a str, ParsedComment<'a>> {
    let (i, comment_tag) = one_of(";#")(i)?;
    let (i, comment) = take_till(is_char_newline)(i)?;
    Ok((
        i,
        ParsedComment {
            comment_tag,
            comment,
        },
    ))
}

fn section<'a>(i: &'a str) -> IResult<&'a str, ParsedSection<'a>> {
    let (i, section_header) = section_header(i)?;
    let (i, items) = many0(alt((
        map(take_spaces, |space| vec![Event::Whitespace(space)]),
        map(take_newline, |newline| vec![Event::Newline(newline)]),
        map(section_body, |(key, values)| {
            let mut vec = vec![Event::Key(key)];
            vec.extend(values);
            vec
        }),
        map(comment, |comment| vec![Event::Comment(comment)]),
    )))(i)?;
    Ok((
        i,
        ParsedSection {
            section_header,
            events: items.into_iter().flatten().collect(),
        },
    ))
}

fn section_header<'a>(i: &'a str) -> IResult<&'a str, ParsedSectionHeader<'a>> {
    let (i, _) = char('[')(i)?;
    // No spaces must be between section name and section start
    let (i, name) = take_while(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '.')(i)?;

    if let Ok((i, _)) = char::<_, NomError<&str>>(']')(i) {
        // Either section does not have a subsection or using deprecated
        // subsection syntax at this point.
        let header = match name.rfind('.') {
            Some(index) => ParsedSectionHeader {
                name: &name[..index],
                separator: name.get(index..index + 1),
                subsection_name: name.get(index + 1..),
            },
            None => ParsedSectionHeader {
                name: name,
                separator: None,
                subsection_name: None,
            },
        };

        return Ok((i, header));
    }

    // Section header must be using modern subsection syntax at this point.

    let (i, whitespace) = take_spaces(i)?;

    let (i, subsection_name) = delimited(
        char('"'),
        opt(escaped(none_of("\"\\\n\0"), '\\', one_of(r#""\"#))),
        tag("\"]"),
    )(i)?;

    Ok((
        i,
        ParsedSectionHeader {
            name: name,
            separator: Some(whitespace),
            // We know that there's some section name here, so if we get an
            // empty vec here then we actually parsed an empty section name.
            subsection_name: subsection_name.or(Some("")),
        },
    ))
}

fn section_body<'a>(i: &'a str) -> IResult<&'a str, (&'a str, Vec<Event<'a>>)> {
    // maybe need to check for [ here
    let (i, name) = config_name(i)?;
    let (i, whitespace) = opt(take_spaces)(i)?;
    let (i, value) = config_value(i)?;
    if let Some(whitespace) = whitespace {
        let mut events = vec![Event::Whitespace(whitespace)];
        events.extend(value);
        Ok((i, (name, events)))
    } else {
        Ok((i, (name, value)))
    }
}

/// Parses the config name of a config pair. Assumes the input has already been
/// trimmed of any leading whitespace.
fn config_name<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    if i.is_empty() {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::NonEmpty,
        }));
    }

    if !i.chars().nth(0).unwrap().is_alphabetic() {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Alpha,
        }));
    }

    take_while(|c: char| c.is_alphanumeric() || c == '-')(i)
}

fn config_value<'a>(i: &'a str) -> IResult<&'a str, Vec<Event<'a>>> {
    if let (i, Some(_)) = opt(char('='))(i)? {
        let (i, whitespace) = opt(take_spaces)(i)?;
        let (i, values) = value_impl(i)?;
        if let Some(whitespace) = whitespace {
            let mut events = vec![Event::Whitespace(whitespace)];
            events.extend(values);
            Ok((i, events))
        } else {
            Ok((i, values))
        }
    } else {
        Ok((i, vec![Event::Value("")]))
    }
}

fn value_impl<'a>(i: &'a str) -> IResult<&'a str, Vec<Event<'a>>> {
    // I wrote this code and don't know how it works.
    //
    // Even after sleeping on it I still don't know how it works.

    let mut events = vec![];
    let mut parsed_index: usize = 0;
    let mut offset: usize = 0;

    let mut was_prev_char_escape_char = false;
    // This is required to ignore comment markers if they're in a quote.
    let mut is_in_quotes = false;
    // Used to determine if we return a Value or Value{Not,}Done
    let mut partial_value_found = false;

    for (index, c) in i.as_bytes().iter().enumerate() {
        if was_prev_char_escape_char {
            was_prev_char_escape_char = false;
            match c {
                // We're escaping a newline, which means we've found a
                // continuation.
                b'\n' => {
                    partial_value_found = true;
                    events.push(Event::ValueNotDone(&i[offset..index - 1]));
                    events.push(Event::Newline(&i[index..index + 1]));
                    offset = index + 1;
                    parsed_index = 0;
                }
                b't' | b'\\' | b'n' | b'"' => (),
                _ => {
                    return Err(nom::Err::Error(NomError {
                        input: i,
                        code: ErrorKind::Escaped,
                    }));
                }
            }
        } else {
            match c {
                b'\n' => {
                    parsed_index = index;
                    break;
                }
                b';' | b'#' if !is_in_quotes => {
                    parsed_index = index;
                    break;
                }
                b'\\' => was_prev_char_escape_char = true,
                b'"' => is_in_quotes = !is_in_quotes,
                _ => {}
            }
        }
    }

    if parsed_index == 0 {
        parsed_index = i.len();
    }

    // Handle incomplete escape
    if was_prev_char_escape_char {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Escaped,
        }));
    }

    // Handle incomplete quotes
    if is_in_quotes {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Tag,
        }));
    }

    let (i, remainder_value) = {
        let mut new_index = parsed_index;
        for index in (offset..parsed_index).rev() {
            if !(i.as_bytes()[index] as char).is_whitespace() {
                new_index = index + 1;
                break;
            }
        }
        (&i[new_index..], &i[offset..new_index])
    };

    if partial_value_found {
        events.push(Event::ValueDone(remainder_value));
    } else {
        events.push(Event::Value(remainder_value));
    }

    Ok((i, events))
}

fn is_char_newline(c: char) -> bool {
    c.is_ascii() && is_newline(c as u8)
}

fn take_spaces<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    take_common(i, |c: char| c.is_ascii() && is_space(c as u8))
}

fn take_newline<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    take_common(i, is_char_newline)
}

fn take_common<'a, F: Fn(char) -> bool>(i: &'a str, f: F) -> IResult<&'a str, &'a str> {
    let (i, v) = take_while(f)(i)?;
    if v.is_empty() {
        Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Eof,
        }))
    } else {
        Ok((i, v))
    }
}
#[cfg(test)]
mod parse {
    use super::*;

    fn fully_consumed<T>(t: T) -> (&'static str, T) {
        ("", t)
    }

    fn gen_section_header(
        name: &str,
        subsection: impl Into<Option<(&'static str, &'static str)>>,
    ) -> ParsedSectionHeader<'_> {
        if let Some((separator, subsection_name)) = subsection.into() {
            ParsedSectionHeader {
                name,
                separator: Some(separator),
                subsection_name: Some(subsection_name),
            }
        } else {
            ParsedSectionHeader {
                name,
                separator: None,
                subsection_name: None,
            }
        }
    }

    mod comments {
        use super::super::*;
        use super::*;

        #[test]
        fn semicolon() {
            assert_eq!(
                comment("; this is a semicolon comment").unwrap(),
                fully_consumed(ParsedComment {
                    comment_tag: ';',
                    comment: " this is a semicolon comment",
                })
            );
        }

        #[test]
        fn octothorpe() {
            assert_eq!(
                comment("# this is an octothorpe comment").unwrap(),
                fully_consumed(ParsedComment {
                    comment_tag: '#',
                    comment: " this is an octothorpe comment",
                })
            );
        }

        #[test]
        fn multiple_markers() {
            assert_eq!(
                comment("###### this is an octothorpe comment").unwrap(),
                fully_consumed(ParsedComment {
                    comment_tag: '#',
                    comment: "##### this is an octothorpe comment",
                })
            );
        }
    }

    mod section_headers {
        use super::super::*;
        use super::*;

        #[test]
        fn no_subsection() {
            assert_eq!(
                section_header("[hello]").unwrap(),
                fully_consumed(gen_section_header("hello", None)),
            );
        }

        #[test]
        fn modern_subsection() {
            assert_eq!(
                section_header(r#"[hello "world"]"#).unwrap(),
                fully_consumed(gen_section_header("hello", (" ", "world"))),
            );
        }

        #[test]
        fn escaped_subsection() {
            assert_eq!(
                section_header(r#"[hello "foo\\bar\""]"#).unwrap(),
                fully_consumed(gen_section_header("hello", (" ", r#"foo\\bar\""#))),
            );
        }

        #[test]
        fn deprecated_subsection() {
            assert_eq!(
                section_header(r#"[hello.world]"#).unwrap(),
                fully_consumed(gen_section_header("hello", (".", "world")))
            );
        }

        #[test]
        fn empty_legacy_subsection_name() {
            assert_eq!(
                section_header(r#"[hello.]"#).unwrap(),
                fully_consumed(gen_section_header("hello", (".", "")))
            );
        }

        #[test]
        fn empty_modern_subsection_name() {
            assert_eq!(
                section_header(r#"[hello ""]"#).unwrap(),
                fully_consumed(gen_section_header("hello", (" ", "")))
            );
        }

        #[test]
        fn newline_in_header() {
            assert!(section_header("[hello\n]").is_err())
        }

        #[test]
        fn null_byte_in_header() {
            assert!(section_header("[hello\0]").is_err())
        }

        #[test]
        fn right_brace_in_subsection_name() {
            assert_eq!(
                section_header(r#"[hello "]"]"#).unwrap(),
                fully_consumed(gen_section_header("hello", (" ", "]")))
            );
        }
    }

    mod config_name {
        use super::super::*;
        use super::*;

        #[test]
        fn just_name() {
            assert_eq!(config_name("name").unwrap(), fully_consumed("name"));
        }

        #[test]
        fn must_start_with_alphabetic() {
            assert!(config_name("4aaa").is_err());
            assert!(config_name("-aaa").is_err());
        }

        #[test]
        fn cannot_be_empty() {
            assert!(config_name("").is_err())
        }
    }

    mod value_no_continuation {
        use super::super::*;
        use super::*;

        #[test]
        fn no_comment() {
            assert_eq!(
                value_impl("hello").unwrap(),
                fully_consumed(vec![Event::Value("hello")])
            );
        }

        #[test]
        fn no_comment_newline() {
            assert_eq!(
                value_impl("hello\na").unwrap(),
                ("\na", vec![Event::Value("hello")])
            )
        }

        #[test]
        fn semicolon_comment_not_consumed() {
            assert_eq!(
                value_impl("hello;world").unwrap(),
                (";world", vec![Event::Value("hello"),])
            );
        }

        #[test]
        fn octothorpe_comment_not_consumed() {
            assert_eq!(
                value_impl("hello#world").unwrap(),
                ("#world", vec![Event::Value("hello"),])
            );
        }

        #[test]
        fn values_with_extraneous_whitespace_without_comment() {
            assert_eq!(
                value_impl("hello               ").unwrap(),
                ("               ", vec![Event::Value("hello")])
            );
        }

        #[test]
        fn values_with_extraneous_whitespace_before_comment() {
            assert_eq!(
                value_impl("hello             #world").unwrap(),
                ("             #world", vec![Event::Value("hello"),])
            );
            assert_eq!(
                value_impl("hello             ;world").unwrap(),
                ("             ;world", vec![Event::Value("hello"),])
            );
        }

        #[test]
        fn trans_escaped_comment_marker_not_consumed() {
            assert_eq!(
                value_impl(r##"hello"#"world; a"##).unwrap(),
                ("; a", vec![Event::Value(r##"hello"#"world"##)])
            );
        }

        #[test]
        fn complex_test() {
            assert_eq!(
                value_impl(r#"value";";ahhhh"#).unwrap(),
                (";ahhhh", vec![Event::Value(r#"value";""#)])
            );
        }

        #[test]
        fn garbage_after_continution_is_err() {
            assert!(value_impl("hello \\afwjdls").is_err());
        }
    }

    mod value_continuation {
        use super::super::*;
        use super::*;

        #[test]
        fn simple_continuation() {
            assert_eq!(
                value_impl("hello\\\nworld").unwrap(),
                fully_consumed(vec![
                    Event::ValueNotDone("hello"),
                    Event::Newline("\n"),
                    Event::ValueDone("world")
                ])
            );
        }
        #[test]
        fn complex_continuation_with_leftover_comment() {
            assert_eq!(
                value_impl("1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c").unwrap(),
                (
                    " # \"b\t ; c",
                    vec![
                        Event::ValueNotDone(r#"1    "\""#),
                        Event::Newline("\n"),
                        Event::ValueNotDone(r#"a ; e "\""#),
                        Event::Newline("\n"),
                        Event::ValueDone("d"),
                    ]
                )
            );
        }

        #[test]
        fn quote_split_over_two_lines_with_leftover_comment() {
            assert_eq!(
                value_impl("\"\\\n;\";a").unwrap(),
                (
                    ";a",
                    vec![
                        Event::ValueNotDone("\""),
                        Event::Newline("\n"),
                        Event::ValueDone(";\""),
                    ]
                )
            )
        }
    }

    mod section {
        use super::super::*;
        use super::*;

        #[test]
        fn empty_section() {
            assert_eq!(
                section("[test]").unwrap(),
                fully_consumed(ParsedSection {
                    section_header: gen_section_header("test", None),
                    events: vec![]
                })
            );
        }

        #[test]
        fn simple_section() {
            let section_data = r#"[hello]
            a = b
            c
            d = "lol""#;
            assert_eq!(
                section(section_data).unwrap(),
                fully_consumed(ParsedSection {
                    section_header: gen_section_header("hello", None),
                    events: vec![
                        Event::Newline("\n"),
                        Event::Whitespace("            "),
                        Event::Key("a"),
                        Event::Whitespace(" "),
                        Event::Whitespace(" "),
                        Event::Value("b"),
                        Event::Newline("\n"),
                        Event::Whitespace("            "),
                        Event::Key("c"),
                        Event::Value(""),
                        Event::Newline("\n"),
                        Event::Whitespace("            "),
                        Event::Key("d"),
                        Event::Whitespace(" "),
                        Event::Whitespace(" "),
                        Event::Value("\"lol\"")
                    ]
                })
            )
        }

        #[test]
        fn section_single_line() {
            assert_eq!(
                section("[hello] c").unwrap(),
                fully_consumed(ParsedSection {
                    section_header: gen_section_header("hello", None),
                    events: vec![Event::Whitespace(" "), Event::Key("c"), Event::Value("")]
                })
            );
        }

        #[test]
        fn section_very_commented() {
            let section_data = r#"[hello] ; commentA
            a = b # commentB
            ; commentC
            ; commentD
            c = d"#;
            assert_eq!(
                section(section_data).unwrap(),
                fully_consumed(ParsedSection {
                    section_header: gen_section_header("hello", None),
                    events: vec![
                        Event::Whitespace(" "),
                        Event::Comment(ParsedComment {
                            comment_tag: ';',
                            comment: " commentA",
                        }),
                        Event::Newline("\n"),
                        Event::Whitespace("            "),
                        Event::Key("a"),
                        Event::Whitespace(" "),
                        Event::Whitespace(" "),
                        Event::Value("b"),
                        Event::Whitespace(" "),
                        Event::Comment(ParsedComment {
                            comment_tag: '#',
                            comment: " commentB",
                        }),
                        Event::Newline("\n"),
                        Event::Whitespace("            "),
                        Event::Comment(ParsedComment {
                            comment_tag: ';',
                            comment: " commentC",
                        }),
                        Event::Newline("\n"),
                        Event::Whitespace("            "),
                        Event::Comment(ParsedComment {
                            comment_tag: ';',
                            comment: " commentD",
                        }),
                        Event::Newline("\n"),
                        Event::Whitespace("            "),
                        Event::Key("c"),
                        Event::Whitespace(" "),
                        Event::Whitespace(" "),
                        Event::Value("d"),
                    ]
                })
            );
        }

        #[test]
        fn complex_continuation() {
            // This test is absolute hell. Good luck if this fails.
            assert_eq!(
                section("[section] a = 1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c").unwrap(),
                fully_consumed(ParsedSection {
                    section_header: gen_section_header("section", None),
                    events: vec![
                        Event::Whitespace(" "),
                        Event::Key("a"),
                        Event::Whitespace(" "),
                        Event::Whitespace(" "),
                        Event::ValueNotDone(r#"1    "\""#),
                        Event::Newline("\n"),
                        Event::ValueNotDone(r#"a ; e "\""#),
                        Event::Newline("\n"),
                        Event::ValueDone("d"),
                        Event::Whitespace(" "),
                        Event::Comment(ParsedComment {
                            comment_tag: '#',
                            comment: " \"b\t ; c"
                        })
                    ]
                })
            );
        }

        #[test]
        fn quote_split_over_two_lines() {
            assert_eq!(
                section("[section \"a\"] b =\"\\\n;\";a").unwrap(),
                fully_consumed(ParsedSection {
                    section_header: gen_section_header("section", (" ", "a")),
                    events: vec![
                        Event::Whitespace(" "),
                        Event::Key("b"),
                        Event::Whitespace(" "),
                        Event::ValueNotDone("\""),
                        Event::Newline("\n"),
                        Event::ValueDone(";\""),
                        Event::Comment(ParsedComment {
                            comment_tag: ';',
                            comment: "a",
                        })
                    ]
                })
            )
        }

        #[test]
        fn section_handles_extranous_whitespace_before_comment() {
            assert_eq!(
                section("[s]hello             #world").unwrap(),
                fully_consumed(ParsedSection {
                    section_header: gen_section_header("s", None),
                    events: vec![
                        Event::Key("hello"),
                        Event::Whitespace("             "),
                        Event::Value(""),
                        Event::Comment(ParsedComment {
                            comment_tag: '#',
                            comment: "world",
                        }),
                    ]
                })
            );
        }
    }
}
