use std::{cell::RefCell, path::PathBuf};

use git_hash::ObjectId;

use crate::easy;

pub(crate) mod ext;

pub mod borrow;
pub mod commit;
pub mod handle;
pub mod head;
pub mod object;
pub mod oid;
pub mod reference;
pub mod tag;

/// The head reference, as created from looking at `.git/HEAD`, able to represent all of its possible states.
///
/// Note that like [`Reference`], this type's data is snapshot of persisted state on disk.
pub struct Head<'repo> {
    /// One of various possible states for the HEAD reference
    pub kind: head::Kind,
    handle: &'repo easy::Handle,
}

/// An [ObjectId] with access to a repository.
#[derive(Clone, Copy)]
pub struct Oid<'r> {
    /// The actual object id
    inner: ObjectId,
    handle: &'r easy::Handle,
}

/// A decoded object with a reference to its owning repository.
///
/// ## Limitations
///
/// Note that it holds a reference to a buffer of it's associated repository handle, so there
/// can only be one at a time, per handle.
pub struct ObjectRef<'repo> {
    /// The id of the object
    pub id: ObjectId,
    /// The kind of the object
    pub kind: git_object::Kind,
    /// The fully decoded object data
    pub data: std::cell::Ref<'repo, [u8]>,
    handle: &'repo easy::Handle,
}

/// A decoded tree object with access to its owning repository.
///
/// Please note that the limitations described in [ObjectRef] apply here as well.
pub struct TreeRef<'repo> {
    /// The id of the tree
    pub id: ObjectId,
    /// The fully decoded tree data
    pub data: std::cell::Ref<'repo, [u8]>,
    handle: &'repo easy::Handle,
}

/// A detached, self-contained object, without access to its source repository.
///
/// Use it if an `ObjectRef` should be sent over thread boundaries or stored in collections.
#[derive(Clone)]
pub struct Object {
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
    pub(crate) handle: &'r easy::Handle,
}

/// State for use in `Easy*` to provide mutable parts of a repository such as caches and buffers.
///
/// Note that it clones itself so that it is empty, requiring the user to configure each clone separately, specifically
/// and explicitly. This is to have the fastest-possible default configuration available by default, but allow
/// those who experiment with workloads to get speed boosts of 2x or more.
pub struct Handle {
    /// A ref store with shared ownership (or the equivalent of it).
    pub refs: crate::RefStore,
    /// A way to access objects.
    pub objects: crate::OdbHandle,
    work_tree: Option<PathBuf>,
    /// The kind of hash that is used or should be used for object ids
    hash_kind: git_hash::Kind,
    buf: RefCell<Vec<u8>>,
}
