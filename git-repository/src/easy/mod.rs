use std::{cell::RefCell, path::PathBuf};

use git_hash::ObjectId;

use crate::easy;

pub(crate) mod ext;

pub mod borrow;
pub mod commit;
pub mod head;
pub mod object;
pub mod oid;
pub mod reference;
pub mod repository;
pub mod tag;

/// The head reference, as created from looking at `.git/HEAD`, able to represent all of its possible states.
///
/// Note that like [`Reference`], this type's data is snapshot of persisted state on disk.
pub struct Head<'repo> {
    /// One of various possible states for the HEAD reference
    pub kind: head::Kind,
    handle: &'repo easy::Repository,
}

/// An [ObjectId] with access to a repository.
#[derive(Clone, Copy)]
pub struct Oid<'r> {
    /// The actual object id
    inner: ObjectId,
    handle: &'r easy::Repository,
}

/// A decoded object with a reference to its owning repository.
///
/// ## Limitations
///
/// Note that it holds a reference to a buffer of it's associated repository handle, so there
/// can only be one at a time, per handle.
pub struct Object<'repo> {
    /// The id of the object
    pub id: ObjectId,
    /// The kind of the object
    pub kind: git_object::Kind,
    /// The fully decoded object data
    pub data: Vec<u8>,
    handle: &'repo easy::Repository,
}

impl<'a> Drop for Object<'a> {
    fn drop(&mut self) {
        self.handle.reuse_buffer(&mut self.data);
    }
}

/// A decoded tree object with access to its owning repository.
///
/// Please note that the limitations described in [Object] apply here as well.
pub struct Tree<'repo> {
    /// The id of the tree
    pub id: ObjectId,
    /// The fully decoded tree data
    pub data: Vec<u8>,
    handle: &'repo easy::Repository,
}

impl<'a> Drop for Tree<'a> {
    fn drop(&mut self) {
        self.handle.reuse_buffer(&mut self.data);
    }
}

/// A decoded commit object with access to its owning repository.
///
/// Please note that the limitations described in [Object] apply here as well.
pub struct Commit<'repo> {
    /// The id of the commit
    pub id: ObjectId,
    /// The fully decoded commit data
    pub data: Vec<u8>,
    handle: &'repo easy::Repository,
}

impl<'a> Drop for Commit<'a> {
    fn drop(&mut self) {
        self.handle.reuse_buffer(&mut self.data);
    }
}

/// A detached, self-contained object, without access to its source repository.
///
/// Use it if an `ObjectRef` should be sent over thread boundaries or stored in collections.
#[derive(Clone)]
pub struct DetachedObject {
    /// The id of the object
    pub id: ObjectId,
    /// The kind of the object
    pub kind: git_object::Kind,
    /// The fully decoded object data
    pub data: Vec<u8>,
}

/// A reference that points to an object or reference, with access to its source repository.
///
/// Note that these are snapshots and won't recognize if they are stale.
pub struct Reference<'r> {
    /// The actual reference data
    pub inner: git_ref::Reference,
    pub(crate) handle: &'r easy::Repository,
}

/// A thread-local handle to interact with a repository from a single thread.
///
/// It is `Send` but **not** `Sync` - for the latter you can convert it `to_sync()`.
/// Note that it clones itself so that it is empty, requiring the user to configure each clone separately, specifically
/// and explicitly. This is to have the fastest-possible default configuration available by default, but allow
/// those who experiment with workloads to get speed boosts of 2x or more.
pub struct Repository {
    /// A ref store with shared ownership (or the equivalent of it).
    pub refs: crate::RefStore,
    /// A way to access objects.
    pub objects: crate::OdbHandle,
    work_tree: Option<PathBuf>,
    /// The kind of hash that is used or should be used for object ids
    object_hash: git_hash::Kind,
    /// Access to all repository configuration, must be hidden as there is a lot figure out.
    config: crate::Config,
    /// A free-list of re-usable object backing buffers
    bufs: RefCell<Vec<Vec<u8>>>,
}
