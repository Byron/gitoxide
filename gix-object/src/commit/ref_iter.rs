use std::borrow::Cow;

use bstr::BStr;
use gix_hash::{oid, ObjectId};
use nom::{
    branch::alt,
    bytes::complete::is_not,
    combinator::{all_consuming, opt},
    error::context,
};

use crate::{bstr::ByteSlice, commit::decode, parse, parse::NL, CommitRefIter};

#[derive(Copy, Clone)]
pub(crate) enum SignatureKind {
    Author,
    Committer,
}

#[derive(Default, Copy, Clone)]
pub(crate) enum State {
    #[default]
    Tree,
    Parents,
    Signature {
        of: SignatureKind,
    },
    Encoding,
    ExtraHeaders,
    Message,
}

impl<'a> CommitRefIter<'a> {
    /// Create a commit iterator from data.
    pub fn from_bytes(data: &'a [u8]) -> CommitRefIter<'a> {
        CommitRefIter {
            data,
            state: State::default(),
        }
    }

    /// Returns the object id of this commits tree if it is the first function called and if there is no error in decoding
    /// the data.
    ///
    /// Note that this method must only be called once or else will always return None while consuming a single token.
    /// Errors are coerced into options, hiding whether there was an error or not. The caller should assume an error if they
    /// call the method as intended. Such a squelched error cannot be recovered unless the objects data is retrieved and parsed again.
    /// `next()`.
    pub fn tree_id(&mut self) -> Result<ObjectId, crate::decode::Error> {
        let tree_id = self.next().ok_or_else(missing_field)??;
        Token::try_into_id(tree_id).ok_or_else(missing_field)
    }

    /// Return all `parent_ids` as iterator.
    ///
    /// Parsing errors are ignored quietly.
    pub fn parent_ids(self) -> impl Iterator<Item = gix_hash::ObjectId> + 'a {
        self.filter_map(|t| match t {
            Ok(Token::Parent { id }) => Some(id),
            _ => None,
        })
    }

    /// Returns all signatures, first the author, then the committer, if there is no decoding error.
    ///
    /// Errors are coerced into options, hiding whether there was an error or not. The caller knows if there was an error or not
    /// if not exactly two signatures were iterable.
    /// Errors are not the common case - if an error needs to be detectable, use this instance as iterator.
    pub fn signatures(self) -> impl Iterator<Item = gix_actor::SignatureRef<'a>> + 'a {
        self.filter_map(|t| match t {
            Ok(Token::Author { signature } | Token::Committer { signature }) => Some(signature),
            _ => None,
        })
    }

    /// Returns the committer signature if there is no decoding error.
    pub fn committer(mut self) -> Result<gix_actor::SignatureRef<'a>, crate::decode::Error> {
        self.find_map(|t| match t {
            Ok(Token::Committer { signature }) => Some(Ok(signature)),
            Err(err) => Some(Err(err)),
            _ => None,
        })
        .ok_or_else(missing_field)?
    }

    /// Returns the author signature if there is no decoding error.
    ///
    /// It may contain white space surrounding it, and is exactly as parsed.
    pub fn author(mut self) -> Result<gix_actor::SignatureRef<'a>, crate::decode::Error> {
        self.find_map(|t| match t {
            Ok(Token::Author { signature }) => Some(Ok(signature)),
            Err(err) => Some(Err(err)),
            _ => None,
        })
        .ok_or_else(missing_field)?
    }

    /// Returns the message if there is no decoding error.
    ///
    /// It may contain white space surrounding it, and is exactly as
    //  parsed.
    pub fn message(mut self) -> Result<&'a BStr, crate::decode::Error> {
        self.find_map(|t| match t {
            Ok(Token::Message(msg)) => Some(Ok(msg)),
            Err(err) => Some(Err(err)),
            _ => None,
        })
        .transpose()
        .map(Option::unwrap_or_default)
    }
}

fn missing_field() -> crate::decode::Error {
    crate::decode::empty_error()
}

impl<'a> CommitRefIter<'a> {
    fn next_inner(i: &'a [u8], state: &mut State) -> Result<(&'a [u8], Token<'a>), crate::decode::Error> {
        Ok(match state {
            State::Tree => {
                let (i, tree) = context("tree <40 lowercase hex char>", |i| {
                    parse::header_field(i, b"tree", parse::hex_hash)
                })(i)?;
                *state = State::Parents;
                (
                    i,
                    Token::Tree {
                        id: ObjectId::from_hex(tree).expect("parsing validation"),
                    },
                )
            }
            State::Parents => {
                let (i, parent) = context(
                    "commit <40 lowercase hex char>",
                    opt(|i| parse::header_field(i, b"parent", parse::hex_hash)),
                )(i)?;
                match parent {
                    Some(parent) => (
                        i,
                        Token::Parent {
                            id: ObjectId::from_hex(parent).expect("parsing validation"),
                        },
                    ),
                    None => {
                        *state = State::Signature {
                            of: SignatureKind::Author,
                        };
                        return Self::next_inner(i, state);
                    }
                }
            }
            State::Signature { ref mut of } => {
                let who = *of;
                let (field_name, err_msg) = match of {
                    SignatureKind::Author => {
                        *of = SignatureKind::Committer;
                        (&b"author"[..], "author <signature>")
                    }
                    SignatureKind::Committer => {
                        *state = State::Encoding;
                        (&b"committer"[..], "committer <signature>")
                    }
                };
                let (i, signature) = context(err_msg, |i| parse::header_field(i, field_name, parse::signature))(i)?;
                (
                    i,
                    match who {
                        SignatureKind::Author => Token::Author { signature },
                        SignatureKind::Committer => Token::Committer { signature },
                    },
                )
            }
            State::Encoding => {
                let (i, encoding) = context(
                    "encoding <encoding>",
                    opt(|i| parse::header_field(i, b"encoding", is_not(NL))),
                )(i)?;
                *state = State::ExtraHeaders;
                match encoding {
                    Some(encoding) => (i, Token::Encoding(encoding.as_bstr())),
                    None => return Self::next_inner(i, state),
                }
            }
            State::ExtraHeaders => {
                let (i, extra_header) = context(
                    "<field> <single-line|multi-line>",
                    opt(alt((
                        |i| parse::any_header_field_multi_line(i).map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Owned(o)))),
                        |i| {
                            parse::any_header_field(i, is_not(NL))
                                .map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Borrowed(o.as_bstr()))))
                        },
                    ))),
                )(i)?;
                match extra_header {
                    Some(extra_header) => (i, Token::ExtraHeader(extra_header)),
                    None => {
                        *state = State::Message;
                        return Self::next_inner(i, state);
                    }
                }
            }
            State::Message => {
                let (i, message) = all_consuming(decode::message)(i)?;
                debug_assert!(
                    i.is_empty(),
                    "we should have consumed all data - otherwise iter may go forever"
                );
                return Ok((i, Token::Message(message)));
            }
        })
    }
}

impl<'a> Iterator for CommitRefIter<'a> {
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

/// A token returned by the [commit iterator][CommitRefIter].
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Token<'a> {
    Tree {
        id: ObjectId,
    },
    Parent {
        id: ObjectId,
    },
    /// A person who authored the content of the commit.
    Author {
        signature: gix_actor::SignatureRef<'a>,
    },
    /// A person who committed the authors work to the repository.
    Committer {
        signature: gix_actor::SignatureRef<'a>,
    },
    Encoding(&'a BStr),
    ExtraHeader((&'a BStr, Cow<'a, BStr>)),
    Message(&'a BStr),
}

impl<'a> Token<'a> {
    /// Return the object id of this token if its a [tree][Token::Tree] or a [parent commit][Token::Parent].
    pub fn id(&self) -> Option<&oid> {
        match self {
            Token::Tree { id } | Token::Parent { id } => Some(id.as_ref()),
            _ => None,
        }
    }

    /// Return the owned object id of this token if its a [tree][Token::Tree] or a [parent commit][Token::Parent].
    pub fn try_into_id(self) -> Option<ObjectId> {
        match self {
            Token::Tree { id } | Token::Parent { id } => Some(id),
            _ => None,
        }
    }
}
