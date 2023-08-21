use winnow::{
    combinator::{alt, delimited, eof, opt, preceded, rest, terminated},
    error::{AddContext, ParserError, StrContext},
    prelude::*,
    stream::AsChar,
    token::{take_until0, take_while},
};

use crate::{parse, parse::NL, BStr, ByteSlice, TagRef};

pub fn git_tag<'a, E: ParserError<&'a [u8]> + AddContext<&'a [u8], StrContext>>(
    i: &mut &'a [u8],
) -> PResult<TagRef<'a>, E> {
    (
        (|i: &mut _| parse::header_field(i, b"object", parse::hex_hash))
            .context(StrContext::Expected("object <40 lowercase hex char>".into())),
        (|i: &mut _| parse::header_field(i, b"type", take_while(1.., AsChar::is_alpha)))
            .verify_map(|kind| crate::Kind::from_bytes(kind).ok())
            .context(StrContext::Expected("type <object kind>".into())),
        (|i: &mut _| parse::header_field(i, b"tag", take_while(1.., |b| b != NL[0])))
            .context(StrContext::Expected("tag <version>".into())),
        opt(|i: &mut _| parse::header_field(i, b"tagger", parse::signature))
            .context(StrContext::Expected("tagger <signature>".into())),
        terminated(message, eof),
    )
        .map(
            |(target, kind, tag_version, signature, (message, pgp_signature))| TagRef {
                target,
                name: tag_version.as_bstr(),
                target_kind: kind,
                message,
                tagger: signature,
                pgp_signature,
            },
        )
        .parse_next(i)
}

pub fn message<'a, E: ParserError<&'a [u8]>>(i: &mut &'a [u8]) -> PResult<(&'a BStr, Option<&'a BStr>), E> {
    const PGP_SIGNATURE_BEGIN: &[u8] = b"\n-----BEGIN PGP SIGNATURE-----";
    const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----";

    if i.is_empty() {
        return Ok((b"".as_bstr(), None));
    }
    delimited(
        NL,
        alt((
            (
                take_until0(PGP_SIGNATURE_BEGIN),
                preceded(
                    NL,
                    (
                        &PGP_SIGNATURE_BEGIN[1..],
                        take_until0(PGP_SIGNATURE_END),
                        PGP_SIGNATURE_END,
                        rest,
                    )
                        .recognize()
                        .map(|signature: &[u8]| {
                            if signature.is_empty() {
                                None
                            } else {
                                Some(signature.as_bstr())
                            }
                        }),
                ),
            ),
            rest.map(|rest: &[u8]| (rest, None)),
        )),
        opt(NL),
    )
    .map(|(message, signature)| (message.as_bstr(), signature))
    .parse_next(i)
}
