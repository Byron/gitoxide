//! An object database representing a list of [compound databases][compound::Db] commonly created using _git alternates_.
use crate::compound;

/// A database with a list of [compound databases][compound::Db] created by traversing git `alternates` files.
///
/// It does not contain any objects itself.
pub struct Db {
    /// The compound databases containing the actual objects.
    pub dbs: Vec<compound::Db>,
}

///
pub mod init;

///
pub mod locate;

///
mod write;

///
mod iter {
    use crate::linked;
    use crate::loose;
    use git_hash::ObjectId;
    use std::{borrow::Borrow, sync::Arc};

    /// An iterator over all objects of a linked database
    pub struct AllObjects<Db> {
        db: Db,
    }

    impl<Db> AllObjects<Db>
    where
        Db: Borrow<linked::Db>,
    {
        /// Create a new iterator from a linked database
        pub fn new(db: Db) -> Self {
            AllObjects { db }
        }
    }

    impl<Db> Iterator for AllObjects<Db>
    where
        Db: Borrow<linked::Db>,
    {
        type Item = Result<ObjectId, loose::db::iter::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    impl linked::Db {
        /// Return an iterator over all objects in all linked databases, database after database, first packed
        /// objects with the 'best' packs first, followed by loose objects.
        /// For specialized iterations, use the `dbs` fields directly as all databases are accessible.
        pub fn iter(&self) -> AllObjects<&linked::Db> {
            AllObjects::new(self)
        }

        /// Like [`iter()`][linked::Db::iter()] but works with this instance living in an [`Arc`]
        ///
        /// Useful in conjunction with `'static threads`.
        pub fn arc_iter(self: &Arc<linked::Db>) -> AllObjects<Arc<linked::Db>> {
            AllObjects::new(Arc::clone(&self))
        }
    }
}
