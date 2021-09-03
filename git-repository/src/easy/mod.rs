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
    sync::Arc,
    time::SystemTime,
};

use git_hash::ObjectId;
use git_object as objs;
use git_odb as odb;
use git_ref as refs;

use crate::Repository;

mod impls;

pub(crate) mod ext;

pub mod borrow;
pub mod commit;
pub mod object;
mod oid;
pub mod reference;
pub mod state;

/// An [ObjectId] with access to a repository.
#[derive(Eq, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct Oid<'r, A> {
    id: ObjectId,
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
    pub kind: objs::Kind,
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
    pub kind: objs::Kind,
    /// The fully decoded object data
    pub data: Vec<u8>,
}

/// A reference that points to an object or reference, with access to its source repository.
///
/// Note that these are snapshots and won't recognize if they are stale.
pub struct Reference<'r, A> {
    pub(crate) backing: Option<reference::Backing>,
    pub(crate) access: &'r A,
}

#[cfg(not(feature = "local"))]
type PackCache = odb::pack::cache::Never;
#[cfg(feature = "local")]
type PackCache = odb::pack::cache::lru::StaticLinkedList<64>;

#[derive(Default)]
struct ModifieablePackedRefsBuffer {
    packed_refs: Option<refs::packed::Buffer>,
    modified: Option<SystemTime>,
}

/// State for use in `Easy*` to provide mutable parts of a repository such as caches and buffers.
#[derive(Default)]
pub struct State {
    /// As the packed-buffer may hold onto a memory map, we avoid that to exist once per thread, multiplying system resources, cloning
    /// it with every clone of the owning `Easy`.
    /// This seems worth the cost of always going through an `Arc<RwLock<…>>>`. Note that `EasyArcExclusive` uses the same construct
    /// but the reason we make this distinction at all is that there are other easy's that allows to chose exactly what you need in
    /// your application. `State` is one size fits all with supporting single-threaded applications only.
    packed_refs: Arc<parking_lot::RwLock<ModifieablePackedRefsBuffer>>,
    pack_cache: RefCell<PackCache>,
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
