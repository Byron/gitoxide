use crate::Graph;
use gix_hash::oid;
use smallvec::SmallVec;
use std::ops::Index;

impl<'find, T> Graph<'find, T> {
    /// Create a new instance with `find` to retrieve commits and optionally `cache` to accelerate commit access.
    pub fn new<Find, E>(mut find: Find, cache: impl Into<Option<gix_commitgraph::Graph>>) -> Self
    where
        Find:
            for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<Option<gix_object::CommitRefIter<'a>>, E> + 'find,
        E: std::error::Error + Send + Sync + 'static,
    {
        Graph {
            find: Box::new(move |id, buf| {
                find(id, buf).map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)
            }),
            cache: cache.into(),
            set: gix_hashtable::HashMap::default(),
            buf: Vec::new(),
            parent_buf: Vec::new(),
        }
    }

    /// Returns true if `id` has data associated with it, meaning that we processed it already.
    pub fn contains(&self, id: &gix_hash::oid) -> bool {
        self.set.contains_key(id.as_ref())
    }

    /// Returns the data associated with `id` if available.
    pub fn get(&self, id: &gix_hash::oid) -> Option<&T> {
        self.set.get(id)
    }

    /// Returns the data associated with `id` if available as mutable reference.
    pub fn get_mut(&mut self, id: &gix_hash::oid) -> Option<&mut T> {
        self.set.get_mut(id)
    }

    /// Insert `id` into the graph and associate it with `value`, returning the previous value associated with it if it existed.
    pub fn insert(&mut self, id: gix_hash::ObjectId, value: T) -> Option<T> {
        self.set.insert(id, value)
    }

    /// Remove all data from the graph to start over.
    pub fn clear(&mut self) {
        self.set.clear();
    }

    /// Try to lookup `id` and return a handle to it for accessing its data, but don't fail if the commit doesn't exist.
    ///
    /// It's possible that commits don't exist if the repository is shallow.
    pub fn try_lookup(&mut self, id: &gix_hash::oid) -> Result<Option<Commit<'_>>, lookup::Error> {
        try_lookup(id, &mut self.find, self.cache.as_ref(), &mut self.buf)
    }

    /// Lookup `id` and return a handle to it, or fail if it doesn't exist.
    pub fn lookup(&mut self, id: &gix_hash::oid) -> Result<Commit<'_>, lookup::existing::Error> {
        self.try_lookup(id)?.ok_or(lookup::existing::Error::Missing)
    }

    /// Insert the parents of commit named `id` to the graph and associate new parents with data
    /// by calling `new_parent_data(parent_id, committer_timestamp)`, or update existing parents
    /// data with `update_existing(parent_id, &mut existing_data)`.
    /// If `first_parent` is `true`, only the first parent of commits will be looked at.
    pub fn insert_parents(
        &mut self,
        id: &gix_hash::oid,
        mut new_parent_data: impl FnMut(gix_hash::ObjectId, u64) -> T,
        mut update_existing: impl FnMut(gix_hash::ObjectId, &mut T),
        first_parent: bool,
    ) -> Result<(), insert_parents::Error> {
        let commit = self.lookup(id)?;
        let parents: SmallVec<[_; 2]> = commit
            .iter_parents()
            .take(if first_parent { 1 } else { usize::MAX })
            .collect();
        for parent_id in parents {
            let parent_id = parent_id?;
            match self.set.entry(parent_id) {
                gix_hashtable::hash_map::Entry::Vacant(entry) => {
                    let parent = match try_lookup(&parent_id, &mut self.find, self.cache.as_ref(), &mut self.parent_buf)
                        .map_err(|err| insert_parents::Error::Lookup(lookup::existing::Error::Find(err)))?
                    {
                        Some(p) => p,
                        None => continue, // skip missing objects, this is due to shallow clones for instance.
                    };

                    let parent_commit_date = parent.committer_timestamp().unwrap_or_default();
                    entry.insert(new_parent_data(parent_id, parent_commit_date));
                }
                gix_hashtable::hash_map::Entry::Occupied(mut entry) => {
                    update_existing(parent_id, entry.get_mut());
                }
            }
            if first_parent {
                break;
            }
        }
        Ok(())
    }
}

fn try_lookup<'graph>(
    id: &gix_hash::oid,
    find: &mut Box<FindFn<'_>>,
    cache: Option<&'graph gix_commitgraph::Graph>,
    buf: &'graph mut Vec<u8>,
) -> Result<Option<Commit<'graph>>, lookup::Error> {
    if let Some(cache) = cache {
        if let Some(pos) = cache.lookup(id) {
            return Ok(Some(Commit {
                backing: Either::Right((cache, pos)),
            }));
        }
    }
    #[allow(clippy::manual_map)]
    Ok(match find(id, buf)? {
        Some(_) => Some(Commit {
            backing: Either::Left(buf),
        }),
        None => None,
    })
}

impl<'a, 'find, T> Index<&'a gix_hash::oid> for Graph<'find, T> {
    type Output = T;

    fn index(&self, index: &'a oid) -> &Self::Output {
        &self.set[index]
    }
}

///
pub mod lookup {
    /// The error returned by [`try_lookup()`][crate::Graph::try_lookup()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("There was an error looking up a commit")]
        Find(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    }

    ///
    pub mod existing {
        /// The error returned by [`lookup()`][crate::Graph::lookup()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            Find(#[from] super::Error),
            #[error("Commit could not be found")]
            Missing,
        }
    }
}

///
pub mod insert_parents {
    use crate::graph::commit::iter_parents;
    use crate::graph::lookup;

    /// The error returned by [`insert_parents()`][crate::Graph::insert_parents()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Lookup(#[from] lookup::existing::Error),
        #[error("A commit could not be decoded during traversal")]
        Decode(#[from] gix_object::decode::Error),
        #[error(transparent)]
        Parent(#[from] iter_parents::Error),
    }
}

enum Either<T, U> {
    Left(T),
    Right(U),
}

/// A commit that provides access to graph-related information.
pub struct Commit<'graph> {
    backing: Either<&'graph [u8], (&'graph gix_commitgraph::Graph, gix_commitgraph::Position)>,
}

///
pub mod commit {
    use super::Commit;
    use crate::graph::Either;

    impl<'graph> Commit<'graph> {
        /// Return an iterator over the parents of this commit.
        pub fn iter_parents(&self) -> Parents<'graph> {
            let backing = match &self.backing {
                Either::Left(buf) => Either::Left(gix_object::CommitRefIter::from_bytes(buf)),
                Either::Right((cache, pos)) => Either::Right((*cache, cache.commit_at(*pos).iter_parents())),
            };
            Parents { backing }
        }

        /// Returns the timestamp at which this commit was created.
        ///
        /// This is the single-most important date for determining recency of commits.
        /// Note that this can only fail if the commit is backed by the object database *and* parsing fails.
        pub fn committer_timestamp(&self) -> Result<u64, gix_object::decode::Error> {
            Ok(match &self.backing {
                Either::Left(buf) => {
                    gix_object::CommitRefIter::from_bytes(buf)
                        .committer()?
                        .time
                        .seconds_since_unix_epoch as u64
                }
                Either::Right((cache, pos)) => cache.commit_at(*pos).committer_timestamp(),
            })
        }
    }

    /// An iterator over the parents of a commit.
    pub struct Parents<'graph> {
        backing: Either<
            gix_object::CommitRefIter<'graph>,
            (
                &'graph gix_commitgraph::Graph,
                gix_commitgraph::file::commit::Parents<'graph>,
            ),
        >,
    }

    impl<'graph> Iterator for Parents<'graph> {
        type Item = Result<gix_hash::ObjectId, iter_parents::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            match &mut self.backing {
                Either::Left(it) => {
                    for token in it {
                        match token {
                            Ok(gix_object::commit::ref_iter::Token::Tree { .. }) => continue,
                            Ok(gix_object::commit::ref_iter::Token::Parent { id }) => return Some(Ok(id)),
                            Ok(_unused_token) => break,
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                    None
                }
                Either::Right((cache, it)) => it
                    .next()
                    .map(|r| r.map(|pos| cache.id_at(pos).to_owned()).map_err(Into::into)),
            }
        }
    }

    ///
    pub mod iter_parents {
        /// The error returned by the [`Parents`][super::Parents] iterator.
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("An error occurred when parsing commit parents")]
            DecodeCommit(#[from] gix_object::decode::Error),
            #[error("An error occurred when parsing parents from the commit graph")]
            DecodeCommitGraph(#[from] gix_commitgraph::file::commit::Error),
        }
    }
}

pub(crate) type FindFn<'find> = dyn for<'a> FnMut(
        &gix_hash::oid,
        &'a mut Vec<u8>,
    )
        -> Result<Option<gix_object::CommitRefIter<'a>>, Box<dyn std::error::Error + Send + Sync + 'static>>
    + 'find;
