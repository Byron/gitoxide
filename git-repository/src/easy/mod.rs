#![allow(missing_docs)]
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

use crate::{hash::ObjectId, objs, odb, refs, Repository};

mod impls;

pub(crate) mod ext;

pub mod borrow;
pub mod object;
mod oid;
pub mod reference;
pub mod state;

pub struct Oid<'r, A> {
    id: ObjectId,
    access: &'r A,
}

pub struct ObjectRef<'repo, A> {
    pub id: ObjectId,
    pub kind: objs::Kind,
    pub data: std::cell::Ref<'repo, [u8]>,
    access: &'repo A,
}

pub struct TreeRef<'repo, A> {
    pub id: ObjectId,
    pub data: std::cell::Ref<'repo, [u8]>,
    access: &'repo A,
}

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    pub kind: objs::Kind,
    pub data: Vec<u8>,
}

pub struct Reference<'r, A> {
    pub(crate) backing: Option<reference::Backing>,
    pub(crate) access: &'r A,
}

#[cfg(not(feature = "local"))]
type PackCache = odb::pack::cache::Never;
#[cfg(feature = "local")]
type PackCache = odb::pack::cache::lru::StaticLinkedList<64>;

/// State for use in `Easy*` to provide mutable parts of a repository such as caches and buffers.
#[derive(Default)]
pub struct State {
    packed_refs: RefCell<Option<refs::packed::Buffer>>,
    pack_cache: RefCell<PackCache>,
    buf: RefCell<Vec<u8>>,
}

pub trait Access {
    type RepoRef: Deref<Target = Repository>;
    // TODO: Once GATs become stable, try to use them to make it work with RefCells too, aka EasyExclusive
    type RepoRefMut: DerefMut<Target = Repository>;

    fn repo(&self) -> std::result::Result<Self::RepoRef, borrow::repo::Error>;
    /// # NOTE
    ///
    /// This is implemented only for `EasyArcExclusive` to be obtained via `to_easy_arc_exclusive()`
    fn repo_mut(&self) -> std::result::Result<Self::RepoRefMut, borrow::repo::Error>;
    fn state(&self) -> &State;
}

pub type Result<T> = std::result::Result<T, borrow::state::Error>;
