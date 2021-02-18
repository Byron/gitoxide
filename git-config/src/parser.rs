use crate::values::{Boolean, TrueVariant, Value};
use nom::bytes::complete::{escaped, tag, take_till, take_while};
use nom::character::complete::{char, none_of, one_of};
use nom::character::{is_newline, is_space};
use nom::combinator::{map, opt};
use nom::error::{Error as NomError, ErrorKind};
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult;
use nom::{branch::alt, multi::many0};

/// An event is any syntactic event that occurs in the config.
#[derive(PartialEq, Debug)]
pub enum Event<'a> {
    Comment(Comment<'a>),
    Key(&'a str),
    /// Represents any token used to signify a new line character. On Unix
    /// platforms, this is typically just `\n`, but can be any valid newline
    /// sequence.
    Newline(&'a str),
    ///
    Value(Value<'a>),
    /// Any value that isn't completed. This occurs when the value is continued
    /// onto the next line. A Newline event is guaranteed after, followed by
    /// either another ValueNotDone or a ValueDone.
    ValueNotDone(&'a str),
    /// The last line of a value which was continued onto another line.
    ValueDone(&'a str),
}

#[derive(PartialEq, Debug)]
pub struct Section<'a> {
    section_header: SectionHeader<'a>,
    items: Vec<Event<'a>>,
}

#[derive(PartialEq, Debug)]
pub struct SectionHeader<'a> {
    name: &'a str,
    subsection_name: Option<&'a str>,
}

#[derive(PartialEq, Debug)]
pub struct Comment<'a> {
    comment_tag: char,
    comment: &'a str,
}

pub struct Parser<'a> {
    init_comments: Vec<Comment<'a>>,
    sections: Vec<Section<'a>>,
}

pub fn parse(input: &str) -> Result<Parser<'_>, ()> {
    let (i, comments) = many0(comment)(input).unwrap();
    let (i, sections) = many1(section)(i).unwrap();

    Ok(Parser {
        init_comments: comments,
        sections,
    })
}

fn comment<'a>(i: &'a str) -> IResult<&'a str, Comment<'a>> {
    let i = i.trim_start();
    let (i, comment_tag) = one_of(";#")(i)?;
    let (i, comment) = take_till(is_char_newline)(i)?;
    Ok((
        i,
        Comment {
            comment_tag,
            comment,
        },
    ))
}

fn section<'a>(i: &'a str) -> IResult<&'a str, Section<'a>> {
    let (i, section_header) = section_header(i)?;
    // need alt here for eof?
    let (i, items) = many1(alt((
        map(section_body, |(key, values)| {
            let mut vec = vec![Event::Key(key)];
            vec.extend(values);
            vec
        }),
        map(comment, |comment| vec![Event::Comment(comment)]),
    )))(i)?;
    Ok((
        i,
        Section {
            section_header,
            items: items.into_iter().flatten().collect(),
        },
    ))
}

fn section_header<'a>(i: &'a str) -> IResult<&'a str, SectionHeader<'a>> {
    let (i, _) = char('[')(i)?;
    // No spaces must be between section name and section start
    let (i, name) = take_while(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '.')(i)?;

    if let Ok((i, _)) = char::<_, NomError<&str>>(']')(i) {
        // Either section does not have a subsection or using deprecated
        // subsection syntax at this point.
        let header = match name.rfind('.') {
            Some(index) => SectionHeader {
                name: &name[..index],
                subsection_name: Some(&name[index + 1..]),
            },
            None => SectionHeader {
                name: name,
                subsection_name: None,
            },
        };

        return Ok((i, header));
    }

    // Section header must be using modern subsection syntax at this point.

    let (i, _) = take_spaces(i)?;

    let (i, subsection_name) = delimited(
        char('"'),
        opt(escaped(none_of("\"\\\n\0"), '\\', one_of(r#""\"#))),
        tag("\"]"),
    )(i)?;

    Ok((
        i,
        SectionHeader {
            name: name,
            // We know that there's some section name here, so if we get an
            // empty vec here then we actually parsed an empty section name.
            subsection_name: subsection_name.or(Some("")),
        },
    ))
}

fn take_spaces<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    take_while(|c: char| c.is_ascii() && is_space(c as u8))(i)
}

fn section_body<'a>(i: &'a str) -> IResult<&'a str, (&'a str, Vec<Event<'a>>)> {
    let i = i.trim_start();
    // maybe need to check for [ here
    let (i, name) = config_name(i)?;
    let (i, _) = take_spaces(i)?;
    let (i, value) = config_value(i)?;

    Ok((i, (name, value)))
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
        let (i, _) = take_spaces(i)?;
        value_impl(i)
    } else {
        Ok((
            i,
            vec![Event::Value(Value::Boolean(Boolean::True(
                TrueVariant::Implicit,
            )))],
        ))
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

    let remainder_value = &i[offset..parsed_index].trim_end();
    if partial_value_found {
        events.push(Event::ValueDone(remainder_value));
    } else {
        events.push(Event::Value(Value::from_str(remainder_value)));
    }

    Ok((&i[parsed_index..], events))
}

fn is_char_newline(c: char) -> bool {
    c.is_ascii() && is_newline(c as u8)
}

#[cfg(test)]
mod parse {
    use super::*;

    fn fully_consumed<T>(t: T) -> (&'static str, T) {
        ("", t)
    }

    mod comments {
        use super::super::*;
        use super::*;

        #[test]
        fn semicolon() {
            assert_eq!(
                comment("; this is a semicolon comment").unwrap(),
                fully_consumed(Comment {
                    comment_tag: ';',
                    comment: " this is a semicolon comment",
                })
            );
        }

        #[test]
        fn octothorpe() {
            assert_eq!(
                comment("# this is an octothorpe comment").unwrap(),
                fully_consumed(Comment {
                    comment_tag: '#',
                    comment: " this is an octothorpe comment",
                })
            );
        }

        #[test]
        fn multiple_markers() {
            assert_eq!(
                comment("###### this is an octothorpe comment").unwrap(),
                fully_consumed(Comment {
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
                fully_consumed(SectionHeader {
                    name: "hello",
                    subsection_name: None
                })
            );
        }

        #[test]
        fn modern_subsection() {
            assert_eq!(
                section_header(r#"[hello "world"]"#).unwrap(),
                fully_consumed(SectionHeader {
                    name: "hello",
                    subsection_name: Some("world")
                })
            );
        }

        #[test]
        fn escaped_subsection() {
            assert_eq!(
                section_header(r#"[hello "foo\\bar\""]"#).unwrap(),
                fully_consumed(SectionHeader {
                    name: "hello",
                    subsection_name: Some(r#"foo\\bar\""#)
                })
            );
        }

        #[test]
        fn deprecated_subsection() {
            assert_eq!(
                section_header(r#"[hello.world]"#).unwrap(),
                fully_consumed(SectionHeader {
                    name: "hello",
                    subsection_name: Some("world")
                })
            );
        }

        #[test]
        fn empty_legacy_subsection_name() {
            assert_eq!(
                section_header(r#"[hello.]"#).unwrap(),
                fully_consumed(SectionHeader {
                    name: "hello",
                    subsection_name: Some("")
                })
            );
        }

        #[test]
        fn empty_modern_subsection_name() {
            assert_eq!(
                section_header(r#"[hello ""]"#).unwrap(),
                fully_consumed(SectionHeader {
                    name: "hello",
                    subsection_name: Some("")
                })
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
                fully_consumed(SectionHeader {
                    name: "hello",
                    subsection_name: Some("]")
                })
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
                fully_consumed(vec![Event::Value(Value::Other("hello"))])
            );
        }

        #[test]
        fn no_comment_newline() {
            assert_eq!(
                value_impl("hello\na").unwrap(),
                ("\na", vec![Event::Value(Value::Other("hello"))])
            )
        }

        #[test]
        fn no_comment_is_trimmed() {
            assert_eq!(
                value_impl("hello").unwrap(),
                value_impl("hello               ").unwrap()
            );
        }

        #[test]
        fn semicolon_comment_not_consumed() {
            assert_eq!(
                value_impl("hello;world").unwrap(),
                (";world", vec![Event::Value(Value::Other("hello")),])
            );
        }

        #[test]
        fn octothorpe_comment_not_consumed() {
            assert_eq!(
                value_impl("hello#world").unwrap(),
                ("#world", vec![Event::Value(Value::Other("hello")),])
            );
        }

        #[test]
        fn values_with_comments_are_trimmed() {
            assert_eq!(
                value_impl("hello#world").unwrap(),
                value_impl("hello             #world").unwrap(),
            );
            assert_eq!(
                value_impl("hello;world").unwrap(),
                value_impl("hello             ;world").unwrap(),
            );
        }

        #[test]
        fn trans_escaped_comment_marker_not_consumed() {
            assert_eq!(
                value_impl(r##"hello"#"world; a"##).unwrap(),
                (
                    "; a",
                    vec![Event::Value(Value::Other(r##"hello"#"world"##)),]
                )
            );
        }

        #[test]
        fn complex_test() {
            assert_eq!(
                value_impl(r#"value";";ahhhh"#).unwrap(),
                (";ahhhh", vec![Event::Value(Value::Other(r#"value";""#)),])
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
                    "# \"b\t ; c",
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
        fn simple_section() {
            let section_data = r#"[hello]
            a = b
            c
            d = "lol""#;
            assert_eq!(
                section(section_data).unwrap(),
                fully_consumed(Section {
                    section_header: SectionHeader {
                        name: "hello",
                        subsection_name: None,
                    },
                    items: vec![
                        Event::Key("a"),
                        Event::Value(Value::from_str("b")),
                        Event::Key("c"),
                        Event::Value(Value::Boolean(Boolean::True(TrueVariant::Implicit))),
                        Event::Key("d"),
                        Event::Value(Value::from_str("\"lol\""))
                    ]
                })
            )
        }

        #[test]
        fn section_single_line() {
            assert_eq!(
                section("[hello] c").unwrap(),
                fully_consumed(Section {
                    section_header: SectionHeader {
                        name: "hello",
                        subsection_name: None,
                    },
                    items: vec![
                        Event::Key("c"),
                        Event::Value(Value::Boolean(Boolean::True(TrueVariant::Implicit)))
                    ]
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
                fully_consumed(Section {
                    section_header: SectionHeader {
                        name: "hello",
                        subsection_name: None,
                    },
                    items: vec![
                        Event::Comment(Comment {
                            comment_tag: ';',
                            comment: " commentA",
                        }),
                        Event::Key("a"),
                        Event::Value(Value::from_str("b")),
                        Event::Comment(Comment {
                            comment_tag: '#',
                            comment: " commentB",
                        }),
                        Event::Comment(Comment {
                            comment_tag: ';',
                            comment: " commentC",
                        }),
                        Event::Comment(Comment {
                            comment_tag: ';',
                            comment: " commentD",
                        }),
                        Event::Key("c"),
                        Event::Value(Value::from_str("d")),
                    ]
                })
            );
        }

        #[test]
        fn complex_continuation() {
            // This test is absolute hell. Good luck if this fails.
            assert_eq!(
                section("[section] a = 1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c").unwrap(),
                fully_consumed(Section {
                    section_header: SectionHeader {
                        name: "section",
                        subsection_name: None,
                    },
                    items: vec![
                        Event::Key("a"),
                        Event::ValueNotDone(r#"1    "\""#),
                        Event::Newline("\n"),
                        Event::ValueNotDone(r#"a ; e "\""#),
                        Event::Newline("\n"),
                        Event::ValueDone("d"),
                        Event::Comment(Comment {
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
                fully_consumed(Section {
                    section_header: SectionHeader {
                        name: "section",
                        subsection_name: Some("a")
                    },
                    items: vec![
                        Event::Key("b"),
                        Event::ValueNotDone("\""),
                        Event::Newline("\n"),
                        Event::ValueDone(";\""),
                        Event::Comment(Comment {
                            comment: "a",
                            comment_tag: ';'
                        })
                    ]
                })
            )
        }
    }
}
