use bstr::BStr;
use gix_hash::{oid, ObjectId};
use winnow::{
    bytes::take_while,
    combinator::{eof, opt},
    error::ParseError,
    prelude::*,
    sequence::terminated,
    stream::AsChar,
};

use crate::{bstr::ByteSlice, parse, parse::NL, tag::decode, Kind, TagRefIter};

#[derive(Default, Copy, Clone)]
pub(crate) enum State {
    #[default]
    Target,
    TargetKind,
    Name,
    Tagger,
    Message,
}

impl<'a> TagRefIter<'a> {
    /// Create a tag iterator from data.
    pub fn from_bytes(data: &'a [u8]) -> TagRefIter<'a> {
        TagRefIter {
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
    pub fn target_id(mut self) -> Result<ObjectId, crate::decode::Error> {
        let token = self.next().ok_or_else(missing_field)??;
        Token::into_id(token).ok_or_else(missing_field)
    }

    /// Returns the taggers signature if there is no decoding error, and if this field exists.
    /// Errors are coerced into options, hiding whether there was an error or not. The caller knows if there was an error or not.
    pub fn tagger(mut self) -> Result<Option<gix_actor::SignatureRef<'a>>, crate::decode::Error> {
        self.find_map(|t| match t {
            Ok(Token::Tagger(signature)) => Some(Ok(signature)),
            Err(err) => Some(Err(err)),
            _ => None,
        })
        .ok_or_else(missing_field)?
    }
}

fn missing_field() -> crate::decode::Error {
    crate::decode::empty_error()
}

impl<'a> TagRefIter<'a> {
    fn next_inner(i: &'a [u8], state: &mut State) -> Result<(&'a [u8], Token<'a>), crate::decode::Error> {
        use State::*;
        Ok(match state {
            Target => {
                let (i, target) = (|i| parse::header_field(i, b"object", parse::hex_hash))
                    .context("object <40 lowercase hex char>")
                    .parse_next(i)?;
                *state = TargetKind;
                (
                    i,
                    Token::Target {
                        id: ObjectId::from_hex(target).expect("parsing validation"),
                    },
                )
            }
            TargetKind => {
                let (i, kind) = (|i| parse::header_field(i, b"type", take_while(1.., AsChar::is_alpha)))
                    .context("type <object kind>")
                    .parse_next(i)?;
                let kind = Kind::from_bytes(kind)
                    .map_err(|_| winnow::error::ErrMode::from_error_kind(i, winnow::error::ErrorKind::Verify))?;
                *state = Name;
                (i, Token::TargetKind(kind))
            }
            Name => {
                let (i, tag_version) = (|i| parse::header_field(i, b"tag", take_while(1.., |b| b != NL[0])))
                    .context("tag <version>")
                    .parse_next(i)?;
                *state = Tagger;
                (i, Token::Name(tag_version.as_bstr()))
            }
            Tagger => {
                let (i, signature) = opt(|i| parse::header_field(i, b"tagger", parse::signature))
                    .context("tagger <signature>")
                    .parse_next(i)?;
                *state = Message;
                (i, Token::Tagger(signature))
            }
            Message => {
                let (i, (message, pgp_signature)) = terminated(decode::message, eof).parse_next(i)?;
                debug_assert!(
                    i.is_empty(),
                    "we should have consumed all data - otherwise iter may go forever"
                );
                return Ok((i, Token::Body { message, pgp_signature }));
            }
        })
    }
}

impl<'a> Iterator for TagRefIter<'a> {
    type Item = Result<Token<'a>, crate::decode::Error>;

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

/// A token returned by the [tag iterator][TagRefIter].
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Token<'a> {
    Target {
        id: ObjectId,
    },
    TargetKind(Kind),
    Name(&'a BStr),
    Tagger(Option<gix_actor::SignatureRef<'a>>),
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
