use std::path::PathBuf;

pub struct Reference<'a> {
    parent: &'a Store,
    path: PathBuf,
    state: reference::State,
}

pub struct Store {
    /// The location at which loose references can be found as per conventions of a typical git repository
    pub base: PathBuf,
}

pub mod reference {
    use crate::loose::Reference;
    use crate::Kind;
    use bstr::BString;
    use git_hash::ObjectId;

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
    }

    pub mod decode {
        use crate::loose::reference::State;
        use crate::loose::{Reference, Store};
        use nom::IResult;
        use quick_error::quick_error;
        use std::{io, path::PathBuf};

        impl<'a> Reference<'a> {
            pub fn from_path(parent: &'a Store, path: impl Into<PathBuf>) -> Result<Self, Error> {
                let path = path.into();
                let state = {
                    let contents = std::fs::read(&path).map_err(|err| Error::Io(err, path.clone()))?;
                    parse(&contents).map_err(|err| Error::Parse(err.to_string()))?.1
                };
                Ok(Reference { parent, path, state })
            }
        }

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Io(err: io::Error, path: PathBuf) {
                    display("Could not access '{}' for reading", path.display())
                    source(err)
                }
                Parse(err: String) {
                    display("parsing failed: {}", err)
                }
            }
        }

        fn parse(bytes: &[u8]) -> IResult<&[u8], State> {
            todo!("parse loose ref bytes into reference")
        }
    }
}
