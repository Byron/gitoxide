use std::path::PathBuf;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Reference<'a> {
    parent: &'a Store,
    /// The path relative to the stores base at which this reference is located
    relative_path: PathBuf,
    state: reference::State,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Store {
    /// The location at which loose references can be found as per conventions of a typical git repository
    pub base: PathBuf,
}

impl Store {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Store { base: path.into() }
    }
}

pub mod reference {
    use crate::{loose::Reference, Kind, Target};
    use bstr::BString;
    use git_hash::ObjectId;

    #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub(crate) enum State {
        Id(ObjectId),
        Path(BString),
    }

    impl<'a> Reference<'a> {
        pub fn kind(&self) -> Kind {
            match self.state {
                State::Path(_) => Kind::Symbolic,
                State::Id(_) => Kind::Peeled,
            }
        }
        pub fn target(&'a self) -> Target<'a> {
            match self.state {
                State::Path(ref path) => Target::Symbolic(path.as_ref()),
                State::Id(ref oid) => Target::Peeled(oid.as_ref()),
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
        use std::path::PathBuf;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Parse(content: BString) {
                    display("{:?} could not be parsed", content)
                }
            }
        }

        impl<'a> Reference<'a> {
            pub fn from_path(
                parent: &'a Store,
                relative_path: impl Into<PathBuf>,
                path_contents: &[u8],
            ) -> Result<Self, Error> {
                Ok(Reference {
                    parent,
                    relative_path: relative_path.into(),
                    state: parse(path_contents)
                        .map_err(|err| err.to_string())
                        .map_err(|_| Error::Parse(path_contents.into()))?
                        .1,
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

        fn parse(bytes: &[u8]) -> IResult<&[u8], State> {
            let is_space = |b: u8| b == b' ';
            if let (path, Some(_ref_prefix)) = opt(terminated(tag("ref: "), take_while(is_space)))(bytes)? {
                map(terminated(take_while(|b| b != b'\r' && b != b'\n'), newline), |path| {
                    State::Path(path.into())
                })(path)
            } else {
                map(terminated(hex_sha1, newline), |hex| {
                    State::Id(ObjectId::from_hex(hex).expect("prior validation"))
                })(bytes)
            }
        }
    }
}
