//! Various iterators to traverse a graph

///
pub mod ancestors {
    use crate::{compound, linked, pack};
    use git_hash::ObjectId;
    use std::borrow::Borrow;
    use std::{collections::BTreeSet, collections::VecDeque};

    /// The error used in the iterator implementation of [Iter].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Compound(#[from] compound::locate::Error),
        #[error(transparent)]
        ObjectDecode(#[from] git_object::immutable::decode::Error),
        #[error("Object id {oid} wasn't found in object database")]
        NotFound { oid: ObjectId },
    }

    /// An iterator over the ancestors one or more starting commits
    pub struct Iter<'a, Cache, DbRef> {
        db: DbRef,
        next: VecDeque<ObjectId>,
        buf: Vec<u8>,
        seen: BTreeSet<ObjectId>,
        cache: &'a mut Cache,
    }

    impl<'a, Cache, DbRef> Iter<'a, Cache, DbRef>
    where
        Cache: pack::cache::DecodeEntry,
        DbRef: Borrow<linked::Db>,
    {
        /// Create a new instance.
        ///
        /// * `db` - a way to lookup new object data during traversal
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        /// * `cache` - a way to speedup object database access
        pub fn new(db: DbRef, tips: impl IntoIterator<Item = impl Into<ObjectId>>, cache: &'a mut Cache) -> Self {
            let next: VecDeque<_> = tips.into_iter().map(Into::into).collect();
            let seen = next.iter().cloned().collect();
            Iter {
                db,
                next,
                buf: Vec::with_capacity(4096),
                seen,
                cache,
            }
        }
    }

    impl<'a, Cache, DbRef> Iterator for Iter<'a, Cache, DbRef>
    where
        Cache: pack::cache::DecodeEntry,
        DbRef: Borrow<linked::Db>,
    {
        type Item = Result<ObjectId, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            let res = self.next.pop_front();
            if let Some(oid) = res {
                match self.db.borrow().locate(oid, &mut self.buf, self.cache) {
                    Ok(Some(obj)) => match obj.decode().map_err(Error::from) {
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
                        Err(err) => return Some(Err(err)),
                    },
                    Ok(None) => return Some(Err(Error::NotFound { oid })),
                    Err(err) => return Some(Err(err.into())),
                }
            }
            res.map(Ok)
        }
    }
}
