use gix_hash::ObjectId;
use gix_object::{bstr::BStr, TreeRefIter};

use crate::{object::find, Id, Tree};

/// Initialization
impl<'repo> Tree<'repo> {
    /// Obtain a tree instance by handing in all components that it is made up of.
    pub fn from_data(id: impl Into<ObjectId>, data: Vec<u8>, repo: &'repo crate::Repository) -> Self {
        Tree {
            id: id.into(),
            data,
            repo,
        }
    }
}

/// Access
impl<'repo> Tree<'repo> {
    /// Return this tree's identifier.
    pub fn id(&self) -> Id<'repo> {
        Id::from_id(self.id, self.repo)
    }

    // TODO: tests.
    /// Follow a sequence of `path` components starting from this instance, and look them up one by one until the last component
    /// is looked up and its tree entry is returned.
    ///
    /// # Performance Notes
    ///
    /// Searching tree entries is currently done in sequence, which allows to the search to be allocation free. It would be possible
    /// to re-use a vector and use a binary search instead, which might be able to improve performance over all.
    /// However, a benchmark should be created first to have some data and see which trade-off to choose here.
    ///
    /// # Why is this consuming?
    ///
    /// The borrow checker shows pathological behaviour in loops that mutate a buffer, but also want to return from it.
    /// Workarounds include keeping an index and doing a separate access to the memory, which seems hard to do here without
    /// re-parsing the entries.
    pub fn lookup_entry<I, P>(mut self, path: I) -> Result<Option<Entry<'repo>>, find::existing::Error>
    where
        I: IntoIterator<Item = P>,
        P: PartialEq<BStr>,
    {
        let mut path = path.into_iter().peekable();
        while let Some(component) = path.next() {
            match TreeRefIter::from_bytes(&self.data)
                .filter_map(Result::ok)
                .find(|entry| component.eq(entry.filename))
            {
                Some(entry) => {
                    if path.peek().is_none() {
                        return Ok(Some(Entry {
                            inner: entry.into(),
                            repo: self.repo,
                        }));
                    } else {
                        let next_id = entry.oid.to_owned();
                        let repo = self.repo;
                        drop(self);
                        self = match repo.find_object(next_id)?.try_into_tree() {
                            Ok(tree) => tree,
                            Err(_) => return Ok(None),
                        };
                    }
                }
                None => return Ok(None),
            }
        }
        Ok(None)
    }

    /// Like [`lookup_entry()`][Self::lookup_entry()], but takes a `Path` directly via `relative_path`, a path relative to this tree.
    ///
    /// # Note
    ///
    /// If any path component contains illformed UTF-8 and thus can't be converted to bytes on platforms which can't do so natively,
    /// the returned component will be empty which makes the lookup fail.
    pub fn lookup_entry_by_path(
        self,
        relative_path: impl AsRef<std::path::Path>,
    ) -> Result<Option<Entry<'repo>>, find::existing::Error> {
        use crate::bstr::ByteSlice;
        self.lookup_entry(relative_path.as_ref().components().map(|c: std::path::Component<'_>| {
            gix_path::os_str_into_bstr(c.as_os_str())
                .unwrap_or_else(|_| "".into())
                .as_bytes()
        }))
    }
}

///
pub mod diff;

///
pub mod traverse;

///
mod iter;
pub use iter::EntryRef;

impl<'r> std::fmt::Debug for Tree<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tree({})", self.id)
    }
}

/// An entry in a [`Tree`], similar to an entry in a directory.
#[derive(PartialEq, Debug, Clone)]
pub struct Entry<'repo> {
    inner: gix_object::tree::Entry,
    repo: &'repo crate::Repository,
}

mod entry {
    use crate::{bstr::BStr, ext::ObjectIdExt, object::tree::Entry};

    /// Access
    impl<'repo> Entry<'repo> {
        /// The kind of object to which `oid` is pointing to.
        pub fn mode(&self) -> gix_object::tree::EntryMode {
            self.inner.mode
        }

        /// The name of the file in the parent tree.
        pub fn filename(&self) -> &BStr {
            self.inner.filename.as_ref()
        }

        /// Return the object id of the entry.
        pub fn id(&self) -> crate::Id<'repo> {
            self.inner.oid.attach(self.repo)
        }

        /// Return the object this entry points to.
        pub fn object(&self) -> Result<crate::Object<'repo>, crate::object::find::existing::Error> {
            self.id().object()
        }

        /// Return the plain object id of this entry, without access to the repository.
        pub fn oid(&self) -> &gix_hash::oid {
            &self.inner.oid
        }

        /// Return the plain object id of this entry, without access to the repository.
        pub fn object_id(&self) -> gix_hash::ObjectId {
            self.inner.oid
        }
    }

    /// Consuming
    impl Entry<'_> {
        /// Return the contained object.
        pub fn detach(self) -> gix_object::tree::Entry {
            self.inner
        }
    }
}
