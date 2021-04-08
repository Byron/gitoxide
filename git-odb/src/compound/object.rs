use crate::loose;

/// An object within an object database with loose or entirely borrowed objects.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[allow(missing_docs)]
pub enum Object<'a> {
    Loose(Box<loose::Object>),
    Borrowed(crate::borrowed::Object<'a>),
}

/// Access
impl<'a> Object<'a> {
    /// Returns the object [`kind`][`git_object::Kind`].
    pub fn kind(&self) -> git_object::Kind {
        match self {
            Object::Borrowed(object) => object.kind,
            Object::Loose(object) => object.kind,
        }
    }

    /// Returns the uncompressed size of the uncompressed object data in bytes.
    pub fn size(&self) -> usize {
        match self {
            Object::Borrowed(object) => object.data.len(),
            Object::Loose(object) => object.size,
        }
    }
}

///
pub mod decode {
    use crate::{compound::Object, loose};

    /// Returned by [`Object::decode()`].
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Decode(#[from] git_object::borrowed::Error),
        #[error(transparent)]
        LooseObject(#[from] loose::object::decode::Error),
    }

    impl<'a> Object<'a> {
        /// Decode an object for accessing individual fields.
        /// _Note_ that this is inefficient for big loose blobs, which should rather be streamed.
        pub fn decode(&mut self) -> Result<git_object::borrowed::Object<'_>, Error> {
            match self {
                Object::Borrowed(object) => object.decode().map_err(Into::into),
                Object::Loose(object) => object.decode().map_err(Into::into),
            }
        }
    }
}

///
pub mod verify {
    use crate::{compound::Object, loose};

    /// Returned by [`Object::verify_checksum()`].
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Pack(#[from] crate::borrowed::verify::Error),
        #[error(transparent)]
        Loose(#[from] loose::object::verify::Error),
    }

    impl<'a> Object<'a> {
        /// Assert whether the actual checksum of this object matches the `desired` one.
        pub fn verify_checksum(&mut self, desired: impl AsRef<git_hash::oid>) -> Result<(), Error> {
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
            32,
            "the object size should not grow unexpectedly"
        );
    }
}
