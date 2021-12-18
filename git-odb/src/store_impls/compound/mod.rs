//! An object database delegating object access to multiple contained object databases with loose and packed objects.
use crate::{pack, store_impls::loose};

///
pub mod find;
///
pub mod init;
mod write;

/// A static object database with tiered lookup in packs and loose objects.
/// This is a typical git database as used in git repositories, sans 'alternates'.
/// Note that this ODB won't detect changes on disk and will eagerly map all relevant files. Multipack indices are not supported either.
#[deprecated(since = "0.27.0", note = "superseded by git_odb::Store")]
pub struct Store {
    /// A loose object database into which new objects are written
    pub loose: loose::Store,
    /// All packs in the `objects/packs` directory
    pub bundles: Vec<pack::Bundle>,
}
