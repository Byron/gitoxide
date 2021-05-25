#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

//! Git stores all of its data as _Objects_, which are data along with a hash over all data. Storing objects efficiently
//! is what git packs are concerned about.
//!
//! Packs consist of [data files][data::File] and [index files][index::File]. The latter can be generated from a data file
//! and make accessing objects within a pack feasible.
//!
//! A [Bundle] conveniently combines a data pack alongside its index to allow [finding][Find] objects or verifying the pack.
//! Objects returned by `.find(…)` are [objects][data::Object] which know their pack location in order to speed up
//! various common operations like creating new packs from existing ones.
//!
//! When traversing all objects in a pack, a [Tree acceleration structure][tree::Tree] can be built from pack data or an index
//! in order to decompress packs in parallel and without any waste.

///
pub mod bundle;
///
pub mod find;
#[doc(inline)]
pub use crate::bundle::Bundle;

///
pub mod cache;
///
pub mod data;
///
pub mod index;
///
pub mod tree;

pub use find::{Find, FindExt};

///
pub mod loose;
