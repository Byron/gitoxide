use git_hash::ObjectId;
use std::{borrow::Borrow, option::Option::None, sync::Arc};

use crate::store::linked;
use crate::store::loose;

#[allow(clippy::large_enum_variant)]
enum DbState {
    Pack { pack_index: usize, entry_index: u32 },
    Loose { iter: loose::iter::Iter },
}

impl Default for DbState {
    fn default() -> Self {
        DbState::Pack {
            pack_index: 0,
            entry_index: 0,
        }
    }
}

/// An iterator over all objects of a linked database
pub struct AllObjects<Db> {
    db: Db,
    db_index: usize,
    db_state: DbState,
}

impl<Db> AllObjects<Db>
where
    Db: Borrow<linked::Store>,
{
    /// Create a new iterator from a linked database
    pub fn new(db: Db) -> Self {
        let db_index = 0;
        let db_state = {
            let db = &db
                .borrow()
                .dbs
                .get(db_index)
                .expect("at least one db or no linked::Store at all");
            if db.bundles.is_empty() {
                DbState::Loose { iter: db.loose.iter() }
            } else {
                DbState::default()
            }
        };
        AllObjects { db, db_index, db_state }
    }
}

impl<Db> Iterator for AllObjects<Db>
where
    Db: Borrow<linked::Store>,
{
    type Item = Result<ObjectId, loose::iter::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let db = self.db.borrow();
        if self.db_index == db.dbs.len() {
            return None;
        }

        match &mut self.db_state {
            DbState::Pack {
                pack_index,
                entry_index,
            } => {
                let db = &db.dbs[self.db_index];
                match db.bundles.get(*pack_index) {
                    Some(bundle) => {
                        if *entry_index < bundle.index.num_objects() {
                            let oid = bundle.index.oid_at_index(*entry_index).to_owned();
                            *entry_index += 1;
                            Some(Ok(oid))
                        } else {
                            *pack_index += 1;
                            *entry_index = 0;
                            self.next()
                        }
                    }
                    None => {
                        self.db_state = DbState::Loose { iter: db.loose.iter() };
                        self.next()
                    }
                }
            }
            DbState::Loose { iter } => match iter.next() {
                Some(id) => Some(id),
                None => {
                    self.db_index += 1;
                    self.db_state = Default::default();
                    self.next()
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let packed_objects = self.db.borrow().dbs.iter().fold(0usize, |dbc, db| {
            dbc.saturating_add(
                db.bundles
                    .iter()
                    .fold(0, |pc, pack| pc.saturating_add(pack.index.num_objects() as usize)),
            )
        });
        (packed_objects, None)
    }
}

impl linked::Store {
    /// Return an iterator over all objects in all linked databases, database after database, first packed
    /// objects with the 'best' packs first, followed by loose objects.
    /// For specialized iterations, use the `dbs` fields directly as all databases are accessible.
    pub fn iter(&self) -> AllObjects<&linked::Store> {
        AllObjects::new(self)
    }

    /// Like [`iter()`][linked::Store::iter()] but works with this instance living in an [`Arc`]
    ///
    /// Useful in conjunction with `'static threads`.
    pub fn arc_iter(self: &Arc<linked::Store>) -> AllObjects<Arc<linked::Store>> {
        AllObjects::new(Arc::clone(&self))
    }
}
