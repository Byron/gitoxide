//! ### Which `Easy*` is for me?
//!
//! * Use `Easy*Exclusive` when the underlying `Repository` eventually needs mutation, for instance to update data structures
//!    - This is useful for long-running applications that eventually need to adapt to changes in the repository and pick up
//!      new packs after a GC operation or a received pack.
//! * Use the non-exclusive variants if the `Repository` doesn't ever have to change, for example as in one-off commands.
//!
//! ### Implementation Notes
//!
//! - Why no `Easy` with simply an owned `Repository`, instead `Rc<Repository>` is enforced
//!    - When this is desired, rather use `EasyShared` and drop the `EasyShared` once mutable access to the `Repository` is needed.
//!      `Access` is not usable for functions that require official `&mut` mutability, it's made for interior mutability to support
//!       trees of objects.
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use git_hash::ObjectId;

use crate::Repository;

mod impls;

pub(crate) mod ext;

pub mod borrow;
pub mod commit;
pub mod head;
pub mod object;
pub mod odb;
pub mod oid;
pub mod reference;
pub mod state;

/// The head reference, as created from looking at `.git/HEAD`, able to represent all of its possible states.
///
/// Note that like [`Reference`], this type's data is snapshot of persisted state on disk.
pub struct Head<'repo, A> {
    /// One of various possible states for the HEAD reference
    pub kind: head::Kind,
    access: &'repo A,
}

/// An [ObjectId] with access to a repository.
#[derive(Eq, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct Oid<'r, A> {
    /// The actual object id
    inner: ObjectId,
    access: &'r A,
}

/// A decoded object with a reference to its owning repository.
///
/// ## Limitations
///
/// Note that it holds a reference to a buffer of it's associated repository handle, so there
/// can only be one at a time, per handle.
pub struct ObjectRef<'repo, A> {
    /// The id of the object
    pub id: ObjectId,
    /// The kind of the object
    pub kind: git_object::Kind,
    /// The fully decoded object data
    pub data: std::cell::Ref<'repo, [u8]>,
    access: &'repo A,
}

/// A decoded tree object with access to its owning repository.
///
/// Please note that the limitations described in [ObjectRef] apply here as well.
pub struct TreeRef<'repo, A> {
    /// The id of the tree
    pub id: ObjectId,
    /// The fully decoded tree data
    pub data: std::cell::Ref<'repo, [u8]>,
    access: &'repo A,
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
pub struct Reference<'r, A> {
    /// The actual reference data
    pub inner: git_ref::Reference,
    pub(crate) access: &'r A,
}

#[cfg(not(feature = "max-performance"))]
type PackCache = git_pack::cache::Never;
#[cfg(feature = "max-performance")]
type PackCache = Box<dyn git_pack::cache::DecodeEntry + Send + 'static>;

/// State for use in `Easy*` to provide mutable parts of a repository such as caches and buffers.
///
/// Note that it clones itself so that it is empty, requiring the user to configure each clone separately, specifically
/// and explicitly. This is to have the fastest-possible default configuration available by default, but allow
/// those who experiment with workloads to get speed boosts of 2x or more.
pub struct State {
    /// As the packed-buffer may hold onto a memory map, so ideally this State is freed after use instead of keeping it around
    /// for too long. At least `packed_refs` is lazily initialized.
    packed_refs: RefCell<reference::packed::ModifieablePackedRefsBuffer>,
    pack_cache: RefCell<PackCache>,
    object_cache: RefCell<Option<object::cache::MemoryCappedHashmap>>,
    buf: RefCell<Vec<u8>>,
}

/// A utility trait to represent access to a repository.
///
/// It provides immutable and possibly mutable access. Both types of access are validated at runtime, which may fail
/// or may block, depending on the implementation.
///
/// Furthermore it provides access to additional state for use with the [`Repository`]. It is designed for thread-local
/// mutable access, which is checked at runtime as well. This means that operations can't freely be interleaved and some
/// care as to be taken especially in conjunction with [`ObjectRef`] instances.
pub trait Access {
    /// The type of a shared borrow to the Repository
    type RepoRef: Deref<Target = Repository>;
    // TODO: Once GATs become stable, try to use them to make it work with RefCells too, aka EasyExclusive
    /// The type of a mutable borrow to the Repository
    type RepoRefMut: DerefMut<Target = Repository>;

    /// Return a shared borrow to the repository.
    ///
    /// This may fail if there already is a mutable borrow
    fn repo(&self) -> borrow::repo::Result<Self::RepoRef>;

    /// Returns a mutable borrow to the repository if possible.
    ///
    /// # NOTE
    ///
    /// This may not be supported by all implementations. Choosing an implementation that does support it is particularly
    /// relevant for long-running applications that make changes to the repository.
    fn repo_mut(&self) -> borrow::repo::Result<Self::RepoRefMut>;

    /// Return a shared borrow of the repository state, with support for interior mutability.
    fn state(&self) -> &State;
}
