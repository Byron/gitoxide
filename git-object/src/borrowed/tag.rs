use crate::{
    borrowed::{parse, parse::NL, Error, Signature},
    owned, BStr, ByteSlice,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::is_alphabetic,
    combinator::{all_consuming, opt, recognize},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tag<'a> {
    // Target SHA1 in hex, always 40 lower case characters from 0-9 and a-f
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub target: &'a BStr,
    // The name of the tag, e.g. "v1.0"
    pub name: &'a BStr,
    pub target_kind: crate::Kind,
    pub message: &'a BStr,
    pub signature: Option<Signature<'a>>,
    pub pgp_signature: Option<&'a BStr>,
}

fn parse(i: &[u8]) -> IResult<&[u8], Tag, Error> {
    let (i, target) =
        parse::header_field(i, b"object", parse::hex_sha1).map_err(Error::context("object <40 lowercase hex char>"))?;

    let (i, kind) =
        parse::header_field(i, b"type", take_while1(is_alphabetic)).map_err(Error::context("type <object kind>"))?;
    let kind = crate::Kind::from_bytes(kind).map_err(|e| nom::Err::Error(Error::ParseKindError(e)))?;

    let (i, tag_version) =
        parse::header_field(i, b"tag", take_while1(|b| b != NL[0])).map_err(Error::context("tag <version>"))?;

    let (i, signature) = opt(|i| parse::header_field(i, b"tagger", parse::signature))(i)
        .map_err(Error::context("tagger <signature>"))?;
    let (i, (message, pgp_signature)) = all_consuming(parse_message)(i)?;
    Ok((
        i,
        Tag {
            target,
            name: tag_version.as_bstr(),
            target_kind: kind,
            message,
            signature,
            pgp_signature,
        },
    ))
}

fn parse_message(i: &[u8]) -> IResult<&[u8], (&BStr, Option<&BStr>), Error> {
    const PGP_SIGNATURE_BEGIN: &[u8] = b"\n-----BEGIN PGP SIGNATURE-----";
    const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----";

    if i.is_empty() {
        return Ok((i, (i.as_bstr(), None)));
    }
    let (i, _) = tag(NL)(i)?;
    fn all_to_end(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8]), Error> {
        if i.is_empty() {
            return Err(nom::Err::Error(Error::NomDetail(i.into(), "tag message is missing")));
        }
        // an empty signature message signals that there is none - the function signature is needed
        // to work with 'alt(…)'. PGP signatures are never empty
        Ok((&[], (&i, &[])))
    }
    let (i, (message, signature)) = alt((
        tuple((
            take_until(PGP_SIGNATURE_BEGIN),
            preceded(
                tag(NL),
                recognize(delimited(
                    tag(&PGP_SIGNATURE_BEGIN[1..]),
                    take_until(PGP_SIGNATURE_END),
                    tag(PGP_SIGNATURE_END),
                )),
            ),
        )),
        all_to_end,
    ))(i)?;
    let (i, _) = opt(tag(NL))(i)?;
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

impl<'a> Tag<'a> {
    pub fn target(&self) -> owned::Id {
        owned::Id::from_40_bytes_in_hex(self.target).expect("prior validation")
    }
    pub fn from_bytes(d: &'a [u8]) -> Result<Tag<'a>, Error> {
        parse(d).map(|(_, t)| t).map_err(Error::from)
    }
}
