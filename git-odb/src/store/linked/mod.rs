//! An object database representing a list of [compound databases][compound::Store] commonly created using _git alternates_.
use crate::store::compound;

/// A database with a list of [compound databases][compound::Store] created by traversing git `alternates` files.
///
/// It does not contain any objects itself.
pub struct Store {
    /// The compound databases containing the actual objects.
    pub dbs: Vec<compound::Store>,
}

///
pub mod store {
    #![allow(missing_docs, dead_code, unreachable_code)]

    use git_features::threading::OwnShared;

    /// Note that this type is only `Send` if `git-features/parallel` is toggled on.
    pub struct Handle<S> {
        store: S,
        new_pack_cache: OwnShared<handle::NewPackCacheFn>,
        new_object_cache: OwnShared<handle::NewObjectCacheFn>,
        pack_cache: Box<handle::PackCache>,
        object_cache: Box<handle::ObjectCache>,
    }

    pub mod handle;
}

///
pub mod init;

mod find;

///
mod write;

///
mod iter;
