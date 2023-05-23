//! An implementation of negotiation algorithms to help the server figure out what we have in common so it can optimize
//! the pack it sends to only contain what we don't have.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

mod consecutive;
mod noop;
mod skipping;

/// The way the negotiation is performed.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Algorithm {
    /// Do not send any information at all, which typically leads to complete packs to be sent.
    Noop,
    /// Walk over consecutive commits and check each one. This can be costly be assures packs are exactly the size they need to be.
    #[default]
    Consecutive,
    /// Like `Consecutive`, but skips commits to converge faster, at the cost of receiving packs that are larger than they have to be.
    Skipping,
}

/// Calculate how many `HAVE` lines we may send in one round, with variation depending on whether the `transport_is_stateless` or not.
/// `window_size` is the previous (or initial) value of the window size.
pub fn window_size(transport_is_stateless: bool, window_size: impl Into<Option<usize>>) -> usize {
    let current_size = match window_size.into() {
        None => return 16,
        Some(cs) => cs,
    };
    const PIPESAFE_FLUSH: usize = 32;
    const LARGE_FLUSH: usize = 16384;

    if transport_is_stateless {
        if current_size < LARGE_FLUSH {
            current_size * 2
        } else {
            current_size * 11 / 10
        }
    } else if current_size < PIPESAFE_FLUSH {
        current_size * 2
    } else {
        current_size + PIPESAFE_FLUSH
    }
}

impl Algorithm {
    /// Create an instance of a negotiator which implements this algorithm.
    pub fn into_negotiator<'find, Find, E>(
        self,
        find: Find,
        cache: impl Into<Option<gix_commitgraph::Graph>>,
    ) -> Box<dyn Negotiator + 'find>
    where
        Find:
            for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<Option<gix_object::CommitRefIter<'a>>, E> + 'find,
        E: std::error::Error + Send + Sync + 'static,
    {
        match &self {
            Algorithm::Noop => Box::new(noop::Noop) as Box<dyn Negotiator>,
            Algorithm::Consecutive => {
                let graph = gix_revision::Graph::<'_, consecutive::Flags>::new(find, cache);
                Box::new(consecutive::Algorithm::new(graph))
            }
            Algorithm::Skipping => {
                let graph = gix_revision::Graph::<'_, skipping::Entry>::new(find, cache);
                Box::new(skipping::Algorithm::new(graph))
            }
        }
    }
}

/// A delegate to implement a negotiation algorithm.
pub trait Negotiator {
    /// Mark `id` as common between the remote and us.
    ///
    /// These ids are typically the local tips of remote tracking branches.
    fn known_common(&mut self, id: gix_hash::ObjectId) -> Result<(), Error>;

    /// Add `id` as starting point of a traversal across commits that aren't necessarily common between the remote and us.
    ///
    /// These tips are usually the commits of local references whose tips should lead to objects that we have in common with the remote.
    fn add_tip(&mut self, id: gix_hash::ObjectId) -> Result<(), Error>;

    /// Produce the next id of an object that we want the server to know we have. It's an object we don't know we have in common or not.
    ///
    /// Returns `None` if we have exhausted all options, which might mean we have traversed the entire commit graph.
    fn next_have(&mut self) -> Option<Result<gix_hash::ObjectId, Error>>;

    /// Mark `id` as being common with the remote (as informed by the remote itself) and return `true` if we knew it was common already.
    ///
    /// We can assume to have already seen `id` as we were the one to inform the remote in a prior `have`.
    fn in_common_with_remote(&mut self, id: gix_hash::ObjectId) -> Result<bool, Error>;
}

/// An error that happened during any of the methods on a [`Negotiator`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    DecodeCommit(#[from] gix_object::decode::Error),
    #[error(transparent)]
    DecodeCommitInGraph(#[from] gix_commitgraph::file::commit::Error),
    #[error(transparent)]
    LookupCommitInGraph(#[from] gix_revision::graph::lookup::Error),
}
