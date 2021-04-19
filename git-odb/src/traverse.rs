//! Various iterators to traverse a graph

///
pub mod ancestors {
    use crate::{compound, linked, pack};
    use git_hash::ObjectId;
    use std::collections::BTreeSet;
    use std::{collections::VecDeque, iter::FromIterator};

    /// The error used in the iterator implementation of [Iter].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Compound(#[from] compound::locate::Error),
        #[error(transparent)]
        ObjectDecode(#[from] compound::object::decode::Error),
        #[error("Object id {oid} wasn't found in object database")]
        NotFound { oid: ObjectId },
    }

    /// An iterator over the ancestors of a single starting point
    pub struct Iter<'a, Cache> {
        db: &'a linked::Db,
        next: VecDeque<ObjectId>,
        buf: Vec<u8>,
        seen: BTreeSet<ObjectId>,
        cache: &'a mut Cache,
    }

    impl<'a, Cache> Iter<'a, Cache>
    where
        Cache: pack::cache::DecodeEntry,
    {
        /// Create a new instance.
        ///
        /// * `db` - a way to lookup new object data during traversal
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        /// * `cache` - a way to speedup object database access
        pub fn new(db: &'a linked::Db, tips: impl Iterator<Item = impl Into<ObjectId>>, cache: &'a mut Cache) -> Self {
            let next = VecDeque::from_iter(tips.map(Into::into));
            let seen = BTreeSet::from_iter(next.iter().cloned());
            Iter {
                db,
                next,
                buf: Vec::with_capacity(4096),
                seen,
                cache,
            }
        }
    }

    impl<'a, Cache> Iterator for Iter<'a, Cache>
    where
        Cache: pack::cache::DecodeEntry,
    {
        type Item = Result<ObjectId, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            let res = self.next.pop_front();
            if let Some(oid) = res {
                match self.db.locate(oid, &mut self.buf, self.cache) {
                    Ok(Some(mut obj)) => match obj.decode().map_err(Error::from) {
                        Ok(obj) => {
                            if let Some(commit) = obj.as_commit() {
                                for parent_id in commit.parents() {
                                    let was_inserted = self.seen.insert(parent_id);
                                    if was_inserted {
                                        self.next.push_back(parent_id);
                                    }
                                }
                            }
                        }
                        Err(err) => return Some(Err(err.into())),
                    },
                    Ok(None) => return Some(Err(Error::NotFound { oid })),
                    Err(err) => return Some(Err(err.into())),
                }
            }
            res.map(Ok)
        }
    }
}
