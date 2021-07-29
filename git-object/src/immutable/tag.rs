use crate::{immutable::object, BStr};

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
    pub tagger: Option<git_actor::immutable::Signature<'a>>,
    /// The message describing this release.
    pub message: &'a BStr,
    /// A cryptographic signature over the entire content of the serialized tag object thus far.
    pub pgp_signature: Option<&'a BStr>,
}

impl<'a> Tag<'a> {
    /// Deserialize a tag from `data`.
    pub fn from_bytes(data: &'a [u8]) -> Result<Tag<'a>, object::decode::Error> {
        decode::git_tag(data)
            .map(|(_, t)| t)
            .map_err(object::decode::Error::from)
    }
    /// The object this tag points to as `Id`.
    pub fn target(&self) -> git_hash::ObjectId {
        git_hash::ObjectId::from_hex(self.target).expect("prior validation")
    }
}

mod decode {
    use nom::bytes::complete::take_while;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until, take_while1},
        character::is_alphabetic,
        combinator::{all_consuming, opt, recognize},
        error::context,
        sequence::{preceded, tuple},
        IResult,
    };

    use crate::immutable::Tag;
    use crate::{
        immutable::{parse, parse::NL},
        BStr, ByteSlice,
    };
    use nom::error::{ContextError, ParseError};

    pub fn git_tag<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(i: &'a [u8]) -> IResult<&[u8], Tag<'a>, E> {
        let (i, target) = context("object <40 lowercase hex char>", |i| {
            parse::header_field(i, b"object", parse::hex_hash)
        })(i)?;

        let (i, kind) = context("type <object kind>", |i| {
            parse::header_field(i, b"type", take_while1(is_alphabetic))
        })(i)?;
        let kind = crate::Kind::from_bytes(kind)
            .map_err(|_| nom::Err::Error(E::from_error_kind(i, nom::error::ErrorKind::MapRes)))?;

        let (i, tag_version) = context("tag <version>", |i| {
            parse::header_field(i, b"tag", take_while1(|b| b != NL[0]))
        })(i)?;

        let (i, signature) = context(
            "tagger <signature>",
            opt(|i| parse::header_field(i, b"tagger", parse::signature)),
        )(i)?;
        let (i, (message, pgp_signature)) = all_consuming(message)(i)?;
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

    pub fn message<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], (&'a BStr, Option<&'a BStr>), E> {
        const PGP_SIGNATURE_BEGIN: &[u8] = b"\n-----BEGIN PGP SIGNATURE-----";
        const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----";

        if i.is_empty() {
            return Ok((i, (i.as_bstr(), None)));
        }
        let (i, _) = tag(NL)(i)?;
        fn all_to_end<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], (&'a [u8], &'a [u8]), E> {
            if i.is_empty() {
                return Err(nom::Err::Error(E::from_error_kind(i, nom::error::ErrorKind::Eof)));
            }
            // an empty signature message signals that there is none - the function signature is needed
            // to work with 'alt(…)'. PGP signatures are never empty
            Ok((&[], (i, &[])))
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
}

///
pub mod iter {
    use crate::{
        bstr::ByteSlice,
        immutable::{object, parse, parse::NL, tag::decode},
        Kind,
    };
    use bstr::BStr;
    use git_hash::{oid, ObjectId};
    use nom::error::ParseError;
    use nom::{
        bytes::complete::take_while1,
        character::is_alphabetic,
        combinator::{all_consuming, opt},
        error::context,
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

        /// Returns the target id of this tag if it is the first function called and if there is no error in decoding
        /// the data.
        ///
        /// Note that this method must only be called once or else will always return None while consuming a single token.
        /// Errors are coerced into options, hiding whether there was an error or not. The caller should assume an error if they
        /// call the method as intended. Such a squelched error cannot be recovered unless the objects data is retrieved and parsed again.
        /// `next()`.
        pub fn target_id(&mut self) -> Option<ObjectId> {
            self.next().and_then(Result::ok).and_then(Token::into_id)
        }
    }

    impl<'a> Iter<'a> {
        fn next_inner(i: &'a [u8], state: &mut State) -> Result<(&'a [u8], Token<'a>), object::decode::Error> {
            use State::*;
            Ok(match state {
                Target => {
                    let (i, target) = context("object <40 lowercase hex char>", |i| {
                        parse::header_field(i, b"object", parse::hex_hash)
                    })(i)?;
                    *state = State::TargetKind;
                    (
                        i,
                        Token::Target {
                            id: ObjectId::from_hex(target).expect("parsing validation"),
                        },
                    )
                }
                TargetKind => {
                    let (i, kind) = context("type <object kind>", |i| {
                        parse::header_field(i, b"type", take_while1(is_alphabetic))
                    })(i)?;
                    let kind = crate::Kind::from_bytes(kind).map_err(|_| {
                        let err = object::decode::ParseError::from_error_kind(i, nom::error::ErrorKind::MapRes);
                        nom::Err::Error(err)
                    })?;
                    *state = State::Name;
                    (i, Token::TargetKind(kind))
                }
                Name => {
                    let (i, tag_version) = context("tag <version>", |i| {
                        parse::header_field(i, b"tag", take_while1(|b| b != NL[0]))
                    })(i)?;
                    *state = State::Tagger;
                    (i, Token::Name(tag_version.as_bstr()))
                }
                Tagger => {
                    let (i, signature) = context(
                        "tagger <signature>",
                        opt(|i| parse::header_field(i, b"tagger", parse::signature)),
                    )(i)?;
                    *state = State::Message;
                    (i, Token::Tagger(signature))
                }
                Message => {
                    let (i, (message, pgp_signature)) = all_consuming(decode::message)(i)?;
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
        type Item = Result<Token<'a>, object::decode::Error>;

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
        Tagger(Option<git_actor::immutable::Signature<'a>>),
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
