use std::{fmt::Formatter, ops::Index};

use gix_hash::oid;
use smallvec::SmallVec;

use crate::Graph;

/// A mapping between an object id and arbitrary data, and produced when calling [`Graph::detach()`].
pub type IdMap<T> = gix_hashtable::HashMap<gix_hash::ObjectId, T>;

///
#[allow(clippy::empty_docs)]
pub mod commit;

mod errors {
    ///
    #[allow(clippy::empty_docs)]
    pub mod insert_parents {
        use crate::graph::commit::iter_parents;

        /// The error returned by [`insert_parents()`](crate::Graph::insert_parents()).
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            Lookup(#[from] gix_object::find::existing_iter::Error),
            #[error("A commit could not be decoded during traversal")]
            Decode(#[from] gix_object::decode::Error),
            #[error(transparent)]
            Parent(#[from] iter_parents::Error),
        }
    }

    ///
    #[allow(clippy::empty_docs)]
    pub mod try_lookup_or_insert_default {
        use crate::graph::commit::to_owned;

        /// The error returned by [`try_lookup_or_insert_default()`](crate::Graph::try_lookup_or_insert_default()).
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            Lookup(#[from] gix_object::find::existing_iter::Error),
            #[error(transparent)]
            ToOwned(#[from] to_owned::Error),
        }
    }
}
pub use errors::{insert_parents, try_lookup_or_insert_default};
use gix_date::SecondsSinceUnixEpoch;

/// The generation away from the HEAD of graph, useful to limit algorithms by topological depth as well.
///
/// 0 would mean the starting point of the hierarchy, and 1 their parents.
/// This number is only available natively if there is a commit-graph.
pub type Generation = u32;

impl<'find, T: std::fmt::Debug> std::fmt::Debug for Graph<'find, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.map, f)
    }
}

impl<'find, T: Default> Graph<'find, T> {
    /// Lookup `id` without failing if the commit doesn't exist, and assure that `id` is inserted into our set.
    /// If it wasn't, associate it with the default value. Assure `update_data(data)` gets run.
    /// Return the commit when done.
    /// Note that none of the data updates happen if there was no commit named `id`.
    pub fn try_lookup_or_insert(
        &mut self,
        id: gix_hash::ObjectId,
        update_data: impl FnOnce(&mut T),
    ) -> Result<Option<LazyCommit<'_>>, try_lookup_or_insert_default::Error> {
        self.try_lookup_or_insert_default(id, T::default, update_data)
    }
}

/// Access and mutation
impl<'find, T> Graph<'find, T> {
    /// Returns true if `id` has data associated with it, meaning that we processed it already.
    pub fn contains(&self, id: &gix_hash::oid) -> bool {
        self.map.contains_key(id.as_ref())
    }

    /// Returns the data associated with `id` if available.
    pub fn get(&self, id: &gix_hash::oid) -> Option<&T> {
        self.map.get(id)
    }

    /// Returns the data associated with `id` if available as mutable reference.
    pub fn get_mut(&mut self, id: &gix_hash::oid) -> Option<&mut T> {
        self.map.get_mut(id)
    }

    /// Insert `id` into the graph and associate it with `value`, returning the previous value associated with it if it existed.
    pub fn insert(&mut self, id: gix_hash::ObjectId, value: T) -> Option<T> {
        self.map.insert(id, value)
    }

    /// Remove all data from the graph to start over.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Insert the parents of commit named `id` to the graph and associate new parents with data
    /// by calling `new_parent_data(parent_id, committer_timestamp)`, or update existing parents
    /// data with `update_existing(parent_id, &mut existing_data)`.
    /// If `first_parent` is `true`, only the first parent of commits will be looked at.
    pub fn insert_parents(
        &mut self,
        id: &gix_hash::oid,
        new_parent_data: &mut dyn FnMut(gix_hash::ObjectId, SecondsSinceUnixEpoch) -> T,
        update_existing: &mut dyn FnMut(gix_hash::ObjectId, &mut T),
        first_parent: bool,
    ) -> Result<(), insert_parents::Error> {
        let commit = self.lookup(id)?;
        let parents: SmallVec<[_; 2]> = commit.iter_parents().collect();
        for parent_id in parents {
            let parent_id = parent_id?;
            match self.map.entry(parent_id) {
                gix_hashtable::hash_map::Entry::Vacant(entry) => {
                    let parent = match try_lookup(&parent_id, &*self.find, self.cache.as_ref(), &mut self.parent_buf)? {
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

    /// Turn ourselves into the underlying graph structure, which is a mere mapping between object ids and their data.
    pub fn detach(self) -> IdMap<T> {
        self.map
    }
}

/// Initialization
impl<'find, T> Graph<'find, T> {
    /// Create a new instance with `objects` to retrieve commits and optionally `cache` to accelerate commit access.
    ///
    /// ### Performance
    ///
    /// `find` should be optimized to access the same object repeatedly, ideally with an object cache to keep the last couple of
    /// most recently used commits.
    /// Furthermore, **none-existing commits should not trigger the pack-db to be refreshed.** Otherwise, performance may be sub-optimal
    /// in shallow repositories as running into non-existing commits will trigger a refresh of the `packs` directory.
    pub fn new(objects: impl gix_object::Find + 'find, cache: impl Into<Option<gix_commitgraph::Graph>>) -> Self {
        Graph {
            find: Box::new(objects),
            cache: cache.into(),
            map: gix_hashtable::HashMap::default(),
            buf: Vec::new(),
            parent_buf: Vec::new(),
        }
    }
}

/// commit access
impl<'find, T> Graph<'find, Commit<T>> {
    /// Lookup `id` without failing if the commit doesn't exist, and assure that `id` is inserted into our set
    /// with a commit with `new_data()` assigned.
    /// `update_data(data)` gets run either on existing or on new data.
    ///
    /// Note that none of the data updates happen if `id` didn't exist.
    pub fn try_lookup_or_insert_commit_default(
        &mut self,
        id: gix_hash::ObjectId,
        new_data: impl FnOnce() -> T,
        update_data: impl FnOnce(&mut T),
    ) -> Result<Option<&mut Commit<T>>, try_lookup_or_insert_default::Error> {
        match self.map.entry(id) {
            gix_hashtable::hash_map::Entry::Vacant(entry) => {
                let res = try_lookup(&id, &*self.find, self.cache.as_ref(), &mut self.buf)?;
                let commit = match res {
                    None => return Ok(None),
                    Some(commit) => commit,
                };
                let mut commit = commit.to_owned(new_data)?;
                update_data(&mut commit.data);
                entry.insert(commit);
            }
            gix_hashtable::hash_map::Entry::Occupied(mut entry) => {
                update_data(&mut entry.get_mut().data);
            }
        };
        Ok(self.map.get_mut(&id))
    }
}

/// commit access
impl<'find, T: Default> Graph<'find, Commit<T>> {
    /// Lookup `id` without failing if the commit doesn't exist or `id` isn't a commit,
    /// and assure that `id` is inserted into our set with a commit and default data assigned.
    /// `update_data(data)` gets run either on existing or on new data.
    ///
    /// Note that none of the data updates happen if `id` didn't exist.
    ///
    /// If only commit data is desired without the need for attaching custom data, use
    /// [`try_lookup(id).to_owned()`][Graph::try_lookup()] instead.
    pub fn try_lookup_or_insert_commit(
        &mut self,
        id: gix_hash::ObjectId,
        update_data: impl FnOnce(&mut T),
    ) -> Result<Option<&mut Commit<T>>, try_lookup_or_insert_default::Error> {
        self.try_lookup_or_insert_commit_default(id, T::default, update_data)
    }
}

/// Lazy commit access
impl<'find, T> Graph<'find, T> {
    /// Lookup `id` without failing if the commit doesn't exist or `id` isn't a commit,
    /// and assure that `id` is inserted into our set
    /// with a `default` value assigned to it.
    /// `update_data(data)` gets run either on existing or no new data.
    /// Return the commit when done.
    ///
    /// Note that none of the data updates happen if `id` didn't exist.
    ///
    /// If only commit data is desired without the need for attaching custom data, use
    /// [`try_lookup(id)`][Graph::try_lookup()] instead.
    pub fn try_lookup_or_insert_default(
        &mut self,
        id: gix_hash::ObjectId,
        default: impl FnOnce() -> T,
        update_data: impl FnOnce(&mut T),
    ) -> Result<Option<LazyCommit<'_>>, try_lookup_or_insert_default::Error> {
        let res = try_lookup(&id, &*self.find, self.cache.as_ref(), &mut self.buf)?;
        Ok(res.map(|commit| {
            match self.map.entry(id) {
                gix_hashtable::hash_map::Entry::Vacant(entry) => {
                    let mut data = default();
                    update_data(&mut data);
                    entry.insert(data);
                }
                gix_hashtable::hash_map::Entry::Occupied(mut entry) => {
                    update_data(entry.get_mut());
                }
            };
            commit
        }))
    }

    /// Try to lookup `id` and return a handle to it for accessing its data, but don't fail if the commit doesn't exist
    /// or isn't a commit.
    ///
    /// It's possible that commits don't exist if the repository is shallow.
    pub fn try_lookup(
        &mut self,
        id: &gix_hash::oid,
    ) -> Result<Option<LazyCommit<'_>>, gix_object::find::existing_iter::Error> {
        try_lookup(id, &*self.find, self.cache.as_ref(), &mut self.buf)
    }

    /// Lookup `id` and return a handle to it, or fail if it doesn't exist or is no commit.
    pub fn lookup(&mut self, id: &gix_hash::oid) -> Result<LazyCommit<'_>, gix_object::find::existing_iter::Error> {
        self.try_lookup(id)?
            .ok_or(gix_object::find::existing_iter::Error::NotFound { oid: id.to_owned() })
    }
}

fn try_lookup<'graph>(
    id: &gix_hash::oid,
    objects: &dyn gix_object::Find,
    cache: Option<&'graph gix_commitgraph::Graph>,
    buf: &'graph mut Vec<u8>,
) -> Result<Option<LazyCommit<'graph>>, gix_object::find::existing_iter::Error> {
    if let Some(cache) = cache {
        if let Some(pos) = cache.lookup(id) {
            return Ok(Some(LazyCommit {
                backing: Either::Right((cache, pos)),
            }));
        }
    }
    #[allow(clippy::manual_map)]
    Ok(
        match objects
            .try_find(id, buf)
            .map_err(gix_object::find::existing_iter::Error::Find)?
        {
            Some(data) => data.kind.is_commit().then_some(LazyCommit {
                backing: Either::Left(buf),
            }),
            None => None,
        },
    )
}

impl<'a, 'find, T> Index<&'a gix_hash::oid> for Graph<'find, T> {
    type Output = T;

    fn index(&self, index: &'a oid) -> &Self::Output {
        &self.map[index]
    }
}

/// A commit that contains all information we can obtain through the commit-graph, which is typically enough to fuel any graph iteration.
pub struct Commit<T> {
    /// The parents of the commit.
    pub parents: SmallVec<[gix_hash::ObjectId; 1]>,
    /// The time at which the commit was created.
    pub commit_time: SecondsSinceUnixEpoch,
    /// The generation of the commit, if available.
    pub generation: Option<u32>,
    /// Any kind of data to associate with this commit.
    pub data: T,
}

impl<T> std::fmt::Debug for Commit<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Commit")
            .field("parents", &self.parents)
            .field("commit_time", &self.commit_time)
            .field("generation", &self.generation)
            .field("data", &self.data)
            .finish()
    }
}

impl<T> Clone for Commit<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Commit {
            parents: self.parents.clone(),
            commit_time: self.commit_time,
            generation: self.generation,
            data: self.data.clone(),
        }
    }
}

/// A commit that provides access to graph-related information, on demand.
///
/// The owned version of this type is called [`Commit`] and can be obtained by calling [`LazyCommit::to_owned()`].
pub struct LazyCommit<'graph> {
    backing: Either<&'graph [u8], (&'graph gix_commitgraph::Graph, gix_commitgraph::Position)>,
}

enum Either<T, U> {
    Left(T),
    Right(U),
}
