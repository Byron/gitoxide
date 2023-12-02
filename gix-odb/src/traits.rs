use std::io;

use gix_object::WriteTo;

use crate::find;

/// Describe the capability to write git objects into an object store.
pub trait Write {
    /// Write objects using the intrinsic kind of [`hash`][gix_hash::Kind] into the database,
    /// returning id to reference it in subsequent reads.
    fn write(&self, object: &dyn WriteTo) -> Result<gix_hash::ObjectId, crate::write::Error> {
        let mut buf = Vec::with_capacity(2048);
        object.write_to(&mut buf)?;
        self.write_stream(object.kind(), buf.len() as u64, &mut buf.as_slice())
    }
    /// As [`write`][Write::write], but takes an [`object` kind][gix_object::Kind] along with its encoded bytes.
    fn write_buf(&self, object: gix_object::Kind, mut from: &[u8]) -> Result<gix_hash::ObjectId, crate::write::Error> {
        self.write_stream(object, from.len() as u64, &mut from)
    }
    /// As [`write`][Write::write], but takes an input stream.
    /// This is commonly used for writing blobs directly without reading them to memory first.
    fn write_stream(
        &self,
        kind: gix_object::Kind,
        size: u64,
        from: &mut dyn io::Read,
    ) -> Result<gix_hash::ObjectId, crate::write::Error>;
}

/// A way to obtain object properties without fully decoding it.
pub trait Header {
    /// Try to read the header of the object associated with `id` or return `None` if it could not be found.
    fn try_header(&self, id: &gix_hash::oid) -> Result<Option<find::Header>, gix_object::find::Error>;
}

mod _impls {
    use std::{io::Read, ops::Deref, rc::Rc, sync::Arc};

    use gix_hash::{oid, ObjectId};
    use gix_object::{Kind, WriteTo};

    use crate::find::Header;

    impl<T> crate::Write for &T
    where
        T: crate::Write,
    {
        fn write(&self, object: &dyn WriteTo) -> Result<ObjectId, crate::write::Error> {
            (*self).write(object)
        }

        fn write_buf(&self, object: Kind, from: &[u8]) -> Result<ObjectId, crate::write::Error> {
            (*self).write_buf(object, from)
        }

        fn write_stream(&self, kind: Kind, size: u64, from: &mut dyn Read) -> Result<ObjectId, crate::write::Error> {
            (*self).write_stream(kind, size, from)
        }
    }

    impl<T> crate::Write for Arc<T>
    where
        T: crate::Write,
    {
        fn write(&self, object: &dyn WriteTo) -> Result<ObjectId, crate::write::Error> {
            self.deref().write(object)
        }

        fn write_buf(&self, object: Kind, from: &[u8]) -> Result<ObjectId, crate::write::Error> {
            self.deref().write_buf(object, from)
        }

        fn write_stream(&self, kind: Kind, size: u64, from: &mut dyn Read) -> Result<ObjectId, crate::write::Error> {
            self.deref().write_stream(kind, size, from)
        }
    }

    impl<T> crate::Write for Rc<T>
    where
        T: crate::Write,
    {
        fn write(&self, object: &dyn WriteTo) -> Result<ObjectId, crate::write::Error> {
            self.deref().write(object)
        }

        fn write_buf(&self, object: Kind, from: &[u8]) -> Result<ObjectId, crate::write::Error> {
            self.deref().write_buf(object, from)
        }

        fn write_stream(&self, kind: Kind, size: u64, from: &mut dyn Read) -> Result<ObjectId, crate::write::Error> {
            self.deref().write_stream(kind, size, from)
        }
    }

    impl<T> crate::Header for &T
    where
        T: crate::Header,
    {
        fn try_header(&self, id: &oid) -> Result<Option<Header>, gix_object::find::Error> {
            (*self).try_header(id)
        }
    }

    impl<T> crate::Header for Rc<T>
    where
        T: crate::Header,
    {
        fn try_header(&self, id: &oid) -> Result<Option<Header>, gix_object::find::Error> {
            self.deref().try_header(id)
        }
    }

    impl<T> crate::Header for Arc<T>
    where
        T: crate::Header,
    {
        fn try_header(&self, id: &oid) -> Result<Option<Header>, gix_object::find::Error> {
            self.deref().try_header(id)
        }
    }
}

mod ext {
    use crate::find;
    /// An extension trait with convenience functions.
    pub trait HeaderExt: super::Header {
        /// Like [`try_header(…)`][super::Header::try_header()], but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error.
        fn header(&self, id: impl AsRef<gix_hash::oid>) -> Result<find::Header, gix_object::find::existing::Error> {
            let id = id.as_ref();
            self.try_header(id)
                .map_err(gix_object::find::existing::Error::Find)?
                .ok_or_else(|| gix_object::find::existing::Error::NotFound { oid: id.to_owned() })
        }
    }

    impl<T: super::Header> HeaderExt for T {}
}
pub use ext::HeaderExt;
