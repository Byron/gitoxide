use crate::{data, pack};

/// Describe how object can be located in an object store
///
/// ## Notes
///
/// Locate effectively needs [generic associated types][issue] to allow a trait for the returned object type.
/// Until then, we will have to make due with explicit types and give them the potentially added features we want.
///
/// [issue]: https://github.com/rust-lang/rust/issues/44265
pub trait Find {
    /// The error returned by [`find()`][Find::find()]
    type Error: std::error::Error + 'static;

    /// Find an object matching `id` in the database while placing its raw, undecoded data into `buffer`.
    /// A `pack_cache` can be used to speed up subsequent lookups, set it to [`pack::cache::Never`] if the
    /// workload isn't suitable for caching.
    ///
    /// Returns `Some` object if it was present in the database, or the error that occurred during lookup or object
    /// retrieval.
    fn find<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl crate::pack::cache::DecodeEntry,
    ) -> Result<Option<data::Object<'a>>, Self::Error>;

    /// Return the [`PackEntry`] for `object` if it is backed by a pack.
    ///
    /// Note that this is only in the interest of avoiding duplicate work during pack generation
    /// as the input for this is an already decoded [`data::Object`] that is fully known.
    ///
    /// # Notes
    ///
    /// Custom implementations might be interested in providing their own meta-data with `object`,
    /// which currently isn't possible as the `Locate` trait requires GATs to work like that.
    fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>>;
}

mod ext {
    use crate::{data, find};
    use git_object::{immutable, Kind};

    macro_rules! make_obj_lookup {
        ($method:ident, $object_variant:path, $object_kind:path, $object_type:ty) => {
            /// Like [`find_existing(…)`][Self::find_existing()], but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error
            /// while returning the desired object type.
            fn $method<'a>(
                &self,
                id: impl AsRef<git_hash::oid>,
                buffer: &'a mut Vec<u8>,
                pack_cache: &mut impl crate::pack::cache::DecodeEntry,
            ) -> Result<$object_type, find::existing_object::Error<Self::Error>> {
                let id = id.as_ref();
                self.find(id, buffer, pack_cache)
                    .map_err(|err| find::existing_object::Error::Find(err))?
                    .ok_or_else(|| find::existing_object::Error::NotFound {
                        oid: id.as_ref().to_owned(),
                    })
                    .and_then(|o| {
                        o.decode()
                            .map_err(|err| find::existing_object::Error::Decode(err))
                    })
                    .and_then(|o| match o {
                        $object_variant(o) => return Ok(o),
                        _other => Err(find::existing_object::Error::ObjectKind {
                            expected: $object_kind,
                        }),
                    })
            }
        };
    }

    /// An extension trait with convenience functions.
    pub trait FindExt: super::Find {
        /// Like [`find(…)`][super::Find::find()], but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error.
        fn find_existing<'a>(
            &self,
            id: impl AsRef<git_hash::oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl crate::pack::cache::DecodeEntry,
        ) -> Result<data::Object<'a>, find::existing::Error<Self::Error>> {
            let id = id.as_ref();
            self.find(id, buffer, pack_cache)
                .map_err(|err| find::existing::Error::Find(err))?
                .ok_or_else(|| find::existing::Error::NotFound {
                    oid: id.as_ref().to_owned(),
                })
        }

        make_obj_lookup!(
            find_existing_commit,
            immutable::Object::Commit,
            Kind::Commit,
            immutable::Commit<'a>
        );
        make_obj_lookup!(
            find_existing_tree,
            immutable::Object::Tree,
            Kind::Tree,
            immutable::Tree<'a>
        );
        make_obj_lookup!(find_existing_tag, immutable::Object::Tag, Kind::Tag, immutable::Tag<'a>);
        make_obj_lookup!(
            find_existing_blob,
            immutable::Object::Blob,
            Kind::Blob,
            immutable::Blob<'a>
        );
    }

    impl<T: super::Find> FindExt for T {}
}
pub use ext::FindExt;

///
pub mod existing {
    use git_hash::ObjectId;

    /// The error returned by the [`find_existing(…)`][crate::FindExt::find_existing()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<T: std::error::Error + 'static> {
        #[error(transparent)]
        Find(T),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
    }
}

///
pub mod existing_object {
    use git_hash::ObjectId;
    use git_object::immutable;

    /// The error returned by the various [`find_existing_*`][crate::FindExt::find_existing_commit()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<T: std::error::Error + 'static> {
        #[error(transparent)]
        Find(T),
        #[error(transparent)]
        Decode(immutable::object::decode::Error),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {} something else", .expected)]
        ObjectKind { expected: git_object::Kind },
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)] // TODO: docs
pub struct PackEntry<'a> {
    /// The encoded data of the entry as present in the pack file, including the header followed by compressed data.
    pub data: &'a [u8],
    /// The crc32 hash over the entirety of `data`, or None if the pack file format doesn't support it yet.
    pub crc32: Option<u32>,
    /// The version of the pack file containing `data`
    pub version: pack::data::Version,
}

mod find_impls {
    use crate::{data, data::Object, find::PackEntry, pack};
    use git_hash::oid;
    use std::ops::Deref;

    impl<T> super::Find for std::sync::Arc<T>
    where
        T: super::Find,
    {
        type Error = T::Error;

        fn find<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl pack::cache::DecodeEntry,
        ) -> Result<Option<Object<'a>>, Self::Error> {
            self.deref().find(id, buffer, pack_cache)
        }

        fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
            self.deref().pack_entry(object)
        }
    }

    impl<T> super::Find for Box<T>
    where
        T: super::Find,
    {
        type Error = T::Error;

        fn find<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl pack::cache::DecodeEntry,
        ) -> Result<Option<Object<'a>>, Self::Error> {
            self.deref().find(id, buffer, pack_cache)
        }

        fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
            self.deref().pack_entry(object)
        }
    }
}
