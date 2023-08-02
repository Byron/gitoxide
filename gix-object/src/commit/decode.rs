use std::borrow::Cow;

use smallvec::SmallVec;
use winnow::{
    combinator::alt,
    combinator::preceded,
    combinator::repeat,
    combinator::terminated,
    combinator::{eof, opt, rest},
    error::{AddContext, ParserError},
    prelude::*,
    token::take_till1,
};

use crate::{parse, parse::NL, BStr, ByteSlice, CommitRef};

pub fn message<'a, E: ParserError<&'a [u8]> + AddContext<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E> {
    if i.is_empty() {
        // newline + [message]
        return Err(
            winnow::error::ErrMode::from_error_kind(i, winnow::error::ErrorKind::Eof)
                .map(|err: E| err.add_context(i, "newline + <message>")),
        );
    }
    preceded(NL, rest.map(ByteSlice::as_bstr))
        .context("a newline separates headers from the message")
        .parse_next(i)
}

pub fn commit<'a, E: ParserError<&'a [u8]> + AddContext<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], CommitRef<'a>, E> {
    (
        (|i| parse::header_field(i, b"tree", parse::hex_hash)).context("tree <40 lowercase hex char>"),
        repeat(0.., |i| parse::header_field(i, b"parent", parse::hex_hash))
            .map(|p: Vec<_>| p)
            .context("zero or more 'parent <40 lowercase hex char>'"),
        (|i| parse::header_field(i, b"author", parse::signature)).context("author <signature>"),
        (|i| parse::header_field(i, b"committer", parse::signature)).context("committer <signature>"),
        opt(|i| parse::header_field(i, b"encoding", take_till1(NL))).context("encoding <encoding>"),
        repeat(
            0..,
            alt((
                parse::any_header_field_multi_line.map(|(k, o)| (k.as_bstr(), Cow::Owned(o))),
                |i| {
                    parse::any_header_field(i, take_till1(NL))
                        .map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Borrowed(o.as_bstr()))))
                },
            )),
        )
        .context("<field> <single-line|multi-line>"),
        terminated(message, eof),
    )
        .map(
            |(tree, parents, author, committer, encoding, extra_headers, message)| CommitRef {
                tree,
                parents: SmallVec::from(parents),
                author,
                committer,
                encoding: encoding.map(ByteSlice::as_bstr),
                message,
                extra_headers,
            },
        )
        .parse_next(i)
}
