//! An object database representing a list of [compound databases][compound::Backend] commonly created using _git alternates_.
use crate::store::compound;

/// A database with a list of [compound databases][compound::Backend] created by traversing git `alternates` files.
///
/// It does not contain any objects itself.
pub struct Db {
    /// The compound databases containing the actual objects.
    pub dbs: Vec<compound::Backend>,
}

///
pub mod init;

///
pub mod find;

///
mod write;

///
mod iter;
