//! Git stores all of its data as _Objects_, which are data along with a hash over all data. Storing objects efficiently
//! is what git packs are concerned about.
//!
//! Packs consist of [data files][data::File] and [index files][index::File]. The latter can be generated from a data file
//! and make accessing objects within a pack feasible.
//!
//! A [Bundle] conveniently combines a data pack alongside its index to allow [finding][Find] objects or verifying the pack.
//! Objects returned by `.find(…)` are [objects][gix_object::Data] which know their pack location in order to speed up
//! various common operations like creating new packs from existing ones.
//!
//! When traversing all objects in a pack, a _delta tree acceleration structure_ can be built from pack data or an index
//! in order to decompress packs in parallel and without any waste.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

///
pub mod bundle;
/// A bundle of pack data and the corresponding pack index
pub struct Bundle {
    /// The pack file corresponding to `index`
    pub pack: data::File,
    /// The index file corresponding to `pack`
    pub index: index::File,
}

///
pub mod find;

///
pub mod cache;
///
pub mod data;

mod find_traits;
pub use find_traits::{Find, FindExt};

///
pub mod index;
///
pub mod multi_index;

///
pub mod verify;

mod mmap {
    use std::path::Path;

    pub fn read_only(path: &Path) -> std::io::Result<memmap2::Mmap> {
        let file = std::fs::File::open(path)?;
        // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
        #[allow(unsafe_code)]
        unsafe {
            memmap2::Mmap::map(&file)
        }
    }
}

use std::convert::TryInto;

#[inline]
fn read_u32(b: &[u8]) -> u32 {
    u32::from_be_bytes(b.try_into().unwrap())
}

#[inline]
fn read_u64(b: &[u8]) -> u64 {
    u64::from_be_bytes(b.try_into().unwrap())
}
