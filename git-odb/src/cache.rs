use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

use crate::Cache;

/// A type to store pack caches in boxes.
pub type PackCache = dyn git_pack::cache::DecodeEntry + Send + 'static;
/// A constructor for boxed pack caches.
pub type NewPackCacheFn = dyn Fn() -> Box<PackCache> + Send + Sync + 'static;

/// A type to store object caches in boxes.
pub type ObjectCache = dyn git_pack::cache::Object + Send + 'static;
/// A constructor for boxed object caches.
pub type NewObjectCacheFn = dyn Fn() -> Box<ObjectCache> + Send + Sync + 'static;

impl Cache<crate::store::Handle<Rc<crate::Store>>> {
    /// Convert this cache's handle into one that keeps its store in an arc. This creates an entirely new store,
    /// so should be done early to avoid unnecessary work (and mappings).
    pub fn into_arc(self) -> std::io::Result<Cache<crate::store::Handle<Arc<crate::Store>>>> {
        let inner = self.inner.into_arc()?;
        Ok(Cache {
            inner,
            new_pack_cache: self.new_pack_cache,
            new_object_cache: self.new_object_cache,
            pack_cache: self.pack_cache,
            object_cache: self.object_cache,
        })
    }
}
impl Cache<crate::store::Handle<Arc<crate::Store>>> {
    /// No op, as we are containing an arc handle already.
    pub fn into_arc(self) -> std::io::Result<Cache<crate::store::Handle<Arc<crate::Store>>>> {
        Ok(self)
    }
}

impl<S> Cache<S> {
    /// Dissolve this instance, discard all caches, and return the inner implementation.
    pub fn into_inner(self) -> S {
        self.inner
    }
    /// Use this methods directly after creating a new instance to add a constructor for pack caches.
    ///
    /// These are used to speed up decoding objects which are located in packs, reducing long delta chains by storing
    /// their intermediate results.
    pub fn with_pack_cache(mut self, create: impl Fn() -> Box<PackCache> + Send + Sync + 'static) -> Self {
        self.pack_cache = Some(RefCell::new(create()));
        self.new_pack_cache = Some(Arc::new(create));
        self
    }
    /// Use this methods directly after creating a new instance to add a constructor for object caches.
    ///
    /// Only use this kind of cache if the same objects are repeatedly accessed for great speedups, usually during diffing of
    /// trees.
    pub fn with_object_cache(mut self, create: impl Fn() -> Box<ObjectCache> + Send + Sync + 'static) -> Self {
        self.object_cache = Some(RefCell::new(create()));
        self.new_object_cache = Some(Arc::new(create));
        self
    }
    /// Set the pack cache constructor on this instance.
    pub fn set_pack_cache(&mut self, create: impl Fn() -> Box<PackCache> + Send + Sync + 'static) {
        self.pack_cache = Some(RefCell::new(create()));
        self.new_pack_cache = Some(Arc::new(create));
    }
    /// Set the object cache constructor on this instance.
    pub fn set_object_cache(&mut self, create: impl Fn() -> Box<ObjectCache> + Send + Sync + 'static) {
        self.object_cache = Some(RefCell::new(create()));
        self.new_object_cache = Some(Arc::new(create));
    }
    /// Return true if an object cache is present.
    pub fn has_object_cache(&self) -> bool {
        self.object_cache.is_some()
    }
    /// Return true if a pack cache is present.
    pub fn has_pack_cache(&self) -> bool {
        self.pack_cache.is_some()
    }
    /// Remove the current pack cache as well as its constructor from this instance.
    pub fn unset_pack_cache(&mut self) {
        self.pack_cache = None;
        self.new_pack_cache = None;
    }
    /// Remove the current object cache as well as its constructor from this instance.
    pub fn unset_object_cache(&mut self) {
        self.object_cache = None;
        self.new_object_cache = None;
    }
}

impl<S> From<S> for Cache<S>
where
    S: git_pack::Find,
{
    fn from(store: S) -> Self {
        Self {
            inner: store,
            pack_cache: None,
            new_pack_cache: None,
            object_cache: None,
            new_object_cache: None,
        }
    }
}

impl<S: Clone> Clone for Cache<S> {
    fn clone(&self) -> Self {
        Cache {
            inner: self.inner.clone(),
            new_pack_cache: self.new_pack_cache.clone(),
            new_object_cache: self.new_object_cache.clone(),
            pack_cache: self.new_pack_cache.as_ref().map(|create| RefCell::new(create())),
            object_cache: self.new_object_cache.as_ref().map(|create| RefCell::new(create())),
        }
    }
}

impl<S> Deref for Cache<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S> DerefMut for Cache<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

mod impls {
    use std::{io::Read, ops::DerefMut};

    use git_hash::{oid, ObjectId};
    use git_object::{Data, Kind};
    use git_pack::cache::Object;

    use crate::find::Header;
    use crate::{pack::data::entry::Location, Cache};

    impl<S> crate::Write for Cache<S>
    where
        S: crate::Write,
    {
        type Error = S::Error;

        fn write_stream(&self, kind: Kind, size: u64, from: impl Read) -> Result<ObjectId, Self::Error> {
            self.inner.write_stream(kind, size, from)
        }
    }

    impl<S> crate::Find for Cache<S>
    where
        S: git_pack::Find,
    {
        type Error = S::Error;

        fn contains(&self, id: impl AsRef<oid>) -> bool {
            self.inner.contains(id)
        }

        fn try_find<'a>(&self, id: impl AsRef<oid>, buffer: &'a mut Vec<u8>) -> Result<Option<Data<'a>>, Self::Error> {
            git_pack::Find::try_find(self, id, buffer).map(|t| t.map(|t| t.0))
        }
    }

    impl<S> crate::Header for Cache<S>
    where
        S: crate::Header,
    {
        type Error = S::Error;

        fn try_header(&self, id: impl AsRef<oid>) -> Result<Option<Header>, Self::Error> {
            self.inner.try_header(id)
        }
    }

    impl<S> git_pack::Find for Cache<S>
    where
        S: git_pack::Find,
    {
        type Error = S::Error;

        fn contains(&self, id: impl AsRef<oid>) -> bool {
            self.inner.contains(id)
        }

        fn try_find<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
        ) -> Result<Option<(Data<'a>, Option<Location>)>, Self::Error> {
            match self.pack_cache.as_ref().map(|rc| rc.borrow_mut()) {
                Some(mut pack_cache) => self.try_find_cached(id, buffer, pack_cache.deref_mut()),
                None => self.try_find_cached(id, buffer, &mut git_pack::cache::Never),
            }
        }

        fn try_find_cached<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl git_pack::cache::DecodeEntry,
        ) -> Result<Option<(Data<'a>, Option<git_pack::data::entry::Location>)>, Self::Error> {
            if let Some(mut obj_cache) = self.object_cache.as_ref().map(|rc| rc.borrow_mut()) {
                if let Some(kind) = obj_cache.get(&id.as_ref().to_owned(), buffer) {
                    return Ok(Some((Data::new(kind, buffer), None)));
                }
            }
            let possibly_obj = self.inner.try_find_cached(id.as_ref(), buffer, pack_cache)?;
            if let (Some(mut obj_cache), Some((obj, _location))) =
                (self.object_cache.as_ref().map(|rc| rc.borrow_mut()), &possibly_obj)
            {
                obj_cache.put(id.as_ref().to_owned(), obj.kind, obj.data);
            }
            Ok(possibly_obj)
        }

        fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<git_pack::data::entry::Location> {
            self.inner.location_by_oid(id, buf)
        }

        fn pack_offsets_and_oid(&self, pack_id: u32) -> Option<Vec<(u64, git_hash::ObjectId)>> {
            self.inner.pack_offsets_and_oid(pack_id)
        }

        fn entry_by_location(&self, location: &Location) -> Option<git_pack::find::Entry> {
            self.inner.entry_by_location(location)
        }
    }
}
