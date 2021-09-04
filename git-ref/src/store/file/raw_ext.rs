use crate::store::{
    file,
    file::{log, loose::reference::logiter::must_be_io_err},
};

pub trait Sealed {}
impl Sealed for crate::Reference {}

/// A trait to extend [Reference][crate::Reference] with functionality requiring a [file::Store].
pub trait ReferenceExt: Sealed {
    /// Obtain a reverse iterator over logs of this reference. See [crate::file::loose::Reference::log_iter_rev()] for details.
    fn log_iter_rev<'b>(
        &self,
        store: &file::Store,
        buf: &'b mut [u8],
    ) -> std::io::Result<Option<log::iter::Reverse<'b, std::fs::File>>>;

    /// Obtain an iterator over logs of this reference. See [crate::file::loose::Reference::log_iter()] for details.
    fn log_iter<'a, 'b: 'a>(
        &'a self,
        store: &file::Store,
        buf: &'b mut Vec<u8>,
    ) -> std::io::Result<Option<log::iter::Forward<'b>>>;

    /// For details, see [Reference::log_exists()].
    fn log_exists(&self, store: &file::Store) -> bool;
}

use crate::raw::Reference;
impl ReferenceExt for Reference {
    fn log_iter_rev<'b>(
        &self,
        store: &file::Store,
        buf: &'b mut [u8],
    ) -> std::io::Result<Option<log::iter::Reverse<'b, std::fs::File>>> {
        store.reflog_iter_rev(self.name.to_ref(), buf).map_err(must_be_io_err)
    }

    fn log_iter<'a, 'b: 'a>(
        &'a self,
        store: &file::Store,
        buf: &'b mut Vec<u8>,
    ) -> std::io::Result<Option<log::iter::Forward<'b>>> {
        store.reflog_iter(self.name.to_ref(), buf).map_err(must_be_io_err)
    }

    fn log_exists(&self, store: &file::Store) -> bool {
        store
            .reflog_exists(self.name.to_ref())
            .expect("infallible name conversion")
    }
}
