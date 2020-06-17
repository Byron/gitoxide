use super::Error;
use crate::parsed::util::{parse_signature_nom, NL};
use crate::parsed::Signature;
use bstr::{BStr, ByteSlice};
use hex::FromHex;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1, take_while_m_n},
    character::is_alphabetic,
    combinator::{all_consuming, recognize},
    sequence::{delimited, preceded, terminated, tuple},
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

fn is_hex_digit_lc(b: u8) -> bool {
    match b {
        b'0'..=b'9' => true,
        b'a'..=b'f' => true,
        _ => false,
    }
}

pub(crate) fn parse_tag_nom(i: &[u8]) -> IResult<&[u8], Tag, Error> {
    let (i, target) = terminated(
        preceded(
            tag(b"object "),
            take_while_m_n(40usize, 40, is_hex_digit_lc),
        ),
        tag(NL),
    )(i)
    .map_err(Error::context("object <40 lowercase hex char>"))?;

    let (i, kind) = terminated(preceded(tag(b"type "), take_while1(is_alphabetic)), tag(NL))(i)
        .map_err(Error::context("type <object kind>"))?;
    let kind =
        crate::Kind::from_bytes(kind).map_err(|e| nom::Err::Error(Error::ParseKindError(e)))?;

    let (i, tag_version) =
        terminated(preceded(tag(b"tag "), take_while1(|b| b != NL[0])), tag(NL))(i)
            .map_err(Error::context("tag <version>"))?;

    let (i, signature) = terminated(preceded(tag(b"tagger "), parse_signature_nom), tag(NL))(i)
        .map_err(Error::context("tagger <signature>"))?;
    let (i, (message, pgp_signature)) = all_consuming(parse_message_nom)(i)?;
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

pub(crate) fn parse_message_nom(i: &[u8]) -> IResult<&[u8], (&BStr, Option<&BStr>), Error> {
    const PGP_SIGNATURE_BEGIN: &[u8] = b"\n-----BEGIN PGP SIGNATURE-----";
    const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----\n";

    let (i, _) = tag(NL)(i)?;
    if i.is_empty() {
        return Err(nom::Err::Error(Error::NomDetail(
            i.into(),
            "Missing tag message",
        )));
    }
    fn all_but_trailing_newline(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8]), Error> {
        if i.len() < 2 {
            return Err(nom::Err::Error(Error::NomDetail(
                i.into(),
                "tag message is missing",
            )));
        }
        let (i, _) = tag(NL)(&i[i.len() - 1..])
            .map_err(Error::context("tag message must end with newline"))?;
        // an empty signature message signals that there is none - the function signature is needed
        // to work with 'alt(…)'. PGP signatures are never empty
        Ok((i, (&i[..i.len() - 1], &[])))
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
        parse_tag_nom(d).map(|(_, t)| t).map_err(Error::from)
    }
}
