mod count {
    use crate::data;
    use git_hash::ObjectId;

    /// An item representing a future Entry in the leanest way possible.
    ///
    /// One can expect to have one of these in memory when building big objects, so smaller is better here.
    /// They should contain everything of importance to generate a pack as fast as possible.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Count {
        /// The hash of the object to write
        pub id: ObjectId,
        /// A way to locate a pack entry in the object database, only available if the object is in a pack.
        pub entry_pack_location: Option<crate::bundle::Location>,
    }

    impl Count {
        /// Create a new instance from the given `oid` and its corresponding git `obj`ect data.
        pub fn from_data(oid: impl Into<ObjectId>, obj: &data::Object<'_>) -> Self {
            Count {
                id: oid.into(),
                entry_pack_location: obj.pack_location.clone(),
            }
        }
    }
}
#[doc(inline)]
pub use count::Count;

///
pub mod entry;
#[doc(inline)]
pub use entry::Entry;

///
pub mod objects_to_entries;
pub use objects_to_entries::objects_to_entries_iter;

///
pub mod count_objects;
pub use count_objects::count_objects_iter;

///
pub mod entries_to_bytes;
pub use entries_to_bytes::EntriesToBytesIter;
