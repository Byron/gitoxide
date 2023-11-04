/// An object header informing about object properties, without it being fully decoded in the process.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Header {
    /// The object was not packed, but is currently located in the loose object portion of the database.
    ///
    /// As packs are searched first, this means that in this very moment, the object whose header we retrieved is unique
    /// in the object database.
    Loose {
        /// The kind of the object.
        kind: gix_object::Kind,
        /// The size of the object's data in bytes.
        size: u64,
    },
    /// The object was present in a pack.
    ///
    /// Note that this does not imply it is unique in the database, as it might be present in more than one pack and even
    /// as loose object.
    Packed(gix_pack::data::decode::header::Outcome),
}

mod header {
    use super::Header;

    impl Header {
        /// Return the object kind of the object we represent.
        pub fn kind(&self) -> gix_object::Kind {
            match self {
                Header::Packed(out) => out.kind,
                Header::Loose { kind, .. } => *kind,
            }
        }
        /// Return the size of the object in bytes.
        pub fn size(&self) -> u64 {
            match self {
                Header::Packed(out) => out.object_size,
                Header::Loose { size, .. } => *size,
            }
        }
        /// Return the amount of deltas decoded to obtain this header, if the object was packed.
        pub fn num_deltas(&self) -> Option<u32> {
            match self {
                Header::Packed(out) => out.num_deltas.into(),
                Header::Loose { .. } => None,
            }
        }
    }

    impl From<gix_pack::data::decode::header::Outcome> for Header {
        fn from(packed_header: gix_pack::data::decode::header::Outcome) -> Self {
            Header::Packed(packed_header)
        }
    }

    impl From<(u64, gix_object::Kind)> for Header {
        fn from((object_size, kind): (u64, gix_object::Kind)) -> Self {
            Header::Loose {
                kind,
                size: object_size,
            }
        }
    }
}
