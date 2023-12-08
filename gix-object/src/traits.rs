use std::io::Write;

use crate::Kind;

/// Writing of objects to a `Write` implementation
pub trait WriteTo {
    /// Write a representation of this instance to `out`.
    fn write_to(&self, out: &mut dyn std::io::Write) -> std::io::Result<()>;

    /// Returns the type of this object.
    fn kind(&self) -> Kind;

    /// Returns the size of this object's representation (the amount
    /// of data which would be written by [`write_to`](Self::write_to)).
    ///
    /// [`size`](Self::size)'s value has no bearing on the validity of
    /// the object, as such it's possible for [`size`](Self::size) to
    /// return a sensible value but [`write_to`](Self::write_to) to
    /// fail because the object was not actually valid in some way.
    fn size(&self) -> u64;

    /// Returns a loose object header based on the object's data
    fn loose_header(&self) -> smallvec::SmallVec<[u8; 28]> {
        crate::encode::loose_header(self.kind(), self.size())
    }
}

impl<T> WriteTo for &T
where
    T: WriteTo,
{
    fn write_to(&self, out: &mut dyn Write) -> std::io::Result<()> {
        <T as WriteTo>::write_to(self, out)
    }

    fn kind(&self) -> Kind {
        <T as WriteTo>::kind(self)
    }

    fn size(&self) -> u64 {
        <T as WriteTo>::size(self)
    }
}

mod find {
    use crate::find;

    /// Check if an object is present in an object store.
    pub trait Exists {
        /// Returns `true` if the object exists in the database.
        fn exists(&self, id: &gix_hash::oid) -> bool;
    }

    /// Find an object in the object store.
    ///
    /// ## Notes
    ///
    /// Find effectively needs [generic associated types][issue] to allow a trait for the returned object type.
    /// Until then, we will have to make due with explicit types and give them the potentially added features we want.
    ///
    /// [issue]: https://github.com/rust-lang/rust/issues/44265
    pub trait Find {
        /// Find an object matching `id` in the database while placing its raw, possibly encoded data into `buffer`.
        ///
        /// Returns `Some` object if it was present in the database, or the error that occurred during lookup or object
        /// retrieval.
        fn try_find<'a>(
            &self,
            id: &gix_hash::oid,
            buffer: &'a mut Vec<u8>,
        ) -> Result<Option<crate::Data<'a>>, find::Error>;
    }

    /// Find the header of an object in the object store.
    pub trait Header {
        /// Find the header of the object matching `id` in the database.
        ///
        /// Returns `Some` header if it was present, or the error that occurred during lookup.
        fn try_header(&self, id: &gix_hash::oid) -> Result<Option<crate::Header>, find::Error>;
    }

    /// A combination of [`Find`] and [`Header`] traits to help with `dyn` trait objects.
    pub trait FindObjectOrHeader: Find + Header {}

    mod _impls {
        use std::{ops::Deref, rc::Rc, sync::Arc};

        use gix_hash::oid;

        use crate::Data;

        impl<T> crate::Exists for &T
        where
            T: crate::Exists,
        {
            fn exists(&self, id: &oid) -> bool {
                (*self).exists(id)
            }
        }

        impl<T> crate::FindObjectOrHeader for T where T: crate::Find + crate::FindHeader {}

        impl<T> crate::Find for &T
        where
            T: crate::Find,
        {
            fn try_find<'a>(&self, id: &oid, buffer: &'a mut Vec<u8>) -> Result<Option<Data<'a>>, crate::find::Error> {
                (*self).try_find(id, buffer)
            }
        }

        impl<T> crate::FindHeader for &T
        where
            T: crate::FindHeader,
        {
            fn try_header(&self, id: &gix_hash::oid) -> Result<Option<crate::Header>, crate::find::Error> {
                (*self).try_header(id)
            }
        }

        impl<T> crate::Exists for Box<T>
        where
            T: crate::Exists,
        {
            fn exists(&self, id: &oid) -> bool {
                self.deref().exists(id)
            }
        }

        impl<T> crate::Exists for Rc<T>
        where
            T: crate::Exists,
        {
            fn exists(&self, id: &oid) -> bool {
                self.deref().exists(id)
            }
        }

        impl<T> crate::Find for Rc<T>
        where
            T: crate::Find,
        {
            fn try_find<'a>(&self, id: &oid, buffer: &'a mut Vec<u8>) -> Result<Option<Data<'a>>, crate::find::Error> {
                self.deref().try_find(id, buffer)
            }
        }

        impl<T> crate::FindHeader for Rc<T>
        where
            T: crate::FindHeader,
        {
            fn try_header(&self, id: &gix_hash::oid) -> Result<Option<crate::Header>, crate::find::Error> {
                self.deref().try_header(id)
            }
        }

        impl<T> crate::Find for Box<T>
        where
            T: crate::Find,
        {
            fn try_find<'a>(&self, id: &oid, buffer: &'a mut Vec<u8>) -> Result<Option<Data<'a>>, crate::find::Error> {
                self.deref().try_find(id, buffer)
            }
        }

        impl<T> crate::FindHeader for Box<T>
        where
            T: crate::FindHeader,
        {
            fn try_header(&self, id: &gix_hash::oid) -> Result<Option<crate::Header>, crate::find::Error> {
                self.deref().try_header(id)
            }
        }

        impl<T> crate::Exists for Arc<T>
        where
            T: crate::Exists,
        {
            fn exists(&self, id: &oid) -> bool {
                self.deref().exists(id)
            }
        }

        impl<T> crate::Find for Arc<T>
        where
            T: crate::Find,
        {
            fn try_find<'a>(&self, id: &oid, buffer: &'a mut Vec<u8>) -> Result<Option<Data<'a>>, crate::find::Error> {
                self.deref().try_find(id, buffer)
            }
        }

        impl<T> crate::FindHeader for Arc<T>
        where
            T: crate::FindHeader,
        {
            fn try_header(&self, id: &gix_hash::oid) -> Result<Option<crate::Header>, crate::find::Error> {
                self.deref().try_header(id)
            }
        }
    }

    mod ext {
        use crate::{
            find, BlobRef, CommitRef, CommitRefIter, Kind, ObjectRef, TagRef, TagRefIter, TreeRef, TreeRefIter,
        };

        macro_rules! make_obj_lookup {
            ($method:ident, $object_variant:path, $object_kind:path, $object_type:ty) => {
                /// Like [`find(…)`][Self::find()], but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error
                /// while returning the desired object type.
                fn $method<'a>(
                    &self,
                    id: &gix_hash::oid,
                    buffer: &'a mut Vec<u8>,
                ) -> Result<$object_type, find::existing_object::Error> {
                    self.try_find(id, buffer)
                        .map_err(find::existing_object::Error::Find)?
                        .ok_or_else(|| find::existing_object::Error::NotFound {
                            oid: id.as_ref().to_owned(),
                        })
                        .and_then(|o| {
                            o.decode()
                                .map_err(|err| find::existing_object::Error::Decode {
                                    source: err,
                                    oid: id.as_ref().to_owned(),
                                })
                        })
                        .and_then(|o| match o {
                            $object_variant(o) => return Ok(o),
                            o => Err(find::existing_object::Error::ObjectKind {
                                oid: id.as_ref().to_owned(),
                                actual: o.kind(),
                                expected: $object_kind,
                            }),
                        })
                }
            };
        }

        macro_rules! make_iter_lookup {
            ($method:ident, $object_kind:path, $object_type:ty, $into_iter:tt) => {
                /// Like [`find(…)`][Self::find()], but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error
                /// while returning the desired iterator type.
                fn $method<'a>(
                    &self,
                    id: &gix_hash::oid,
                    buffer: &'a mut Vec<u8>,
                ) -> Result<$object_type, find::existing_iter::Error> {
                    self.try_find(id, buffer)
                        .map_err(find::existing_iter::Error::Find)?
                        .ok_or_else(|| find::existing_iter::Error::NotFound {
                            oid: id.as_ref().to_owned(),
                        })
                        .and_then(|o| {
                            o.$into_iter()
                                .ok_or_else(|| find::existing_iter::Error::ObjectKind {
                                    oid: id.as_ref().to_owned(),
                                    actual: o.kind,
                                    expected: $object_kind,
                                })
                        })
                }
            };
        }

        /// An extension trait with convenience functions.
        pub trait HeaderExt: super::Header {
            /// Like [`try_header(…)`](super::Header::try_header()), but flattens the `Result<Option<_>>` into a single `Result` making a non-existing header an error.
            fn header(&self, id: &gix_hash::oid) -> Result<crate::Header, find::existing::Error> {
                self.try_header(id)
                    .map_err(find::existing::Error::Find)?
                    .ok_or_else(|| find::existing::Error::NotFound { oid: id.to_owned() })
            }
        }

        /// An extension trait with convenience functions.
        pub trait FindExt: super::Find {
            /// Like [`try_find(…)`](super::Find::try_find()), but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error.
            fn find<'a>(
                &self,
                id: &gix_hash::oid,
                buffer: &'a mut Vec<u8>,
            ) -> Result<crate::Data<'a>, find::existing::Error> {
                self.try_find(id, buffer)
                    .map_err(find::existing::Error::Find)?
                    .ok_or_else(|| find::existing::Error::NotFound { oid: id.to_owned() })
            }

            make_obj_lookup!(find_commit, ObjectRef::Commit, Kind::Commit, CommitRef<'a>);
            make_obj_lookup!(find_tree, ObjectRef::Tree, Kind::Tree, TreeRef<'a>);
            make_obj_lookup!(find_tag, ObjectRef::Tag, Kind::Tag, TagRef<'a>);
            make_obj_lookup!(find_blob, ObjectRef::Blob, Kind::Blob, BlobRef<'a>);
            make_iter_lookup!(find_commit_iter, Kind::Commit, CommitRefIter<'a>, try_into_commit_iter);
            make_iter_lookup!(find_tree_iter, Kind::Tree, TreeRefIter<'a>, try_into_tree_iter);
            make_iter_lookup!(find_tag_iter, Kind::Tag, TagRefIter<'a>, try_into_tag_iter);
        }

        impl<T: super::Find + ?Sized> FindExt for T {}
    }
    pub use ext::{FindExt, HeaderExt};
}
pub use find::*;
