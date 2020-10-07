//! An object database delegating object access to multiple contained object databases
use crate::{loose, pack};

pub mod object;
pub use object::Object;

pub mod init;
pub mod locate;
mod write;

/// An object database with tiered lookup in Alternates, loose objects and packs.
/// This is a typical git database as used in git repositories.
pub struct Db {
    pub loose: loose::Db,
    pub packs: Vec<pack::Bundle>,
    pub alternates: Vec<Db>,
}
