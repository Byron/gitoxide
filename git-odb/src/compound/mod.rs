//! An object database delegating object access to multiple contained object databases
use crate::{loose, pack};

///
pub mod object;
pub use object::Object;

#[allow(missing_docs)]
pub mod init;
#[allow(missing_docs)]
pub mod locate;
mod write;

/// An object database with tiered lookup in Alternates, loose objects and packs.
/// This is a typical git database as used in git repositories.
pub struct Db {
    /// A loose object database into which new objects are written
    pub loose: loose::Db,
    /// All packs in the `objects/packs` directory
    pub packs: Vec<pack::Bundle>,
    /// Locations of alternate databases
    pub alternates: Vec<Db>,
}
