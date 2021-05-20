use nom::bytes::complete::take_while;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::is_alphabetic,
    combinator::{all_consuming, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};

use crate::{
    immutable::{object::decode, parse, parse::NL, Signature},
    BStr, ByteSlice,
};

/// Represents a git tag, commonly indicating a software release.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tag<'a> {
    /// The hash in hexadecimal being the object this tag points to. Use [`target()`][Tag::target()] to obtain a byte representation.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub target: &'a BStr,
    /// The kind of object that `target` points to.
    pub target_kind: crate::Kind,
    /// The name of the tag, e.g. "v1.0".
    pub name: &'a BStr,
    /// The author of the tag.
    pub tagger: Option<Signature<'a>>,
    /// The message describing this release.
    pub message: &'a BStr,
    /// A cryptographic signature over the entire content of the serialized tag object thus far.
    pub pgp_signature: Option<&'a BStr>,
}

impl<'a> Tag<'a> {
    /// Deserialize a tag from `data`.
    pub fn from_bytes(data: &'a [u8]) -> Result<Tag<'a>, decode::Error> {
        parse(data).map(|(_, t)| t).map_err(decode::Error::from)
    }
    /// The object this tag points to as `Id`.
    pub fn target(&self) -> git_hash::ObjectId {
        git_hash::ObjectId::from_hex(self.target).expect("prior validation")
    }
}

fn parse(i: &[u8]) -> IResult<&[u8], Tag<'_>, decode::Error> {
    let (i, target) = parse::header_field(i, b"object", parse::hex_sha1)
        .map_err(decode::Error::context("object <40 lowercase hex char>"))?;

    let (i, kind) = parse::header_field(i, b"type", take_while1(is_alphabetic))
        .map_err(decode::Error::context("type <object kind>"))?;
    let kind = crate::Kind::from_bytes(kind).map_err(|e| nom::Err::Error(decode::Error::ParseKindError(e)))?;

    let (i, tag_version) =
        parse::header_field(i, b"tag", take_while1(|b| b != NL[0])).map_err(decode::Error::context("tag <version>"))?;

    let (i, signature) = opt(|i| parse::header_field(i, b"tagger", parse::signature))(i)
        .map_err(decode::Error::context("tagger <signature>"))?;
    let (i, (message, pgp_signature)) = all_consuming(parse_message)(i)?;
    Ok((
        i,
        Tag {
            target,
            name: tag_version.as_bstr(),
            target_kind: kind,
            message,
            tagger: signature,
            pgp_signature,
        },
    ))
}

fn parse_message(i: &[u8]) -> IResult<&[u8], (&BStr, Option<&BStr>), decode::Error> {
    const PGP_SIGNATURE_BEGIN: &[u8] = b"\n-----BEGIN PGP SIGNATURE-----";
    const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----";

    if i.is_empty() {
        return Ok((i, (i.as_bstr(), None)));
    }
    let (i, _) = tag(NL)(i)?;
    fn all_to_end(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8]), decode::Error> {
        if i.is_empty() {
            return Err(nom::Err::Error(decode::Error::NomDetail(
                i.into(),
                "tag message is missing",
            )));
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
                recognize(tuple((
                    tag(&PGP_SIGNATURE_BEGIN[1..]),
                    take_until(PGP_SIGNATURE_END),
                    tag(PGP_SIGNATURE_END),
                    take_while(|_| true),
                ))),
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

///
pub mod iter {
    use crate::{
        bstr::ByteSlice,
        immutable::{
            object::decode,
            parse::NL,
            tag::{parse, parse_message},
            Signature,
        },
        Kind,
    };
    use bstr::BStr;
    use git_hash::{oid, ObjectId};
    use nom::{
        bytes::complete::take_while1,
        character::is_alphabetic,
        combinator::{all_consuming, opt},
    };

    enum State {
        Target,
        TargetKind,
        Name,
        Tagger,
        Message,
    }

    impl Default for State {
        fn default() -> Self {
            State::Target
        }
    }

    /// Like [`immutable::Tag`][super::Tag], but as `Iterator` to support entirely allocation free parsing.
    /// It's particularly useful to dereference only the target chain.
    pub struct Iter<'a> {
        data: &'a [u8],
        state: State,
    }

    impl<'a> Iter<'a> {
        /// Create a tag iterator from data.
        pub fn from_bytes(data: &'a [u8]) -> Iter<'a> {
            Iter {
                data,
                state: State::default(),
            }
        }
    }

    impl<'a> Iter<'a> {
        fn next_inner(i: &'a [u8], state: &mut State) -> Result<(&'a [u8], Token<'a>), decode::Error> {
            use State::*;
            Ok(match state {
                Target => {
                    let (i, target) = parse::header_field(i, b"object", parse::hex_sha1)
                        .map_err(decode::Error::context("object <40 lowercase hex char>"))?;
                    *state = State::TargetKind;
                    (
                        i,
                        Token::Target {
                            id: ObjectId::from_hex(target).expect("parsing validation"),
                        },
                    )
                }
                TargetKind => {
                    let (i, kind) = parse::header_field(i, b"type", take_while1(is_alphabetic))
                        .map_err(decode::Error::context("type <object kind>"))?;
                    let kind =
                        crate::Kind::from_bytes(kind).map_err(|e| nom::Err::Error(decode::Error::ParseKindError(e)))?;
                    *state = State::Name;
                    (i, Token::TargetKind(kind))
                }
                Name => {
                    let (i, tag_version) = parse::header_field(i, b"tag", take_while1(|b| b != NL[0]))
                        .map_err(decode::Error::context("tag <version>"))?;
                    *state = State::Tagger;
                    (i, Token::Name(tag_version.as_bstr()))
                }
                Tagger => {
                    let (i, signature) = opt(|i| parse::header_field(i, b"tagger", parse::signature))(i)
                        .map_err(decode::Error::context("tagger <signature>"))?;
                    *state = State::Message;
                    (i, Token::Tagger(signature))
                }
                Message => {
                    let (i, (message, pgp_signature)) = all_consuming(parse_message)(i)?;
                    debug_assert!(
                        i.is_empty(),
                        "we should have consumed all data - otherwise iter may go forever"
                    );
                    return Ok((i, Token::Body { message, pgp_signature }));
                }
            })
        }
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = Result<Token<'a>, decode::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.data.is_empty() {
                return None;
            }
            match Self::next_inner(self.data, &mut self.state) {
                Ok((data, token)) => {
                    self.data = data;
                    Some(Ok(token))
                }
                Err(err) => {
                    self.data = &[];
                    Some(Err(err))
                }
            }
        }
    }

    /// A token returned by the [commit iterator][Iter].
    #[allow(missing_docs)]
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    pub enum Token<'a> {
        Target {
            id: ObjectId,
        },
        TargetKind(Kind),
        Name(&'a BStr),
        Tagger(Option<Signature<'a>>),
        Body {
            message: &'a BStr,
            pgp_signature: Option<&'a BStr>,
        },
    }

    impl<'a> Token<'a> {
        /// Return the object id of this token if its a [Target][Token::Target].
        pub fn id(&self) -> Option<&oid> {
            match self {
                Token::Target { id } => Some(id.as_ref()),
                _ => None,
            }
        }

        /// Return the owned object id of this token if its a [Target][Token::Target].
        pub fn into_id(self) -> Option<ObjectId> {
            match self {
                Token::Target { id } => Some(id),
                _ => None,
            }
        }
    }
}
