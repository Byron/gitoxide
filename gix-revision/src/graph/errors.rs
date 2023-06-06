///
pub mod lookup {
    /// The error returned by [`try_lookup()`][crate::Graph::try_lookup()].
    #[derive(Debug, thiserror::Error)]
    #[error("There was an error looking up a commit")]
    pub struct Error {
        #[from]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    }

    ///
    pub mod commit {
        /// The error returned by [`try_lookup_or_insert_commit()`][crate::Graph::try_lookup_or_insert_commit()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            Find(#[from] crate::graph::lookup::Error),
            #[error(transparent)]
            ToOwned(#[from] crate::graph::commit::to_owned::Error),
        }
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
    use crate::graph::{commit::iter_parents, lookup};

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
