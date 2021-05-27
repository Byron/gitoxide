use crate::{loose::Reference, Kind, Target};
use bstr::BString;
use git_hash::ObjectId;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub(crate) enum State {
    Id(ObjectId),
    ValidatedPath(BString),
}

impl<'a> Reference<'a> {
    pub fn kind(&self) -> Kind {
        match self.state {
            State::ValidatedPath(_) => Kind::Symbolic,
            State::Id(_) => Kind::Peeled,
        }
    }
    pub fn target(&'a self) -> Target<'a> {
        match self.state {
            State::ValidatedPath(ref path) => Target::Symbolic(path.as_ref()),
            State::Id(ref oid) => Target::Peeled(oid.as_ref()),
        }
    }
}

pub mod peel {
    use crate::{
        loose::{self, find, reference::State, Reference},
        Target,
    };
    use bstr::ByteSlice;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            FindExisting(err: find::existing::Error) {
                display("Could not resolve symbolic reference name that is expected to exist")
                from()
                source(err)
            }
            Decode(err: loose::reference::decode::Error) {
                display("The reference could not be decoded.")
                from()
                source(err)
            }
        }
    }

    impl<'a> Reference<'a> {
        pub fn peel_one(&mut self) -> Option<Result<Target<'_>, Error>> {
            match &self.state {
                State::Id(_) => None,
                State::ValidatedPath(relative_path) => {
                    let path = relative_path.to_path_lossy();
                    match self.parent.find_one_with_verified_input(path.as_ref()) {
                        Ok(Some(next)) => {
                            self.relative_path = next.relative_path;
                            self.state = next.state;
                            return Some(Ok(self.target()));
                        }
                        Ok(None) => {
                            return Some(Err(Error::FindExisting(find::existing::Error::NotFound(
                                path.into_owned(),
                            ))))
                        }
                        Err(err) => return Some(Err(Error::FindExisting(find::existing::Error::Find(err)))),
                    }
                }
            }
        }
    }
}

pub mod decode {
    use crate::loose::{reference::State, Reference, Store};
    use bstr::BString;
    use git_hash::ObjectId;
    use nom::{
        branch::alt,
        bytes::complete::take_while,
        bytes::complete::{tag, take_while_m_n},
        combinator::{map, opt},
        sequence::terminated,
        IResult,
    };
    use quick_error::quick_error;
    use std::convert::TryInto;
    use std::{convert::TryFrom, path::PathBuf};

    enum MaybeUnsafeState {
        Id(ObjectId),
        UnvalidatedPath(BString),
    }

    quick_error! {
        #[derive(Debug)]
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

    fn is_hex_digit_lc(b: u8) -> bool {
        matches!(b, b'0'..=b'9' | b'a'..=b'f')
    }

    fn hex_sha1(i: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while_m_n(40usize, 40, is_hex_digit_lc)(i)
    }

    fn newline(i: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((tag(b"\r\n"), tag(b"\n")))(i)
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
}
