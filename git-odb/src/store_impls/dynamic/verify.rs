use crate::pack;
use git_features::progress::Progress;
use std::sync::atomic::AtomicBool;

#[allow(missing_docs, unused)]

///
pub mod integrity {
    use crate::pack;

    /// Returned by [`Store::verify_integrity()`][crate::Store::verify_integrity()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        MultiIndex(#[from] pack::index::traverse::Error<pack::multi_index::verify::integrity::Error>),
        #[error(transparent)]
        Index(#[from] pack::index::traverse::Error<pack::index::verify::integrity::Error>),
    }

    /// Returned by [`Store::verify_integrity()`][crate::Store::verify_integrity()].
    pub struct Outcome<P> {
        /// Pack traversal statistics for each pack whose objects were checked.
        pub pack_traverse_statistics: Vec<pack::index::traverse::Statistics>,
        /// The provided progress instance.
        pub progress: P,
    }
}

impl super::Store {
    /// Check the integrity of all objects as per the given `options`.
    ///
    /// Note that this will not not force loading all indices or packs permanently, as we will only use the momentarily loaded disk state.
    /// This does, however, include all alternates.
    pub fn verify_integrity<C, P, F>(
        &self,
        _progress: P,
        _should_interrupt: &AtomicBool,
        _options: pack::index::verify::integrity::Options<F>,
    ) -> Result<integrity::Outcome<P>, pack::index::traverse::Error<integrity::Error>>
    where
        P: Progress,
        C: pack::cache::DecodeEntry,
        F: Fn() -> C + Send + Clone,
    {
        todo!()
    }
}
