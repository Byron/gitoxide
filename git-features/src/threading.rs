//! Type definitions for putting shared ownership and synchronized mutation behind the `threading` feature toggle.
//!
//! That way, single-threaded applications will not have to use thread-safe primitives, and simply do not specify the 'threading' feature.

#[cfg(feature = "threading")]
mod _impl {
    use std::sync::Arc;

    /// A reference counted pointer type for shared ownership.
    pub type OwnShared<T> = Arc<T>;
    /// A synchronization primitive which can start read-only and transition to support mutation.
    pub type MutableOnDemand<T> = parking_lot::RwLock<T>;

    /// Get an upgradable shared reference through a [`MutableOnDemand`] for read-only access.
    ///
    /// This access can be upgraded using [`upgrade_ref_to_mut()`].
    pub fn get_ref_upgradeable<T>(v: &MutableOnDemand<T>) -> parking_lot::RwLockUpgradableReadGuard<'_, T> {
        v.upgradable_read()
    }

    /// Get a shared reference through a [`MutableOnDemand`] for read-only access.
    pub fn get_ref<T>(v: &MutableOnDemand<T>) -> parking_lot::RwLockReadGuard<'_, T> {
        v.read()
    }

    /// Get a mutable reference through a [`MutableOnDemand`] for read-write access.
    pub fn get_mut<T>(v: &MutableOnDemand<T>) -> parking_lot::RwLockWriteGuard<'_, T> {
        v.write()
    }

    /// Upgrade a handle previously obtained with [`get_ref_upgradeable()`] to support mutation.
    pub fn upgrade_ref_to_mut<T>(
        v: parking_lot::RwLockUpgradableReadGuard<'_, T>,
    ) -> parking_lot::RwLockWriteGuard<'_, T> {
        parking_lot::RwLockUpgradableReadGuard::upgrade(v)
    }
}

#[cfg(not(feature = "threading"))]
mod _impl {
    use std::{
        cell::{Ref, RefCell, RefMut},
        rc::Rc,
    };

    /// A reference counted pointer type for shared ownership.
    pub type OwnShared<T> = Rc<T>;
    /// A synchronization primitive which can start read-only and transition to support mutation.
    pub type MutableOnDemand<T> = RefCell<T>;

    /// Get an upgradable shared reference through a [`MutableOnDemand`] for read-only access.
    ///
    /// This access can be upgraded using [`upgrade_ref_to_mut()`].
    pub fn get_ref_upgradeable<T>(v: &RefCell<T>) -> RefMut<'_, T> {
        v.borrow_mut()
    }

    /// Get a shared reference through a [`MutableOnDemand`] for read-only access.
    pub fn get_mut<T>(v: &RefCell<T>) -> RefMut<'_, T> {
        v.borrow_mut()
    }

    /// Get a mutable reference through a [`MutableOnDemand`] for read-write access.
    pub fn get_ref<T>(v: &RefCell<T>) -> Ref<'_, T> {
        v.borrow()
    }

    /// Upgrade a handle previously obtained with [`get_ref_upgradeable()`] to support mutation.
    pub fn upgrade_ref_to_mut<T>(v: RefMut<'_, T>) -> RefMut<'_, T> {
        v
    }
}

pub use _impl::*;
