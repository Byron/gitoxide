use crate::{loose, pack};

pub struct Db {
    pub loose: loose::Db,
    pub packs: Vec<pack::Bundle>,
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
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Decode(err: git_object::borrowed::Error) {
                    display("An object could not be decoded")
                    source(err)
                    from()
                }
                LooseObject(err: loose::object::decode::Error) {
                    display("A loose object could not be read")
                    source(err)
                    from()
                }
            }
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

mod init;
mod locate;
mod write;
