use std::borrow::Cow;

use bstr::{BStr, BString, ByteSlice, ByteVec};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::{
        complete::{char, one_of},
        is_space,
    },
    combinator::{map, opt},
    error::{Error as NomError, ErrorKind},
    multi::{fold_many0, fold_many1},
    sequence::delimited,
    IResult,
};

use crate::parse::{error::ParseNode, section, Comment, Error, Event};

/// Attempt to zero-copy parse the provided bytes, passing results to `dispatch`.
pub fn from_bytes<'a>(input: &'a [u8], mut dispatch: impl FnMut(Event<'a>)) -> Result<(), Error> {
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
        |_acc, event| dispatch(event),
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
        |i| section(i, &mut node, &mut dispatch),
        || (),
        |_acc, additional_newlines| {
            newlines += additional_newlines;
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
            tag: comment_tag as u8,
            text: Cow::Borrowed(comment.as_bstr()),
        },
    ))
}

#[cfg(test)]
mod tests;

fn section<'a>(i: &'a [u8], node: &mut ParseNode, dispatch: &mut impl FnMut(Event<'a>)) -> IResult<&'a [u8], usize> {
    let (mut i, header) = section_header(i)?;
    dispatch(Event::SectionHeader(header));

    let mut newlines = 0;

    // This would usually be a many0(alt(...)), the manual loop allows us to
    // optimize vec insertions
    loop {
        let old_i = i;

        if let Ok((new_i, v)) = take_spaces(i) {
            if old_i != new_i {
                i = new_i;
                dispatch(Event::Whitespace(Cow::Borrowed(v.as_bstr())));
            }
        }

        if let Ok((new_i, (v, new_newlines))) = take_newlines(i) {
            if old_i != new_i {
                i = new_i;
                newlines += new_newlines;
                dispatch(Event::Newline(Cow::Borrowed(v.as_bstr())));
            }
        }

        if let Ok((new_i, new_newlines)) = key_value_pair(i, node, dispatch) {
            if old_i != new_i {
                i = new_i;
                newlines += new_newlines;
            }
        }

        if let Ok((new_i, comment)) = comment(i) {
            if old_i != new_i {
                i = new_i;
                dispatch(Event::Comment(comment));
            }
        }

        if old_i == i {
            break;
        }
    }

    Ok((i, newlines))
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

        if header.name.is_empty() {
            return Err(nom::Err::Error(NomError {
                input: i,
                code: ErrorKind::NoneOf,
            }));
        }
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
            subsection_name,
        },
    ))
}

fn sub_section(i: &[u8]) -> IResult<&[u8], Cow<'_, BStr>> {
    let (rest, (found_escape, consumed)) = sub_section_delegate(i, &mut |_| ())?;
    if found_escape {
        let mut buf = BString::default();
        sub_section_delegate(i, &mut |b| buf.push_byte(b)).map(|(i, _)| (i, buf.into()))
    } else {
        Ok((rest, i[..consumed].as_bstr().into()))
    }
}

fn sub_section_delegate<'a>(i: &'a [u8], push_byte: &mut dyn FnMut(u8)) -> IResult<&'a [u8], (bool, usize)> {
    let mut cursor = 0;
    let mut bytes = i.iter().copied();
    let mut found_terminator = false;
    let mut found_escape = false;
    while let Some(mut b) = bytes.next() {
        cursor += 1;
        if b == b'\n' || b == 0 {
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
            found_escape = true;
            cursor += 1;
            if b == b'\n' {
                return Err(nom::Err::Error(NomError {
                    input: &i[cursor..],
                    code: ErrorKind::NonEmpty,
                }));
            }
        }
        push_byte(b);
    }

    if !found_terminator {
        return Err(nom::Err::Error(NomError {
            input: &i[cursor..],
            code: ErrorKind::NonEmpty,
        }));
    }

    Ok((&i[cursor - 1..], (found_escape, cursor - 1)))
}

fn key_value_pair<'a>(
    i: &'a [u8],
    node: &mut ParseNode,
    dispatch: &mut impl FnMut(Event<'a>),
) -> IResult<&'a [u8], usize> {
    *node = ParseNode::Name;
    let (i, name) = config_name(i)?;

    dispatch(Event::SectionKey(section::Key(Cow::Borrowed(name))));

    let (i, whitespace) = opt(take_spaces)(i)?;
    if let Some(whitespace) = whitespace {
        dispatch(Event::Whitespace(Cow::Borrowed(whitespace)));
    }

    *node = ParseNode::Value;
    let (i, newlines) = config_value(i, dispatch)?;
    Ok((i, newlines))
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

    let (i, name) = take_while(|c: u8| c.is_ascii_alphanumeric() || c == b'-')(i)?;
    Ok((i, name.as_bstr()))
}

fn config_value<'a>(i: &'a [u8], dispatch: &mut impl FnMut(Event<'a>)) -> IResult<&'a [u8], usize> {
    if let (i, Some(_)) = opt(char('='))(i)? {
        dispatch(Event::KeyValueSeparator);
        let (i, whitespace) = opt(take_spaces)(i)?;
        if let Some(whitespace) = whitespace {
            dispatch(Event::Whitespace(Cow::Borrowed(whitespace)));
        }
        let (i, newlines) = value_impl(i, dispatch)?;
        Ok((i, newlines))
    } else {
        // This is a special way of denoting 'empty' values which a lot of code depends on.
        // Hence, rather to fix this everywhere else, leave it here and fix it where it matters, namely
        // when it's about differentiating between a missing key-value separator, and one followed by emptiness.
        dispatch(Event::Value(Cow::Borrowed("".into())));
        Ok((i, 0))
    }
}

/// Handles parsing of known-to-be values. This function handles both single
/// line values as well as values that are continuations.
fn value_impl<'a>(i: &'a [u8], dispatch: &mut impl FnMut(Event<'a>)) -> IResult<&'a [u8], usize> {
    let (i, value_end, newlines, mut dispatch) = {
        let new_err = |code| nom::Err::Error(NomError { input: i, code });
        let mut value_end = None::<usize>;
        let mut value_start: usize = 0;
        let mut newlines = 0;

        let mut prev_char_was_backslash = false;
        // This is required to ignore comment markers if they're in a quote.
        let mut is_in_quotes = false;
        // Used to determine if we return a Value or Value{Not,}Done
        let mut partial_value_found = false;
        let mut last_value_index: usize = 0;

        let mut bytes = i.iter();
        while let Some(mut c) = bytes.next() {
            if prev_char_was_backslash {
                prev_char_was_backslash = false;
                let mut consumed = 1;
                if *c == b'\r' {
                    c = bytes.next().ok_or_else(|| new_err(ErrorKind::Escaped))?;
                    if *c != b'\n' {
                        return Err(new_err(ErrorKind::Tag));
                    }
                    consumed += 1;
                }

                match c {
                    b'\n' => {
                        partial_value_found = true;
                        let backslash = 1;
                        dispatch(Event::ValueNotDone(Cow::Borrowed(
                            i[value_start..last_value_index - backslash].as_bstr(),
                        )));
                        let nl_end = last_value_index + consumed;
                        dispatch(Event::Newline(Cow::Borrowed(i[last_value_index..nl_end].as_bstr())));
                        value_start = nl_end;
                        value_end = None;
                        newlines += 1;

                        last_value_index += consumed;
                    }
                    b'n' | b't' | b'\\' | b'b' | b'"' => {
                        last_value_index += 1;
                    }
                    _ => {
                        return Err(new_err(ErrorKind::Escaped));
                    }
                }
            } else {
                match c {
                    b'\n' => {
                        value_end = last_value_index.into();
                        break;
                    }
                    b';' | b'#' if !is_in_quotes => {
                        value_end = last_value_index.into();
                        break;
                    }
                    b'\\' => prev_char_was_backslash = true,
                    b'"' => is_in_quotes = !is_in_quotes,
                    _ => {}
                }
                last_value_index += 1;
            }
        }

        if prev_char_was_backslash {
            return Err(new_err(ErrorKind::Escaped));
        }

        if is_in_quotes {
            return Err(new_err(ErrorKind::Tag));
        }

        let value_end = match value_end {
            None => {
                if last_value_index == 0 {
                    dispatch(Event::Value(Cow::Borrowed("".into())));
                    return Ok((&i[0..], newlines));
                } else {
                    i.len()
                }
            }
            Some(idx) => idx,
        };

        let dispatch = move |value: &'a [u8]| {
            if partial_value_found {
                dispatch(Event::ValueDone(Cow::Borrowed(value.as_bstr())));
            } else {
                dispatch(Event::Value(Cow::Borrowed(value.as_bstr())));
            }
        };
        (&i[value_start..], value_end - value_start, newlines, dispatch)
    };

    let (i, remainder_value) = {
        let value_end_no_trailing_whitespace = i[..value_end]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(idx, b)| (!b.is_ascii_whitespace()).then_some(idx + 1))
            .unwrap_or(0);
        (
            &i[value_end_no_trailing_whitespace..],
            &i[..value_end_no_trailing_whitespace],
        )
    };

    dispatch(remainder_value);

    Ok((i, newlines))
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
