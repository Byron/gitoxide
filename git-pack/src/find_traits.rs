use crate::{data, find};

/// Describe how object can be located in an object store with built-in facilities to supports packs specifically.
///
/// ## Notes
///
/// Locate effectively needs [generic associated types][issue] to allow a trait for the returned object type.
/// Until then, we will have to make due with explicit types and give them the potentially added features we want.
///
/// [issue]: https://github.com/rust-lang/rust/issues/44265
pub trait Find {
    /// The error returned by [`try_find()`][Find::try_find()]
    type Error: std::error::Error + 'static;

    /// Find an object matching `id` in the database while placing its raw, undecoded data into `buffer`.
    /// A `pack_cache` can be used to speed up subsequent lookups, set it to [`crate::cache::Never`] if the
    /// workload isn't suitable for caching.
    ///
    /// Returns `Some` object if it was present in the database, or the error that occurred during lookup or object
    /// retrieval.
    fn try_find<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl crate::cache::DecodeEntry,
    ) -> Result<Option<data::Object<'a>>, Self::Error>;

    /// Find the packs location where an object with `id` can be found in the database, or `None` if there is no pack
    /// holding the object.
    ///
    /// _Note_ that the object database may have no notion of packs and thus always returns `None`.
    fn location_by_oid(&self, id: impl AsRef<git_hash::oid>, buf: &mut Vec<u8>) -> Option<crate::bundle::Location>;

    /// Find the bundle matching `pack_id`, or `None` if there is no such pack.
    ///
    /// _Note_ that the object database may have no notion of packs and thus always returns `None`.
    fn bundle_by_pack_id(&self, pack_id: u32) -> Option<&crate::Bundle>;

    /// Return the [`find::Entry`] for `location` if it is backed by a pack.
    ///
    /// Note that this is only in the interest of avoiding duplicate work during pack generation.
    /// Pack locations can be obtained from a [`data::Object`].
    ///
    /// # Notes
    ///
    /// Custom implementations might be interested in providing their own meta-data with `object`,
    /// which currently isn't possible as the `Locate` trait requires GATs to work like that.
    fn entry_by_location(&self, location: &crate::bundle::Location) -> Option<find::Entry<'_>>;
}

mod ext {
    use git_object::{BlobRef, CommitRef, CommitRefIter, Kind, ObjectRef, TagRef, TagRefIter, TreeRef, TreeRefIter};

    use crate::{data, find};

    macro_rules! make_obj_lookup {
        ($method:ident, $object_variant:path, $object_kind:path, $object_type:ty) => {
            /// Like [`find(…)`][Self::find()], but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error
            /// while returning the desired object type.
            fn $method<'a>(
                &self,
                id: impl AsRef<git_hash::oid>,
                buffer: &'a mut Vec<u8>,
                pack_cache: &mut impl crate::cache::DecodeEntry,
            ) -> Result<$object_type, find::existing_object::Error<Self::Error>> {
                let id = id.as_ref();
                self.try_find(id, buffer, pack_cache)
                    .map_err(find::existing_object::Error::Find)?
                    .ok_or_else(|| find::existing_object::Error::NotFound {
                        oid: id.as_ref().to_owned(),
                    })
                    .and_then(|o| o.decode().map_err(find::existing_object::Error::Decode))
                    .and_then(|o| match o {
                        $object_variant(o) => return Ok(o),
                        _other => Err(find::existing_object::Error::ObjectKind {
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
                id: impl AsRef<git_hash::oid>,
                buffer: &'a mut Vec<u8>,
                pack_cache: &mut impl crate::cache::DecodeEntry,
            ) -> Result<$object_type, find::existing_iter::Error<Self::Error>> {
                let id = id.as_ref();
                self.try_find(id, buffer, pack_cache)
                    .map_err(find::existing_iter::Error::Find)?
                    .ok_or_else(|| find::existing_iter::Error::NotFound {
                        oid: id.as_ref().to_owned(),
                    })
                    .and_then(|o| {
                        o.$into_iter()
                            .ok_or_else(|| find::existing_iter::Error::ObjectKind {
                                expected: $object_kind,
                            })
                    })
            }
        };
    }

    /// An extension trait with convenience functions.
    pub trait FindExt: super::Find {
        /// Like [`try_find(…)`][super::Find::try_find()], but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error.
        fn find<'a>(
            &self,
            id: impl AsRef<git_hash::oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl crate::cache::DecodeEntry,
        ) -> Result<data::Object<'a>, find::existing::Error<Self::Error>> {
            let id = id.as_ref();
            self.try_find(id, buffer, pack_cache)
                .map_err(find::existing::Error::Find)?
                .ok_or_else(|| find::existing::Error::NotFound {
                    oid: id.as_ref().to_owned(),
                })
        }

        make_obj_lookup!(find_commit, ObjectRef::Commit, Kind::Commit, CommitRef<'a>);
        make_obj_lookup!(find_tree, ObjectRef::Tree, Kind::Tree, TreeRef<'a>);
        make_obj_lookup!(find_tag, ObjectRef::Tag, Kind::Tag, TagRef<'a>);
        make_obj_lookup!(find_blob, ObjectRef::Blob, Kind::Blob, BlobRef<'a>);
        make_iter_lookup!(find_commit_iter, Kind::Blob, CommitRefIter<'a>, into_commit_iter);
        make_iter_lookup!(find_tree_iter, Kind::Tree, TreeRefIter<'a>, into_tree_iter);
        make_iter_lookup!(find_tag_iter, Kind::Tag, TagRefIter<'a>, into_tag_iter);
    }

    impl<T: super::Find> FindExt for T {}
}
pub use ext::FindExt;

mod find_impls {
    use std::ops::Deref;

    use git_hash::oid;

    use crate::{bundle::Location, data::Object, find, Bundle};

    impl<T> super::Find for std::sync::Arc<T>
    where
        T: super::Find,
    {
        type Error = T::Error;

        fn try_find<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl crate::cache::DecodeEntry,
        ) -> Result<Option<Object<'a>>, Self::Error> {
            self.deref().try_find(id, buffer, pack_cache)
        }

        fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
            self.deref().location_by_oid(id, buf)
        }

        fn bundle_by_pack_id(&self, pack_id: u32) -> Option<&Bundle> {
            self.deref().bundle_by_pack_id(pack_id)
        }

        fn entry_by_location(&self, object: &crate::bundle::Location) -> Option<find::Entry<'_>> {
            self.deref().entry_by_location(object)
        }
    }

    impl<T> super::Find for Box<T>
    where
        T: super::Find,
    {
        type Error = T::Error;

        fn try_find<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl crate::cache::DecodeEntry,
        ) -> Result<Option<Object<'a>>, Self::Error> {
            self.deref().try_find(id, buffer, pack_cache)
        }

        fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
            self.deref().location_by_oid(id, buf)
        }

        fn bundle_by_pack_id(&self, pack_id: u32) -> Option<&Bundle> {
            self.deref().bundle_by_pack_id(pack_id)
        }

        fn entry_by_location(&self, location: &crate::bundle::Location) -> Option<find::Entry<'_>> {
            self.deref().entry_by_location(location)
        }
    }
}
