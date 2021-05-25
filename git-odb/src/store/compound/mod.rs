//! An object database delegating object access to multiple contained object databases with loose and packed objects.
use crate::store::loose;
use git_pack::pack;

///
pub mod find;
///
pub mod init;
mod write;

/// An object database with tiered lookup packs and loose objects.
/// This is a typical git database as used in git repositories, sans 'alternates'.
pub struct Backend {
    /// A loose object database into which new objects are written
    pub loose: loose::Backend,
    /// All packs in the `objects/packs` directory
    pub bundles: Vec<pack::Bundle>,
}
