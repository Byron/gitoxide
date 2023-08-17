use std::borrow::Cow;

use smallvec::SmallVec;
use winnow::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{all_consuming, opt},
    error::{context, ContextError, ParseError},
    multi::many0,
    IResult, Parser,
};

use crate::{parse, parse::NL, BStr, ByteSlice, CommitRef};

pub fn message<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E> {
    if i.is_empty() {
        // newline + [message]
        return Err(winnow::Err::Backtrack(E::add_context(
            E::from_error_kind(i, winnow::error::ErrorKind::Eof),
            i,
            "newline + <message>",
        )));
    }
    let (i, _) = context("a newline separates headers from the message", tag(NL))(i)?;
    Ok((&[], i.as_bstr()))
}

pub fn commit<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    i: &'a [u8],
) -> IResult<&'a [u8], CommitRef<'_>, E> {
    let (i, tree) = context("tree <40 lowercase hex char>", |i| {
        parse::header_field(i, b"tree", parse::hex_hash)
    })(i)?;
    let (i, parents): (_, Vec<_>) = context(
        "zero or more 'parent <40 lowercase hex char>'",
        many0(|i| parse::header_field(i, b"parent", parse::hex_hash)),
    )(i)?;
    let (i, author) = context("author <signature>", |i| {
        parse::header_field(i, b"author", parse::signature)
    })(i)?;
    let (i, committer) = context("committer <signature>", |i| {
        parse::header_field(i, b"committer", parse::signature)
    })(i)?;
    let (i, encoding) = context(
        "encoding <encoding>",
        opt(|i| parse::header_field(i, b"encoding", is_not(NL))),
    )(i)?;
    let (i, extra_headers) = context(
        "<field> <single-line|multi-line>",
        many0(alt((
            parse::any_header_field_multi_line.map(|(k, o)| (k.as_bstr(), Cow::Owned(o))),
            |i| {
                parse::any_header_field(i, is_not(NL)).map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Borrowed(o.as_bstr()))))
            },
        ))),
    )(i)?;
    let (i, message) = all_consuming(message)(i)?;

    Ok((
        i,
        CommitRef {
            tree,
            parents: SmallVec::from(parents),
            author,
            committer,
            encoding: encoding.map(ByteSlice::as_bstr),
            message,
            extra_headers,
        },
    ))
}
