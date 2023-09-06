use std::borrow::Cow;

use bstr::{BStr, ByteSlice};
use winnow::{
    combinator::{alt, delimited, fold_repeat, opt, preceded, repeat},
    error::{ErrorKind, InputError as NomError, ParserError as _},
    prelude::*,
    stream::{Offset as _, Stream as _},
    token::{one_of, take_till0, take_while},
};

use crate::parse::{error::ParseNode, section, Comment, Error, Event};

/// Attempt to zero-copy parse the provided bytes, passing results to `dispatch`.
pub fn from_bytes<'i>(mut input: &'i [u8], dispatch: &mut dyn FnMut(Event<'i>)) -> Result<(), Error> {
    let start = input.checkpoint();

    let bom = unicode_bom::Bom::from(input);
    input.next_slice(bom.len());

    fold_repeat(
        0..,
        alt((
            comment.map(Event::Comment),
            take_spaces1.map(|whitespace| Event::Whitespace(Cow::Borrowed(whitespace))),
            |i: &mut &'i [u8]| {
                let newline = take_newlines1.parse_next(i)?;
                let o = Event::Newline(Cow::Borrowed(newline));
                Ok(o)
            },
        )),
        || (),
        |_acc, event| dispatch(event),
    )
    .parse_next(&mut input)
    // I don't think this can panic. many0 errors if the child parser returns
    // a success where the input was not consumed, but alt will only return Ok
    // if one of its children succeed. However, all of it's children are
    // guaranteed to consume something if they succeed, so the Ok(i) == i case
    // can never occur.
    .expect("many0(alt(...)) panicked. Likely a bug in one of the children parsers.");

    if input.is_empty() {
        return Ok(());
    }

    let mut node = ParseNode::SectionHeader;

    let res = repeat(1.., |i: &mut &'i [u8]| section(i, &mut node, dispatch))
        .map(|()| ())
        .parse_next(&mut input);
    res.map_err(|_| {
        let newlines = newlines_from(input, start);
        Error {
            line_number: newlines,
            last_attempted_parser: node,
            parsed_until: input.as_bstr().into(),
        }
    })?;

    // This needs to happen after we collect sections, otherwise the line number
    // will be off.
    if !input.is_empty() {
        let newlines = newlines_from(input, start);
        return Err(Error {
            line_number: newlines,
            last_attempted_parser: node,
            parsed_until: input.as_bstr().into(),
        });
    }

    Ok(())
}

fn newlines_from(input: &[u8], start: winnow::stream::Checkpoint<&[u8]>) -> usize {
    let offset = input.offset_from(&start);
    let mut start_input = input;
    start_input.reset(start);
    start_input.next_slice(offset).iter().filter(|c| **c == b'\n').count()
}

fn comment<'i>(i: &mut &'i [u8]) -> PResult<Comment<'i>, NomError<&'i [u8]>> {
    (
        one_of([';', '#']),
        take_till0(|c| c == b'\n').map(|text: &[u8]| Cow::Borrowed(text.as_bstr())),
    )
        .map(|(tag, text)| Comment { tag, text })
        .parse_next(i)
}

#[cfg(test)]
mod tests;

fn section<'i>(
    i: &mut &'i [u8],
    node: &mut ParseNode,
    dispatch: &mut dyn FnMut(Event<'i>),
) -> PResult<(), NomError<&'i [u8]>> {
    let start = i.checkpoint();
    let header = section_header(i).map_err(|e| {
        i.reset(start);
        e
    })?;
    dispatch(Event::SectionHeader(header));

    // This would usually be a many0(alt(...)), the manual loop allows us to
    // optimize vec insertions
    loop {
        let start = i.checkpoint();

        if let Some(v) = opt(take_spaces1).parse_next(i)? {
            dispatch(Event::Whitespace(Cow::Borrowed(v.as_bstr())));
        }

        if let Some(v) = opt(take_newlines1).parse_next(i)? {
            dispatch(Event::Newline(Cow::Borrowed(v.as_bstr())));
        }

        key_value_pair(i, node, dispatch)?;

        if let Some(comment) = opt(comment).parse_next(i)? {
            dispatch(Event::Comment(comment));
        }

        if i.offset_from(&start) == 0 {
            break;
        }
    }

    Ok(())
}

fn section_header<'i>(i: &mut &'i [u8]) -> PResult<section::Header<'i>, NomError<&'i [u8]>> {
    // No spaces must be between section name and section start
    let name = preceded('[', take_while(1.., is_section_char).map(bstr::ByteSlice::as_bstr)).parse_next(i)?;

    if opt(one_of::<_, _, NomError<&[u8]>>(']')).parse_next(i)?.is_some() {
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
            return Err(winnow::error::ErrMode::from_error_kind(i, ErrorKind::Fail));
        }
        return Ok(header);
    }

    // Section header must be using modern subsection syntax at this point.
    (take_spaces1, delimited('"', opt(sub_section), "\"]"))
        .map(|(whitespace, subsection_name)| section::Header {
            name: section::Name(Cow::Borrowed(name)),
            separator: Some(Cow::Borrowed(whitespace)),
            subsection_name,
        })
        .parse_next(i)
}

fn is_section_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-' || c == b'.'
}

fn sub_section<'i>(i: &mut &'i [u8]) -> PResult<Cow<'i, BStr>, NomError<&'i [u8]>> {
    let mut output = Cow::Borrowed(Default::default());
    if let Some(sub) = opt(subsection_subset).parse_next(i)? {
        output = Cow::Borrowed(sub.as_bstr());
    }
    while let Some(sub) = opt(subsection_subset).parse_next(i)? {
        output.to_mut().extend(sub);
    }

    Ok(output)
}

fn subsection_subset<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8], NomError<&'i [u8]>> {
    alt((subsection_unescaped, subsection_escaped_char)).parse_next(i)
}

fn subsection_unescaped<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8], NomError<&'i [u8]>> {
    take_while(1.., is_subsection_unescaped_char).parse_next(i)
}

fn subsection_escaped_char<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8], NomError<&'i [u8]>> {
    preceded('\\', one_of(is_subsection_escapeable_char).recognize()).parse_next(i)
}

fn is_subsection_escapeable_char(c: u8) -> bool {
    c != b'\n'
}

fn is_subsection_unescaped_char(c: u8) -> bool {
    c != b'"' && c != b'\\' && c != b'\n' && c != 0
}

fn key_value_pair<'i>(
    i: &mut &'i [u8],
    node: &mut ParseNode,
    dispatch: &mut dyn FnMut(Event<'i>),
) -> PResult<(), NomError<&'i [u8]>> {
    *node = ParseNode::Name;
    if let Some(name) = opt(config_name).parse_next(i)? {
        dispatch(Event::SectionKey(section::Key(Cow::Borrowed(name))));

        if let Some(whitespace) = opt(take_spaces1).parse_next(i)? {
            dispatch(Event::Whitespace(Cow::Borrowed(whitespace)));
        }

        *node = ParseNode::Value;
        config_value(i, dispatch)
    } else {
        Ok(())
    }
}

/// Parses the config name of a config pair. Assumes the input has already been
/// trimmed of any leading whitespace.
fn config_name<'i>(i: &mut &'i [u8]) -> PResult<&'i BStr, NomError<&'i [u8]>> {
    (
        one_of(|c: u8| c.is_ascii_alphabetic()),
        take_while(0.., |c: u8| c.is_ascii_alphanumeric() || c == b'-'),
    )
        .recognize()
        .map(bstr::ByteSlice::as_bstr)
        .parse_next(i)
}

fn config_value<'i>(i: &mut &'i [u8], dispatch: &mut dyn FnMut(Event<'i>)) -> PResult<(), NomError<&'i [u8]>> {
    if opt('=').parse_next(i)?.is_some() {
        dispatch(Event::KeyValueSeparator);
        if let Some(whitespace) = opt(take_spaces1).parse_next(i)? {
            dispatch(Event::Whitespace(Cow::Borrowed(whitespace)));
        }
        value_impl(i, dispatch)
    } else {
        // This is a special way of denoting 'empty' values which a lot of code depends on.
        // Hence, rather to fix this everywhere else, leave it here and fix it where it matters, namely
        // when it's about differentiating between a missing key-value separator, and one followed by emptiness.
        dispatch(Event::Value(Cow::Borrowed("".into())));
        Ok(())
    }
}

/// Handles parsing of known-to-be values. This function handles both single
/// line values as well as values that are continuations.
fn value_impl<'i>(i: &mut &'i [u8], dispatch: &mut dyn FnMut(Event<'i>)) -> PResult<(), NomError<&'i [u8]>> {
    let start_checkpoint = i.checkpoint();
    let mut value_start_checkpoint = i.checkpoint();
    let mut value_end = None;

    // This is required to ignore comment markers if they're in a quote.
    let mut is_in_quotes = false;
    // Used to determine if we return a Value or Value{Not,}Done
    let mut partial_value_found = false;

    loop {
        let _ = take_while(0.., |c| !matches!(c, b'\n' | b'\\' | b'"' | b';' | b'#')).parse_next(i)?;
        if let Some(c) = i.next_token() {
            match c {
                b'\n' => {
                    value_end = Some(i.offset_from(&value_start_checkpoint) - 1);
                    break;
                }
                b';' | b'#' if !is_in_quotes => {
                    value_end = Some(i.offset_from(&value_start_checkpoint) - 1);
                    break;
                }
                b'\\' => {
                    let escaped_index = i.offset_from(&value_start_checkpoint);
                    let escape_index = escaped_index - 1;
                    let Some(mut c) = i.next_token() else {
                        i.reset(start_checkpoint);
                        return Err(winnow::error::ErrMode::from_error_kind(i, ErrorKind::Token));
                    };
                    let mut consumed = 1;
                    if c == b'\r' {
                        c = i.next_token().ok_or_else(|| {
                            i.reset(start_checkpoint);
                            winnow::error::ErrMode::from_error_kind(i, ErrorKind::Token)
                        })?;
                        if c != b'\n' {
                            i.reset(start_checkpoint);
                            return Err(winnow::error::ErrMode::from_error_kind(i, ErrorKind::Slice));
                        }
                        consumed += 1;
                    }

                    match c {
                        b'\n' => {
                            partial_value_found = true;

                            i.reset(value_start_checkpoint);

                            let value = i.next_slice(escape_index).as_bstr();
                            dispatch(Event::ValueNotDone(Cow::Borrowed(value)));

                            i.next_token();

                            let nl = i.next_slice(consumed).as_bstr();
                            dispatch(Event::Newline(Cow::Borrowed(nl)));

                            value_start_checkpoint = i.checkpoint();
                            value_end = None;
                        }
                        b'n' | b't' | b'\\' | b'b' | b'"' => {}
                        _ => {
                            i.reset(start_checkpoint);
                            return Err(winnow::error::ErrMode::from_error_kind(i, ErrorKind::Token));
                        }
                    }
                }
                b'"' => is_in_quotes = !is_in_quotes,
                _ => {}
            }
        } else {
            break;
        }
    }
    if is_in_quotes {
        i.reset(start_checkpoint);
        return Err(winnow::error::ErrMode::from_error_kind(i, ErrorKind::Slice));
    }

    let value_end = match value_end {
        None => {
            let last_value_index = i.offset_from(&value_start_checkpoint);
            if last_value_index == 0 {
                dispatch(Event::Value(Cow::Borrowed("".into())));
                return Ok(());
            } else {
                last_value_index
            }
        }
        Some(idx) => idx,
    };

    i.reset(value_start_checkpoint);
    let value_end_no_trailing_whitespace = i[..value_end]
        .iter()
        .enumerate()
        .rev()
        .find_map(|(idx, b)| (!b.is_ascii_whitespace()).then_some(idx + 1))
        .unwrap_or(0);
    let remainder_value = i.next_slice(value_end_no_trailing_whitespace);

    if partial_value_found {
        dispatch(Event::ValueDone(Cow::Borrowed(remainder_value.as_bstr())));
    } else {
        dispatch(Event::Value(Cow::Borrowed(remainder_value.as_bstr())));
    }

    Ok(())
}

fn take_spaces1<'i>(i: &mut &'i [u8]) -> PResult<&'i BStr, NomError<&'i [u8]>> {
    take_while(1.., winnow::stream::AsChar::is_space)
        .map(bstr::ByteSlice::as_bstr)
        .parse_next(i)
}

fn take_newlines1<'i>(i: &mut &'i [u8]) -> PResult<&'i BStr, NomError<&'i [u8]>> {
    repeat(1.., alt(("\r\n", "\n")))
        .map(|()| ())
        .recognize()
        .map(bstr::ByteSlice::as_bstr)
        .parse_next(i)
}
