use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::{complete::is_not, complete::tag},
    combinator::{all_consuming, opt},
    multi::many0,
    IResult,
};
use smallvec::SmallVec;

use crate::{
    immutable::{object::decode, parse, parse::NL, Signature},
    BStr, ByteSlice,
};

/// A git commit parsed using [`from_bytes()`][Commit::from_bytes()].
///
/// A commit encapsulates information about a point in time at which the state of the repository is recorded, usually after a
/// change which is documented in the commit `message`.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Commit<'a> {
    /// HEX hash of tree object we point to. Usually 40 bytes long.
    ///
    /// Use [`tree()`][Commit::tree()] to obtain a decoded version of it.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub tree: &'a BStr,
    /// HEX hash of each parent commit. Empty for first commit in repository.
    pub parents: SmallVec<[&'a BStr; 2]>,
    /// Who wrote this commit.
    pub author: Signature<'a>,
    /// Who committed this commit.
    ///
    /// This may be different from the `author` in case the author couldn't write to the repository themselves and
    /// is commonly encountered with contributed commits.
    pub committer: Signature<'a>,
    /// The name of the message encoding, otherwise [UTF-8 should be assumed](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/commit.c#L1493:L1493).
    pub encoding: Option<&'a BStr>,
    /// The commit message documenting the change.
    pub message: &'a BStr,
    /// Extra header fields, in order of them being encountered, made accessible with the iterator returned by [`extra_headers()`][Commit::extra_headers()].
    pub extra_headers: Vec<(&'a BStr, Cow<'a, BStr>)>,
}

impl<'a> Commit<'a> {
    /// Deserialize a commit from the given `data` bytes while avoiding most allocations.
    pub fn from_bytes(data: &'a [u8]) -> Result<Commit<'a>, decode::Error> {
        parse(data).map(|(_, t)| t).map_err(decode::Error::from)
    }
    /// Return the `tree` fields hash digest.
    pub fn tree(&self) -> git_hash::ObjectId {
        git_hash::ObjectId::from_hex(self.tree).expect("prior validation of tree hash during parsing")
    }

    /// Returns an iterator of parent object ids
    pub fn parents(&self) -> impl Iterator<Item = git_hash::ObjectId> + '_ {
        self.parents
            .iter()
            .map(|hex_hash| git_hash::ObjectId::from_hex(hex_hash).expect("prior validation of hashes during parsing"))
    }

    /// Returns a convenient iterator over all extra headers.
    pub fn extra_headers(&self) -> crate::commit::ExtraHeaders<impl Iterator<Item = (&BStr, &BStr)>> {
        crate::commit::ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (*k, v.as_ref())))
    }
}

fn parse_message(i: &[u8]) -> IResult<&[u8], &BStr, decode::Error> {
    if i.is_empty() {
        // newline + [message]
        return Err(nom::Err::Error(decode::Error::NomDetail(
            i.into(),
            "commit message is missing",
        )));
    }
    let (i, _) = tag(NL)(i).map_err(decode::Error::context("a newline separates headers from the message"))?;
    Ok((&[], &i.as_bstr()))
}

fn parse(i: &[u8]) -> IResult<&[u8], Commit<'_>, decode::Error> {
    let (i, tree) = parse::header_field(i, b"tree", parse::hex_sha1)
        .map_err(decode::Error::context("tree <40 lowercase hex char>"))?;
    let (i, parents) = many0(|i| parse::header_field(i, b"parent", parse::hex_sha1))(i)
        .map_err(decode::Error::context("zero or more 'parent <40 lowercase hex char>'"))?;
    let (i, author) =
        parse::header_field(i, b"author", parse::signature).map_err(decode::Error::context("author <signature>"))?;
    let (i, committer) = parse::header_field(i, b"committer", parse::signature)
        .map_err(decode::Error::context("committer <signature>"))?;
    let (i, encoding) = opt(|i| parse::header_field(i, b"encoding", is_not(NL)))(i)
        .map_err(decode::Error::context("encoding <encoding>"))?;
    let (i, extra_headers) = many0(alt((
        |i| parse::any_header_field_multi_line(i).map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Owned(o)))),
        |i| parse::any_header_field(i, is_not(NL)).map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Borrowed(o.as_bstr())))),
    )))(i)
    .map_err(decode::Error::context("<field> <single-line|multi-line>"))?;
    let (i, message) = all_consuming(parse_message)(i)?;

    Ok((
        i,
        Commit {
            tree,
            parents: SmallVec::from(parents),
            author,
            committer,
            encoding: encoding.map(ByteSlice::as_bstr),
            message,
            extra_headers,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_commit() {
        assert_eq!(
            std::mem::size_of::<Commit<'_>>(),
            216,
            "the size of an immutable commit shouldn't change unnoticed"
        );
    }
}

///
pub mod iter {
    use crate::{
        bstr::ByteSlice,
        immutable::{commit::parse_message, object::decode, parse, parse::NL, Signature},
    };
    use bstr::BStr;
    use git_hash::{oid, ObjectId};
    use nom::{
        branch::alt,
        bytes::complete::is_not,
        combinator::{all_consuming, opt},
    };
    use std::borrow::Cow;

    #[derive(Copy, Clone)]
    enum SignatureKind {
        Author,
        Committer,
    }

    enum State {
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

    /// Like [`immutable::Commit`][super::Commit], but as `Iterator` to support (up to) entirely allocation free parsing.
    /// It's particularly useful to traverse the commit graph without ever allocating arrays for parents.
    pub struct Iter<'a> {
        data: &'a [u8],
        state: State,
    }

    impl<'a> Iter<'a> {
        /// Create a commit iterator from data.
        pub fn from_bytes(data: &'a [u8]) -> Iter<'a> {
            Iter {
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
        pub fn signatures(&'a mut self) -> impl Iterator<Item = Signature<'_>> + 'a {
            self.filter_map(Result::ok)
                .skip_while(|t| !matches!(t, Token::Author { .. } | Token::Committer { .. }))
                .filter_map(|t| match t {
                    Token::Author { signature } | Token::Committer { signature } => Some(signature),
                    _ => None,
                })
        }
    }

    impl<'a> Iter<'a> {
        fn next_inner(i: &'a [u8], state: &mut State) -> Result<(&'a [u8], Token<'a>), decode::Error> {
            use State::*;
            Ok(match state {
                Tree => {
                    let (i, tree) = parse::header_field(i, b"tree", parse::hex_sha1)
                        .map_err(decode::Error::context("tree <40 lowercase hex char>"))?;
                    *state = State::Parents;
                    (
                        i,
                        Token::Tree {
                            id: ObjectId::from_hex(tree).expect("parsing validation"),
                        },
                    )
                }
                Parents => {
                    let (i, parent) = opt(|i| parse::header_field(i, b"parent", parse::hex_sha1))(i)
                        .map_err(decode::Error::context("commit <40 lowercase hex char>"))?;
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
                    let (i, signature) = parse::header_field(i, field_name, parse::signature)
                        .map_err(decode::Error::context(err_msg))?;
                    (
                        i,
                        match who {
                            SignatureKind::Author => Token::Author { signature },
                            SignatureKind::Committer => Token::Committer { signature },
                        },
                    )
                }
                Encoding => {
                    let (i, encoding) = opt(|i| parse::header_field(i, b"encoding", is_not(NL)))(i)
                        .map_err(decode::Error::context("encoding <encoding>"))?;
                    *state = State::ExtraHeaders;
                    match encoding {
                        Some(encoding) => (i, Token::Encoding(encoding.as_bstr())),
                        None => return Self::next_inner(i, state),
                    }
                }
                ExtraHeaders => {
                    let (i, extra_header) = opt(alt((
                        |i| parse::any_header_field_multi_line(i).map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Owned(o)))),
                        |i| {
                            parse::any_header_field(i, is_not(NL))
                                .map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Borrowed(o.as_bstr()))))
                        },
                    )))(i)
                    .map_err(decode::Error::context("<field> <single-line|multi-line>"))?;
                    match extra_header {
                        Some(extra_header) => (i, Token::ExtraHeader(extra_header)),
                        None => {
                            *state = State::Message;
                            return Self::next_inner(i, state);
                        }
                    }
                }
                Message => {
                    let (i, message) = all_consuming(parse_message)(i)?;
                    debug_assert!(
                        i.is_empty(),
                        "we should have consumed all data - otherwise iter may go forever"
                    );
                    return Ok((i, Token::Message(message)));
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
        Tree {
            id: ObjectId,
        },
        Parent {
            id: ObjectId,
        },
        /// A person who authored the content of the commit.
        Author {
            signature: Signature<'a>,
        },
        /// A person who committed the authors work to the repository.
        Committer {
            signature: Signature<'a>,
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
}
