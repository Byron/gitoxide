//! An object database delegating object access to multiple contained object databases with loose and packed objects.
use crate::{loose, pack};

///
pub mod object;
pub use object::Object;

///
pub mod init;
///
pub mod locate;
mod write;

/// An object database with tiered lookup packs and loose objects.
/// This is a typical git database as used in git repositories, sans 'alternates'.
pub struct Db {
    /// A loose object database into which new objects are written
    pub loose: loose::Db,
    /// All packs in the `objects/packs` directory
    pub packs: Vec<pack::Bundle>,
}
