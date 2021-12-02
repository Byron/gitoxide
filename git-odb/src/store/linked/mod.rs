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

    use crate::linked::handle;
    use git_features::threading::OwnShared;

    /// Note that this type is only `Send` if `git-features/parallel` is toggled on.
    pub struct Handle<S> {
        store: S,
        new_pack_cache: OwnShared<handle::NewPackCacheFn>,
        new_object_cache: OwnShared<handle::NewObjectCacheFn>,
        pack_cache: Box<handle::PackCache>,
        object_cache: Box<handle::ObjectCache>,
    }

    impl<S> Handle<S> {
        pub fn with_pack_cache(mut self, create: impl Fn() -> Box<handle::PackCache> + 'static) -> Self {
            self.new_pack_cache = OwnShared::new(create);
            self.pack_cache = (self.new_pack_cache)();
            self
        }
        pub fn with_object_cache(mut self, create: impl Fn() -> Box<handle::ObjectCache> + 'static) -> Self {
            self.new_object_cache = OwnShared::new(create);
            self.object_cache = (self.new_object_cache)();
            self
        }
    }

    impl<S: Clone> Clone for Handle<S> {
        fn clone(&self) -> Self {
            Handle {
                store: self.store.clone(),
                new_pack_cache: self.new_pack_cache.clone(),
                new_object_cache: self.new_object_cache.clone(),
                pack_cache: (self.new_pack_cache)(),
                object_cache: (self.new_object_cache)(),
            }
        }
    }
}

///
pub mod init;

mod find;

mod handle;

///
mod write;

///
mod iter;
