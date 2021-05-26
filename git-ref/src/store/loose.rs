use std::path::PathBuf;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Reference<'a> {
    parent: &'a Store,
    relative_path: PathBuf,
    state: reference::State,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
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
    use crate::{loose::Reference, Kind};
    use bstr::BString;
    use git_hash::{oid, ObjectId};

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
        pub fn target(&self) -> Option<&oid> {
            match self.state {
                State::Path(_) => None,
                State::Id(ref oid) => Some(oid),
            }
        }
    }

    pub mod decode {
        use crate::loose::{reference::State, Reference, Store};
        use nom::IResult;
        use std::path::PathBuf;

        impl<'a> Reference<'a> {
            pub fn from_path(
                parent: &'a Store,
                relative_path: impl Into<PathBuf>,
                path_contents: &[u8],
            ) -> Result<Self, String> {
                Ok(Reference {
                    parent,
                    relative_path: relative_path.into(),
                    state: parse(path_contents).map_err(|err| err.to_string())?.1,
                })
            }
        }

        fn parse(_bytes: &[u8]) -> IResult<&[u8], State> {
            todo!("parse loose ref bytes into reference")
        }
    }
}
