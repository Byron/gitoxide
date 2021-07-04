use crate::{
    file::{reference::State, Reference, Store},
    parse::{hex_sha1, newline},
};
use bstr::BString;
use git_hash::ObjectId;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::{map, opt},
    sequence::terminated,
    IResult,
};
use quick_error::quick_error;
use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
};

enum MaybeUnsafeState {
    Id(ObjectId),
    UnvalidatedPath(BString),
}

quick_error! {
    /// The error returned by [`Reference::try_from_path()`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        Parse(content: BString) {
            display("{:?} could not be parsed", content)
        }
        RefnameValidation{err: git_validate::reference::name::Error, path: BString} {
            display("The path to a symbolic reference within a ref file is invalid")
            source(err)
        }
    }
}

impl TryFrom<MaybeUnsafeState> for State {
    type Error = Error;

    fn try_from(v: MaybeUnsafeState) -> Result<Self, Self::Error> {
        Ok(match v {
            MaybeUnsafeState::Id(id) => State::Id(id),
            MaybeUnsafeState::UnvalidatedPath(path) => {
                State::ValidatedPath(match git_validate::refname(path.as_ref()) {
                    Err(err) => return Err(Error::RefnameValidation { err, path }),
                    Ok(_) => path,
                })
            }
        })
    }
}

impl<'a> Reference<'a> {
    /// Create a new reference of the given `parent` store with `relative_path` service as unique identifier
    /// at which the `path_contents` was read to obtain the refs value.
    pub fn try_from_path(
        parent: &'a Store,
        relative_path: impl Into<PathBuf>,
        path_contents: &[u8],
    ) -> Result<Self, Error> {
        Ok(Reference {
            parent,
            relative_path: relative_path.into(),
            state: parse(path_contents)
                .map_err(|_| Error::Parse(path_contents.into()))?
                .1
                .try_into()?,
        })
    }
}

fn parse(bytes: &[u8]) -> IResult<&[u8], MaybeUnsafeState> {
    let is_space = |b: u8| b == b' ';
    if let (path, Some(_ref_prefix)) = opt(terminated(tag("ref: "), take_while(is_space)))(bytes)? {
        map(terminated(take_while(|b| b != b'\r' && b != b'\n'), newline), |path| {
            MaybeUnsafeState::UnvalidatedPath(path.into())
        })(path)
    } else {
        map(terminated(hex_sha1, newline), |hex| {
            MaybeUnsafeState::Id(ObjectId::from_hex(hex).expect("prior validation"))
        })(bytes)
    }
}
