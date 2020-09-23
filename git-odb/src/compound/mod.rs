use crate::{loose, pack};

pub struct Db {
    pub loose: loose::Db,
    pub packs: Vec<pack::Bundle>,
    pub alternate: Option<Box<Db>>,
}

pub mod object {
    use crate::loose;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    pub enum Object<'a> {
        Loose(loose::Object),
        Borrowed(crate::borrowed::Object<'a>),
    }

    /// Access
    impl<'a> Object<'a> {
        pub fn kind(&self) -> git_object::Kind {
            match self {
                Object::Borrowed(object) => object.kind,
                Object::Loose(object) => object.kind,
            }
        }

        pub fn size(&self) -> usize {
            match self {
                Object::Borrowed(object) => object.data.len(),
                Object::Loose(object) => object.size,
            }
        }
    }

    pub mod decode {
        use crate::{compound::Object, loose};

        #[derive(thiserror::Error, Debug)]
        pub enum Error {
            #[error(transparent)]
            Decode(#[from] git_object::borrowed::Error),
            #[error(transparent)]
            LooseObject(#[from] loose::object::decode::Error),
        }

        impl<'a> Object<'a> {
            pub fn decode(&mut self) -> Result<git_object::borrowed::Object<'_>, Error> {
                match self {
                    Object::Borrowed(object) => object.decode().map_err(Into::into),
                    Object::Loose(object) => object.decode().map_err(Into::into),
                }
            }
        }
    }

    pub mod verify {
        use crate::{compound::Object, loose};
        use git_object::borrowed;

        #[derive(thiserror::Error, Debug)]
        pub enum Error {
            #[error(transparent)]
            Pack(#[from] crate::borrowed::verify::Error),
            #[error(transparent)]
            Loose(#[from] loose::object::verify::Error),
        }

        impl<'a> Object<'a> {
            pub fn verify_checksum(&mut self, desired: borrowed::Id<'_>) -> Result<(), Error> {
                match self {
                    Object::Borrowed(object) => object.verify_checksum(desired).map_err(Into::into),
                    Object::Loose(object) => object.verify_checksum(desired).map_err(Into::into),
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn size_in_memory() {
            assert_eq!(
                std::mem::size_of::<Object<'_>>(),
                856,
                "the object size should not grow unexpectedly"
            );
        }
    }
}
pub use object::Object;

pub mod init;
pub mod locate;
mod write;
