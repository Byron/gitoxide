use crate::{linked, loose};
use git_hash::ObjectId;
use std::{borrow::Borrow, option::Option::None, sync::Arc};

#[allow(clippy::large_enum_variant)]
enum DbState {
    Pack { pack_index: usize, entry_index: u32 },
    Loose { iter: loose::db::iter::Type },
}

/// An iterator over all objects of a linked database
pub struct AllObjects<Db> {
    db: Db,
    db_index: usize,
    db_state: DbState,
}

impl<Db> AllObjects<Db>
where
    Db: Borrow<linked::Db>,
{
    /// Create a new iterator from a linked database
    pub fn new(db: Db) -> Self {
        let db_index = 0;
        let db_state = {
            let db = &db
                .borrow()
                .dbs
                .get(db_index)
                .expect("at least one db or no linked::Db at all");
            if db.packs.is_empty() {
                DbState::Loose { iter: db.loose.iter() }
            } else {
                DbState::Pack {
                    pack_index: 0,
                    entry_index: 0,
                }
            }
        };
        AllObjects { db, db_index, db_state }
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let packed_objects = self.db.borrow().dbs.iter().fold(0usize, |dbc, db| {
            dbc.saturating_add(
                db.packs
                    .iter()
                    .fold(0, |pc, pack| pc.saturating_add(pack.pack.num_objects() as usize)),
            )
        });
        (packed_objects, None)
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
