use std::{ops::Deref, option::Option::None};

use crate::general::load_index::Snapshot;
use crate::loose;
use git_hash::ObjectId;

use crate::store::general;

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
pub struct AllObjects {
    snapshot: Snapshot,
    loose_db_index: usize,
    db_state: DbState,
}

impl AllObjects {
    /// Create a new iterator from a general database, which will be forced to load all indices eagerly.
    pub fn new(db: &general::Store) -> Result<Self, crate::general::load_index::Error> {
        let mut snapshot = db.collect_snapshot();
        while let Some(new_snapshot) = db.load_one_index(crate::RefreshMode::Never, snapshot.marker)? {
            snapshot = new_snapshot
        }

        let db_index = 0;
        let db_state = {
            if snapshot.indices.is_empty() {
                DbState::Loose {
                    iter: snapshot.loose_dbs.get(db_index).expect("at least one loose db").iter(),
                }
            } else {
                DbState::default()
            }
        };
        Ok(AllObjects {
            snapshot,
            loose_db_index: db_index,
            db_state,
        })
    }
}

impl Iterator for AllObjects {
    type Item = Result<ObjectId, loose::iter::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let snapshot = &self.snapshot;
        if self.loose_db_index == snapshot.indices.len() {
            return None;
        }

        match &mut self.db_state {
            DbState::Pack {
                pack_index,
                entry_index,
            } => match snapshot.indices.get(*pack_index) {
                Some(index) => {
                    if *entry_index < index.num_objects() {
                        let oid = index.oid_at_index(*entry_index).to_owned();
                        *entry_index += 1;
                        Some(Ok(oid))
                    } else {
                        *pack_index += 1;
                        *entry_index = 0;
                        self.next()
                    }
                }
                None => {
                    self.db_state = DbState::Loose {
                        iter: snapshot
                            .loose_dbs
                            .get(self.loose_db_index)
                            .expect("at least one loose db")
                            .iter(),
                    };
                    self.next()
                }
            },
            DbState::Loose { iter } => match iter.next() {
                Some(id) => Some(id),
                None => {
                    self.loose_db_index += 1;
                    let iter = snapshot.loose_dbs.get(self.loose_db_index)?.iter();
                    self.db_state = DbState::Loose { iter };
                    self.next()
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let packed_objects = self
            .snapshot
            .indices
            .iter()
            .fold(0usize, |dbc, index| dbc.saturating_add(index.num_objects() as usize));
        (packed_objects, None)
    }
}

impl<S> super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    /// Return an iterator over all objects in all linked databases, database after database, first packed
    /// objects with the 'best' packs first, followed by loose objects.
    /// For specialized iterations, use the `dbs` fields directly as all databases are accessible.
    pub fn iter(&self) -> Result<AllObjects, general::load_index::Error> {
        AllObjects::new(self.store())
    }
}

impl general::Store {
    /// Like [`Handle::iter()`][super:Handle::iter()].
    pub fn iter(&self) -> Result<AllObjects, general::load_index::Error> {
        AllObjects::new(self)
    }
}
