use crate::{
    mutable,
    parse::{hex_hash, newline},
    store::file::loose::Reference,
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
use std::convert::{TryFrom, TryInto};

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

impl TryFrom<MaybeUnsafeState> for mutable::Target {
    type Error = Error;

    fn try_from(v: MaybeUnsafeState) -> Result<Self, Self::Error> {
        Ok(match v {
            MaybeUnsafeState::Id(id) => mutable::Target::Peeled(id),
            MaybeUnsafeState::UnvalidatedPath(name) => {
                mutable::Target::Symbolic(match git_validate::refname(name.as_ref()) {
                    Ok(_) => mutable::FullName(name),
                    Err(err) => return Err(Error::RefnameValidation { err, path: name }),
                })
            }
        })
    }
}

impl Reference {
    /// Create a new reference of the given `parent` store with `relative_path` service as unique identifier
    /// at which the `path_contents` was read to obtain the refs value.
    pub fn try_from_path(name: mutable::FullName, path_contents: &[u8]) -> Result<Self, Error> {
        Ok(Reference {
            name,
            target: parse(path_contents)
                .map_err(|_| Error::Parse(path_contents.into()))?
                .1
                .try_into()?,
        })
    }
}

fn parse(bytes: &[u8]) -> IResult<&[u8], MaybeUnsafeState> {
    let is_space = |b: u8| b == b' ';
    if let (path, Some(_ref_prefix)) = opt(terminated(tag("ref: "), take_while(is_space)))(bytes)? {
        map(
            terminated(take_while(|b| b != b'\r' && b != b'\n'), opt(newline)),
            |path| MaybeUnsafeState::UnvalidatedPath(path.into()),
        )(path)
    } else {
        map(terminated(hex_hash, opt(newline)), |hex| {
            MaybeUnsafeState::Id(ObjectId::from_hex(hex).expect("prior validation"))
        })(bytes)
    }
}
