use std::path::PathBuf;

pub struct Reference<'a> {
    parent: &'a Store,
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
        use crate::loose::Reference;
        use nom::IResult;
        use quick_error::quick_error;
        use std::{io, path::PathBuf};

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

        fn file(bytes: &[u8]) -> IResult<&[u8], Reference<'_>> {
            todo!("parse loose ref bytes into reference")
        }
    }
}
