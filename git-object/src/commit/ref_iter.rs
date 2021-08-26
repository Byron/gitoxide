use std::borrow::Cow;

use bstr::BStr;
use git_hash::{oid, ObjectId};
use nom::{
    branch::alt,
    bytes::complete::is_not,
    combinator::{all_consuming, opt},
    error::context,
};

use crate::{
    bstr::ByteSlice,
    commit::decode,
    immutable::{object, parse, parse::NL},
    CommitRefIter,
};

#[derive(Copy, Clone)]
pub(crate) enum SignatureKind {
    Author,
    Committer,
}

pub(crate) enum State {
    Tree,
    Parents,
    Signature { of: SignatureKind },
    Encoding,
    ExtraHeaders,
    Message,
}

impl Default for State {
    fn default() -> Self {
        State::Tree
    }
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
    pub fn tree_id(&mut self) -> Option<ObjectId> {
        self.next().and_then(Result::ok).and_then(Token::into_id)
    }

    /// Returns all signatures, first the author, then the committer, if there is no decoding error.
    ///
    /// Errors are coerced into options, hiding whether there was an error or not. The caller knows if there was an error or not
    /// if not exactly two signatures were iterable.
    /// Errors are not the common case - if an error needs to be detectable, use this instance as iterator.
    pub fn signatures(&'a mut self) -> impl Iterator<Item = git_actor::SignatureRef<'_>> + 'a {
        self.filter_map(Result::ok)
            .skip_while(|t| !matches!(t, Token::Author { .. } | Token::Committer { .. }))
            .filter_map(|t| match t {
                Token::Author { signature } | Token::Committer { signature } => Some(signature),
                _ => None,
            })
    }
}

impl<'a> CommitRefIter<'a> {
    fn next_inner(i: &'a [u8], state: &mut State) -> Result<(&'a [u8], Token<'a>), object::decode::Error> {
        use State::*;
        Ok(match state {
            Tree => {
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
            Parents => {
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
            Signature { ref mut of } => {
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
            Encoding => {
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
            ExtraHeaders => {
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
            Message => {
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
        signature: git_actor::SignatureRef<'a>,
    },
    /// A person who committed the authors work to the repository.
    Committer {
        signature: git_actor::SignatureRef<'a>,
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
    pub fn into_id(self) -> Option<ObjectId> {
        match self {
            Token::Tree { id } | Token::Parent { id } => Some(id),
            _ => None,
        }
    }
}
