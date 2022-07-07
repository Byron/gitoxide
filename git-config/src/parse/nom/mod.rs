use crate::parse::{section, Comment, Error, Event, Section};
use bstr::{BStr, BString, ByteSlice, ByteVec};
use std::borrow::Cow;

use crate::parse::error::ParseNode;
use nom::multi::{fold_many0, fold_many1};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::{
        complete::{char, one_of},
        is_space,
    },
    combinator::{map, opt},
    error::{Error as NomError, ErrorKind},
    sequence::delimited,
    IResult,
};
use smallvec::SmallVec;

/// Attempt to zero-copy parse the provided bytes, passing results to `receive_event`.
///
/// # Errors
///
/// Returns an error if the string provided is not a valid `git-config`.
/// This generally is due to either invalid names or if there's extraneous
/// data succeeding valid `git-config` data.
pub fn from_bytes<'a>(input: &'a [u8], mut receive_event: impl FnMut(Event<'a>)) -> Result<(), Error> {
    let bom = unicode_bom::Bom::from(input);
    let mut newlines = 0;
    let (i, _) = fold_many0(
        alt((
            map(comment, Event::Comment),
            map(take_spaces, |whitespace| Event::Whitespace(Cow::Borrowed(whitespace))),
            map(take_newlines, |(newline, counter)| {
                newlines += counter;
                Event::Newline(Cow::Borrowed(newline))
            }),
        )),
        || (),
        |_acc, event| receive_event(event),
    )(&input[bom.len()..])
    // I don't think this can panic. many0 errors if the child parser returns
    // a success where the input was not consumed, but alt will only return Ok
    // if one of its children succeed. However, all of it's children are
    // guaranteed to consume something if they succeed, so the Ok(i) == i case
    // can never occur.
    .expect("many0(alt(...)) panicked. Likely a bug in one of the children parsers.");

    if i.is_empty() {
        return Ok(());
    }

    let mut node = ParseNode::SectionHeader;

    let res = fold_many1(
        |i| section(i, &mut node),
        || (),
        |_acc, (section, additional_newlines)| {
            newlines += additional_newlines;
            receive_event(Event::SectionHeader(section.section_header));
            for event in section.events {
                receive_event(event);
            }
        },
    )(i);
    let (i, _) = res.map_err(|_| Error {
        line_number: newlines,
        last_attempted_parser: node,
        parsed_until: i.as_bstr().into(),
    })?;

    // This needs to happen after we collect sections, otherwise the line number
    // will be off.
    if !i.is_empty() {
        return Err(Error {
            line_number: newlines,
            last_attempted_parser: node,
            parsed_until: i.as_bstr().into(),
        });
    }

    Ok(())
}

fn comment(i: &[u8]) -> IResult<&[u8], Comment<'_>> {
    let (i, comment_tag) = one_of(";#")(i)?;
    let (i, comment) = take_till(|c| c == b'\n')(i)?;
    Ok((
        i,
        Comment {
            comment_tag: comment_tag as u8,
            comment: Cow::Borrowed(comment.as_bstr()),
        },
    ))
}

#[cfg(test)]
mod tests;

fn section<'a, 'b>(i: &'a [u8], node: &'b mut ParseNode) -> IResult<&'a [u8], (Section<'a>, usize)> {
    let (mut i, section_header) = section_header(i)?;

    let mut newlines = 0;
    let mut items = SmallVec::default();

    // This would usually be a many0(alt(...)), the manual loop allows us to
    // optimize vec insertions
    loop {
        let old_i = i;

        if let Ok((new_i, v)) = take_spaces(i) {
            if old_i != new_i {
                i = new_i;
                items.push(Event::Whitespace(Cow::Borrowed(v.as_bstr())));
            }
        }

        if let Ok((new_i, (v, new_newlines))) = take_newlines(i) {
            if old_i != new_i {
                i = new_i;
                newlines += new_newlines;
                items.push(Event::Newline(Cow::Borrowed(v.as_bstr())));
            }
        }

        if let Ok((new_i, _)) = section_body(i, node, &mut items) {
            if old_i != new_i {
                i = new_i;
            }
        }

        if let Ok((new_i, comment)) = comment(i) {
            if old_i != new_i {
                i = new_i;
                items.push(Event::Comment(comment));
            }
        }

        if old_i == i {
            break;
        }
    }

    Ok((
        i,
        (
            Section {
                section_header,
                events: items,
            },
            newlines,
        ),
    ))
}

fn section_header(i: &[u8]) -> IResult<&[u8], section::Header<'_>> {
    let (i, _) = char('[')(i)?;
    // No spaces must be between section name and section start
    let (i, name) = take_while(|c: u8| c.is_ascii_alphanumeric() || c == b'-' || c == b'.')(i)?;

    let name = name.as_bstr();
    if let Ok((i, _)) = char::<_, NomError<&[u8]>>(']')(i) {
        // Either section does not have a subsection or using deprecated
        // subsection syntax at this point.
        let header = match memchr::memrchr(b'.', name.as_bytes()) {
            Some(index) => section::Header {
                name: section::Name(Cow::Borrowed(name[..index].as_bstr())),
                separator: name.get(index..=index).map(|s| Cow::Borrowed(s.as_bstr())),
                subsection_name: name.get(index + 1..).map(|s| Cow::Borrowed(s.as_bstr())),
            },
            None => section::Header {
                name: section::Name(Cow::Borrowed(name.as_bstr())),
                separator: None,
                subsection_name: None,
            },
        };

        return Ok((i, header));
    }

    // Section header must be using modern subsection syntax at this point.

    let (i, whitespace) = take_spaces(i)?;
    let (i, subsection_name) = delimited(char('"'), opt(sub_section), tag("\"]"))(i)?;

    Ok((
        i,
        section::Header {
            name: section::Name(Cow::Borrowed(name)),
            separator: Some(Cow::Borrowed(whitespace)),
            subsection_name: subsection_name.map(Cow::Owned),
        },
    ))
}

fn sub_section(i: &[u8]) -> IResult<&[u8], BString> {
    let mut cursor = 0;
    let mut bytes = i.iter().copied();
    let mut found_terminator = false;
    let mut buf = BString::default();
    while let Some(mut b) = bytes.next() {
        cursor += 1;
        if b == b'\n' {
            return Err(nom::Err::Error(NomError {
                input: &i[cursor..],
                code: ErrorKind::NonEmpty,
            }));
        }
        if b == b'"' {
            found_terminator = true;
            break;
        }
        if b == b'\\' {
            b = bytes.next().ok_or_else(|| {
                nom::Err::Error(NomError {
                    input: &i[cursor..],
                    code: ErrorKind::NonEmpty,
                })
            })?;
            cursor += 1;
            if b == b'\n' {
                return Err(nom::Err::Error(NomError {
                    input: &i[cursor..],
                    code: ErrorKind::NonEmpty,
                }));
            }
        }
        buf.push_byte(b);
    }

    if !found_terminator {
        return Err(nom::Err::Error(NomError {
            input: &i[cursor..],
            code: ErrorKind::NonEmpty,
        }));
    }

    Ok((&i[cursor - 1..], buf))
}

fn section_body<'a, 'b, 'c>(
    i: &'a [u8],
    node: &'b mut ParseNode,
    items: &'c mut section::Events<'a>,
) -> IResult<&'a [u8], ()> {
    // maybe need to check for [ here
    *node = ParseNode::ConfigName;
    let (i, name) = config_name(i)?;

    items.push(Event::SectionKey(section::Key(Cow::Borrowed(name))));

    let (i, whitespace) = opt(take_spaces)(i)?;

    if let Some(whitespace) = whitespace {
        items.push(Event::Whitespace(Cow::Borrowed(whitespace)));
    }

    let (i, _) = config_value(i, items)?;
    Ok((i, ()))
}

/// Parses the config name of a config pair. Assumes the input has already been
/// trimmed of any leading whitespace.
fn config_name(i: &[u8]) -> IResult<&[u8], &BStr> {
    if i.is_empty() {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::NonEmpty,
        }));
    }

    if !i[0].is_ascii_alphabetic() {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Alpha,
        }));
    }

    let (i, v) = take_while(|c: u8| c.is_ascii_alphanumeric() || c == b'-')(i)?;
    Ok((i, v.as_bstr()))
}

fn config_value<'a, 'b>(i: &'a [u8], events: &'b mut section::Events<'a>) -> IResult<&'a [u8], ()> {
    if let (i, Some(_)) = opt(char('='))(i)? {
        events.push(Event::KeyValueSeparator);
        let (i, whitespace) = opt(take_spaces)(i)?;
        if let Some(whitespace) = whitespace {
            events.push(Event::Whitespace(Cow::Borrowed(whitespace)));
        }
        let (i, _) = value_impl(i, events)?;
        Ok((i, ()))
    } else {
        events.push(Event::Value(Cow::Borrowed("".into())));
        Ok((i, ()))
    }
}

/// Handles parsing of known-to-be values. This function handles both single
/// line values as well as values that are continuations.
///
/// # Errors
///
/// Returns an error if an invalid escape was used, if there was an unfinished
/// quote, or there was an escape but there is nothing left to escape.
fn value_impl<'a, 'b>(i: &'a [u8], events: &'b mut section::Events<'a>) -> IResult<&'a [u8], ()> {
    let mut parsed_index: usize = 0;
    let mut offset: usize = 0;

    let mut was_prev_char_escape_char = false;
    // This is required to ignore comment markers if they're in a quote.
    let mut is_in_quotes = false;
    // Used to determine if we return a Value or Value{Not,}Done
    let mut partial_value_found = false;
    let mut index: usize = 0;

    for c in i.iter() {
        if was_prev_char_escape_char {
            was_prev_char_escape_char = false;
            match c {
                // We're escaping a newline, which means we've found a
                // continuation.
                b'\n' => {
                    partial_value_found = true;
                    events.push(Event::ValueNotDone(Cow::Borrowed(i[offset..index - 1].as_bstr())));
                    events.push(Event::Newline(Cow::Borrowed(i[index..=index].as_bstr())));
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
        index += 1;
    }

    if parsed_index == 0 {
        if index != 0 {
            parsed_index = i.len();
        } else {
            // Didn't parse anything at all, newline straight away.
            events.push(Event::Value(Cow::Owned(BString::default())));
            events.push(Event::Newline(Cow::Borrowed("\n".into())));
            return Ok((
                i.get(1..).ok_or(nom::Err::Error(NomError {
                    input: i,
                    code: ErrorKind::Eof,
                }))?,
                (),
            ));
        }
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
            if !i[index].is_ascii_whitespace() {
                new_index = index + 1;
                break;
            }
        }
        (&i[new_index..], &i[offset..new_index])
    };

    if partial_value_found {
        events.push(Event::ValueDone(Cow::Borrowed(remainder_value.as_bstr())));
    } else {
        events.push(Event::Value(Cow::Borrowed(remainder_value.as_bstr())));
    }

    Ok((i, ()))
}

fn take_spaces(i: &[u8]) -> IResult<&[u8], &BStr> {
    let (i, v) = take_while(|c: u8| c.is_ascii() && is_space(c))(i)?;
    if v.is_empty() {
        Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Eof,
        }))
    } else {
        Ok((i, v.as_bstr()))
    }
}

fn take_newlines(i: &[u8]) -> IResult<&[u8], (&BStr, usize)> {
    let mut counter = 0;
    let mut consumed_bytes = 0;
    let mut next_must_be_newline = false;
    for b in i.iter().copied() {
        if !b.is_ascii() {
            break;
        };
        if b == b'\r' {
            if next_must_be_newline {
                break;
            }
            next_must_be_newline = true;
            continue;
        };
        if b == b'\n' {
            counter += 1;
            consumed_bytes += if next_must_be_newline { 2 } else { 1 };
            next_must_be_newline = false;
        } else {
            break;
        }
    }
    let (v, i) = i.split_at(consumed_bytes);
    if v.is_empty() {
        Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Eof,
        }))
    } else {
        Ok((i, (v.as_bstr(), counter)))
    }
}
