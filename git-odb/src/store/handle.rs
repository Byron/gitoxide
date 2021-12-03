#![allow(missing_docs, dead_code, unreachable_code)]

use crate::Handle;
use std::cell::RefCell;
use std::sync::Arc;

pub type PackCache = dyn git_pack::cache::DecodeEntry + Send + 'static;
pub type NewPackCacheFn = dyn Fn() -> Box<PackCache> + Send + Sync + 'static;

pub type ObjectCache = dyn git_pack::cache::Object + Send + 'static;
pub type NewObjectCacheFn = dyn Fn() -> Box<ObjectCache> + Send + Sync + 'static;

impl<S> Handle<S> {
    pub fn with_pack_cache(mut self, create: impl Fn() -> Box<PackCache> + Send + Sync + 'static) -> Self {
        self.pack_cache = Some(RefCell::new(create()));
        self.new_pack_cache = Some(Arc::new(create));
        self
    }
    pub fn with_object_cache(mut self, create: impl Fn() -> Box<ObjectCache> + Send + Sync + 'static) -> Self {
        self.object_cache = Some(RefCell::new(create()));
        self.new_object_cache = Some(Arc::new(create));
        self
    }
    pub fn set_pack_cache(&mut self, create: impl Fn() -> Box<PackCache> + Send + Sync + 'static) {
        self.pack_cache = Some(RefCell::new(create()));
        self.new_pack_cache = Some(Arc::new(create));
    }
    pub fn set_object_cache(&mut self, create: impl Fn() -> Box<ObjectCache> + Send + Sync + 'static) {
        self.object_cache = Some(RefCell::new(create()));
        self.new_object_cache = Some(Arc::new(create));
    }
    pub fn without_pack_cache(&mut self) {
        self.pack_cache = None;
        self.new_pack_cache = None;
    }
    pub fn without_object_cache(&mut self) {
        self.object_cache = None;
        self.new_object_cache = None;
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
            pack_cache: self.new_pack_cache.as_ref().map(|create| RefCell::new(create())),
            object_cache: self.new_object_cache.as_ref().map(|create| RefCell::new(create())),
        }
    }
}

mod find_impl {
    use crate::pack::bundle::Location;
    use crate::Handle;
    use git_hash::oid;
    use git_object::Data;
    use git_pack::cache::Object;
    use std::ops::DerefMut;

    impl<S> crate::Find for Handle<S>
    where
        S: crate::pack::Find,
    {
        type Error = S::Error;

        fn contains(&self, id: impl AsRef<oid>) -> bool {
            self.store.contains(id)
        }

        fn try_find<'a>(&self, id: impl AsRef<oid>, buffer: &'a mut Vec<u8>) -> Result<Option<Data<'a>>, Self::Error> {
            git_pack::Find::try_find(self, id, buffer).map(|t| t.map(|t| t.0))
        }
    }

    impl<S> crate::pack::Find for Handle<S>
    where
        S: crate::pack::Find,
    {
        type Error = S::Error;

        fn contains(&self, id: impl AsRef<oid>) -> bool {
            self.store.contains(id)
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
        ) -> Result<Option<(Data<'a>, Option<git_pack::bundle::Location>)>, Self::Error> {
            if let Some(mut obj_cache) = self.object_cache.as_ref().map(|rc| rc.borrow_mut()) {
                if let Some(kind) = obj_cache.get(&id.as_ref().to_owned(), buffer) {
                    return Ok(Some((Data::new(kind, buffer), None)));
                }
            }
            let possibly_obj = self.store.try_find_cached(id.as_ref(), buffer, pack_cache)?;
            if let (Some(mut obj_cache), Some((obj, _location))) =
                (self.object_cache.as_ref().map(|rc| rc.borrow_mut()), &possibly_obj)
            {
                obj_cache.put(id.as_ref().to_owned(), obj.kind, obj.data);
            }
            Ok(possibly_obj)
        }

        fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<git_pack::bundle::Location> {
            self.store.location_by_oid(id, buf)
        }

        fn index_iter_by_pack_id(&self, pack_id: u32) -> Option<Box<dyn Iterator<Item = git_pack::index::Entry> + '_>> {
            self.store.index_iter_by_pack_id(pack_id)
        }

        fn entry_by_location(&self, location: &Location) -> Option<git_pack::find::Entry<'_>> {
            self.store.entry_by_location(location)
        }
    }
}
