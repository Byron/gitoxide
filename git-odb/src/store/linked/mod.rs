//! An object database representing a list of [compound databases][compound::Store] commonly created using _git alternates_.
use crate::store::compound;

/// A database with a list of [compound databases][compound::Store] created by traversing git `alternates` files.
///
/// It does not contain any objects itself.
///
/// Furthermore, it won't handle multi-pack indices and might be removed at some point in the future to allow focussing on a single database.
#[deprecated(since = "0.27.0", note = "superseded by git_odb::Store")]
pub struct Store {
    /// The compound databases containing the actual objects.
    pub dbs: Vec<compound::Store>,
}

///
pub mod init;

mod handle;

mod find;

///
mod write;

///
mod iter;
