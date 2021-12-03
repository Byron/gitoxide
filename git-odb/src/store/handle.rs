#![allow(missing_docs, dead_code, unreachable_code)]

use crate::Handle;
use git_features::threading::OwnShared;

pub type PackCache = dyn git_pack::cache::DecodeEntry + Send + 'static;
pub type NewPackCacheFn = dyn Fn() -> Box<PackCache> + 'static;

pub type ObjectCache = dyn git_pack::cache::Object + Send + 'static;
pub type NewObjectCacheFn = dyn Fn() -> Box<ObjectCache> + 'static;

impl<S> Handle<S> {
    pub fn with_pack_cache(mut self, create: impl Fn() -> Box<PackCache> + 'static) -> Self {
        self.pack_cache = Some(create());
        self.new_pack_cache = Some(OwnShared::new(create));
        self
    }
    pub fn with_object_cache(mut self, create: impl Fn() -> Box<ObjectCache> + 'static) -> Self {
        self.object_cache = Some(create());
        self.new_object_cache = Some(OwnShared::new(create));
        self
    }
}

impl<S> From<S> for Handle<S>
where
    S: git_pack::Find,
{
    fn from(store: S) -> Self {
        Self {
            store,
            pack_cache: None,
            new_pack_cache: None,
            object_cache: None,
            new_object_cache: None,
        }
    }
}

impl<S: Clone> Clone for Handle<S> {
    fn clone(&self) -> Self {
        Handle {
            store: self.store.clone(),
            new_pack_cache: self.new_pack_cache.clone(),
            new_object_cache: self.new_object_cache.clone(),
            pack_cache: self.new_pack_cache.as_ref().map(|create| create()),
            object_cache: self.new_object_cache.as_ref().map(|create| create()),
        }
    }
}
