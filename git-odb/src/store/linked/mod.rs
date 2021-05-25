//! An object database representing a list of [compound databases][compound::Store] commonly created using _git alternates_.
use crate::store::compound;

/// A database with a list of [compound databases][compound::Store] created by traversing git `alternates` files.
///
/// It does not contain any objects itself.
pub struct Store {
    /// The compound databases containing the actual objects.
    pub dbs: Vec<compound::Store>,
}

///
pub mod init;

///
pub mod find;

///
mod write;

///
mod iter;
