use std::borrow::Cow;

use smallvec::SmallVec;
use winnow::{
    combinator::{alt, eof, opt, preceded, repeat, rest, terminated},
    error::{AddContext, ParserError, StrContext},
    prelude::*,
    token::take_till1,
};

use crate::{parse, parse::NL, BStr, ByteSlice, CommitRef};

pub fn message<'a, E: ParserError<&'a [u8]> + AddContext<&'a [u8], StrContext>>(
    i: &mut &'a [u8],
) -> PResult<&'a BStr, E> {
    if i.is_empty() {
        // newline + [message]
        return Err(
            winnow::error::ErrMode::from_error_kind(i, winnow::error::ErrorKind::Eof)
                .add_context(i, StrContext::Expected("newline + <message>".into())),
        );
    }
    preceded(NL, rest.map(ByteSlice::as_bstr))
        .context(StrContext::Expected(
            "a newline separates headers from the message".into(),
        ))
        .parse_next(i)
}

pub fn commit<'a, E: ParserError<&'a [u8]> + AddContext<&'a [u8], StrContext>>(
    i: &mut &'a [u8],
) -> PResult<CommitRef<'a>, E> {
    (
        (|i: &mut _| parse::header_field(i, b"tree", parse::hex_hash))
            .context(StrContext::Expected("tree <40 lowercase hex char>".into())),
        repeat(0.., |i: &mut _| parse::header_field(i, b"parent", parse::hex_hash))
            .map(|p: Vec<_>| p)
            .context(StrContext::Expected(
                "zero or more 'parent <40 lowercase hex char>'".into(),
            )),
        (|i: &mut _| parse::header_field(i, b"author", parse::signature))
            .context(StrContext::Expected("author <signature>".into())),
        (|i: &mut _| parse::header_field(i, b"committer", parse::signature))
            .context(StrContext::Expected("committer <signature>".into())),
        opt(|i: &mut _| parse::header_field(i, b"encoding", take_till1(NL)))
            .context(StrContext::Expected("encoding <encoding>".into())),
        repeat(
            0..,
            alt((
                parse::any_header_field_multi_line.map(|(k, o)| (k.as_bstr(), Cow::Owned(o))),
                |i: &mut _| {
                    parse::any_header_field(i, take_till1(NL)).map(|(k, o)| (k.as_bstr(), Cow::Borrowed(o.as_bstr())))
                },
            )),
        )
        .context(StrContext::Expected("<field> <single-line|multi-line>".into())),
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
