use std::convert::{TryFrom, TryInto};

use gix_hash::ObjectId;
use gix_object::bstr::BString;
use winnow::{
    combinator::{opt, terminated},
    prelude::*,
    token::take_while,
};

use crate::{
    parse::{hex_hash, newline},
    store_impl::file::loose::Reference,
    FullName, Target,
};

enum MaybeUnsafeState {
    Id(ObjectId),
    UnvalidatedPath(BString),
}

/// The error returned by [`Reference::try_from_path()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("{content:?} could not be parsed")]
    Parse { content: BString },
    #[error("The path {path:?} to a symbolic reference within a ref file is invalid")]
    RefnameValidation {
        source: gix_validate::reference::name::Error,
        path: BString,
    },
}

impl TryFrom<MaybeUnsafeState> for Target {
    type Error = Error;

    fn try_from(v: MaybeUnsafeState) -> Result<Self, Self::Error> {
        Ok(match v {
            MaybeUnsafeState::Id(id) => Target::Peeled(id),
            MaybeUnsafeState::UnvalidatedPath(name) => {
                Target::Symbolic(match gix_validate::reference::name(name.as_ref()) {
                    Ok(_) => FullName(name),
                    Err(err) => {
                        return Err(Error::RefnameValidation {
                            source: err,
                            path: name,
                        })
                    }
                })
            }
        })
    }
}

impl Reference {
    /// Create a new reference of the given `parent` store with `relative_path` service as unique identifier
    /// at which the `path_contents` was read to obtain the refs value.
    pub fn try_from_path(name: FullName, mut path_contents: &[u8]) -> Result<Self, Error> {
        Ok(Reference {
            name,
            target: parse(&mut path_contents)
                .map_err(|_| Error::Parse {
                    content: path_contents.into(),
                })?
                .try_into()?,
        })
    }
}

fn parse(i: &mut &[u8]) -> PResult<MaybeUnsafeState> {
    if let Some(_ref_prefix) = opt(terminated("ref: ", take_while(0.., b' '))).parse_next(i)? {
        terminated(take_while(0.., |b| b != b'\r' && b != b'\n'), opt(newline))
            .map(|path| MaybeUnsafeState::UnvalidatedPath(path.into()))
            .parse_next(i)
    } else {
        terminated(hex_hash, opt(newline))
            .map(|hex| MaybeUnsafeState::Id(ObjectId::from_hex(hex).expect("prior validation")))
            .parse_next(i)
    }
}
