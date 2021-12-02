use crate::linked;
use crate::linked::store::Handle;
use git_features::threading::OwnShared;

pub type PackCache = dyn git_pack::cache::DecodeEntry + Send + 'static;
pub type NewPackCacheFn = dyn Fn() -> Box<PackCache> + 'static;

pub type ObjectCache = dyn git_pack::cache::Object + Send + 'static;
pub type NewObjectCacheFn = dyn Fn() -> Box<ObjectCache> + 'static;

impl<S> Handle<S> {
    pub fn with_pack_cache(mut self, create: impl Fn() -> Box<PackCache> + 'static) -> Self {
        self.new_pack_cache = OwnShared::new(create);
        self.pack_cache = (self.new_pack_cache)();
        self
    }
    pub fn with_object_cache(mut self, create: impl Fn() -> Box<ObjectCache> + 'static) -> Self {
        self.new_object_cache = OwnShared::new(create);
        self.object_cache = (self.new_object_cache)();
        self
    }
}

impl<S> From<S> for Handle<S>
where
    S: git_pack::Find,
{
    fn from(store: S) -> Self {
        let new_pack_cache = OwnShared::new(|| -> Box<PackCache> { Box::new(git_pack::cache::Never) });
        let new_object_cache = OwnShared::new(|| -> Box<ObjectCache> { Box::new(git_pack::cache::object::Never) });
        Self {
            store,
            pack_cache: new_pack_cache(),
            new_pack_cache,
            object_cache: new_object_cache(),
            new_object_cache,
        }
    }
}

impl linked::Store {
    pub fn to_handle(&self) -> Handle<&Self> {
        self.into()
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
