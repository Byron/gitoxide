use winnow::{
    branch::alt,
    bytes::{tag, take_until0, take_while0, take_while1},
    combinator::{eof, opt},
    error::{ContextError, ParseError},
    prelude::*,
    sequence::{preceded, terminated},
    stream::AsChar,
};

use crate::{parse, parse::NL, BStr, ByteSlice, TagRef};

pub fn git_tag<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(i: &'a [u8]) -> IResult<&[u8], TagRef<'a>, E> {
    let (i, target) = (|i| parse::header_field(i, b"object", parse::hex_hash))
        .context("object <40 lowercase hex char>")
        .parse_next(i)?;

    let (i, kind) = (|i| parse::header_field(i, b"type", take_while1(AsChar::is_alpha)))
        .context("type <object kind>")
        .parse_next(i)?;
    let kind = crate::Kind::from_bytes(kind)
        .map_err(|_| winnow::error::ErrMode::from_error_kind(i, winnow::error::ErrorKind::Verify))?;

    let (i, tag_version) = (|i| parse::header_field(i, b"tag", take_while1(|b| b != NL[0])))
        .context("tag <version>")
        .parse_next(i)?;

    let (i, signature) = opt(|i| parse::header_field(i, b"tagger", parse::signature))
        .context("tagger <signature>")
        .parse_next(i)?;
    let (i, (message, pgp_signature)) = terminated(message, eof).parse_next(i)?;
    Ok((
        i,
        TagRef {
            target,
            name: tag_version.as_bstr(),
            target_kind: kind,
            message,
            tagger: signature,
            pgp_signature,
        },
    ))
}

pub fn message<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], (&'a BStr, Option<&'a BStr>), E> {
    const PGP_SIGNATURE_BEGIN: &[u8] = b"\n-----BEGIN PGP SIGNATURE-----";
    const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----";

    if i.is_empty() {
        return Ok((i, (i.as_bstr(), None)));
    }
    let (i, _) = tag(NL).parse_next(i)?;
    fn all_to_end<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], (&'a [u8], &'a [u8]), E> {
        if i.is_empty() {
            // Empty message. That's OK.
            return Ok((&[], (&[], &[])));
        }
        // an empty signature message signals that there is none - the function signature is needed
        // to work with 'alt(…)'. PGP signatures are never empty
        Ok((&[], (i, &[])))
    }
    let (i, (message, signature)) = alt((
        (
            take_until0(PGP_SIGNATURE_BEGIN),
            preceded(
                NL,
                (
                    &PGP_SIGNATURE_BEGIN[1..],
                    take_until0(PGP_SIGNATURE_END),
                    PGP_SIGNATURE_END,
                    take_while0(|_| true),
                )
                    .recognize(),
            ),
        ),
        all_to_end,
    ))
    .parse_next(i)?;
    let (i, _) = opt(NL).parse_next(i)?;
    Ok((
        i,
        (
            message.as_bstr(),
            if signature.is_empty() {
                None
            } else {
                Some(signature.as_bstr())
            },
        ),
    ))
}
