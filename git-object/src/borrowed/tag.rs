use super::Error;
use crate::borrowed::{
    util::{parse_hex_sha1, parse_oneline_header, parse_signature, NL},
    Signature,
};
use bstr::{BStr, ByteSlice};
use hex::FromHex;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::is_alphabetic,
    combinator::{all_consuming, recognize},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Tag<'data> {
    pub target: &'data BStr,
    pub name: &'data BStr,
    pub target_kind: crate::Kind,
    pub message: &'data BStr,
    pub signature: Signature<'data>,
    pub pgp_signature: Option<&'data BStr>,
}

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], Tag, Error> {
    let (i, target) = parse_oneline_header(i, b"object", parse_hex_sha1)
        .map_err(Error::context("object <40 lowercase hex char>"))?;

    let (i, kind) = parse_oneline_header(i, b"type", take_while1(is_alphabetic))
        .map_err(Error::context("type <object kind>"))?;
    let kind =
        crate::Kind::from_bytes(kind).map_err(|e| nom::Err::Error(Error::ParseKindError(e)))?;

    let (i, tag_version) = parse_oneline_header(i, b"tag", take_while1(|b| b != NL[0]))
        .map_err(Error::context("tag <version>"))?;

    let (i, signature) = parse_oneline_header(i, b"tagger", parse_signature)
        .map_err(Error::context("tagger <signature>"))?;
    let (i, (message, pgp_signature)) = all_consuming(parse_message)(i)?;
    Ok((
        i,
        Tag {
            target: target.as_bstr(),
            name: tag_version.as_bstr(),
            target_kind: kind,
            message,
            signature,
            pgp_signature,
        },
    ))
}

pub(crate) fn parse_message(i: &[u8]) -> IResult<&[u8], (&BStr, Option<&BStr>), Error> {
    const PGP_SIGNATURE_BEGIN: &[u8] = b"\n-----BEGIN PGP SIGNATURE-----";
    const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----\n";

    let (i, _) = tag(NL)(i)?;
    if i.is_empty() {
        return Ok((i, (i.as_bstr(), None)));
    }
    fn all_but_trailing_newline(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8]), Error> {
        if i.len() < 2 {
            return Err(nom::Err::Error(Error::NomDetail(
                i.into(),
                "tag message is missing",
            )));
        }
        let (x, _) = tag(NL)(&i[i.len() - 1..])
            .map_err(Error::context("tag message must end with newline"))?;
        // an empty signature message signals that there is none - the function signature is needed
        // to work with 'alt(…)'. PGP signatures are never empty
        Ok((x, (&i[..i.len() - 1], &[])))
    }
    let (i, (message, signature)) = alt((
        tuple((
            take_until(PGP_SIGNATURE_BEGIN),
            delimited(
                tag(NL),
                recognize(delimited(
                    tag(&PGP_SIGNATURE_BEGIN[1..]),
                    take_until(PGP_SIGNATURE_END),
                    tag(&PGP_SIGNATURE_END[..PGP_SIGNATURE_END.len() - 1]),
                )),
                tag(NL),
            ),
        )),
        all_but_trailing_newline,
    ))(i)?;
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

impl<'data> Tag<'data> {
    pub fn target(&self) -> crate::Id {
        <[u8; 20]>::from_hex(self.target).expect("prior validation")
    }
    pub fn from_bytes(d: &'data [u8]) -> Result<Tag<'data>, Error> {
        parse(d).map(|(_, t)| t).map_err(Error::from)
    }
}
