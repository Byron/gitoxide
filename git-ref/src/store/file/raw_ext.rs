use std::collections::BTreeSet;

use git_hash::ObjectId;

use crate::{
    peel,
    raw::Reference,
    store::{
        file,
        file::{log, loose::reference::logiter::must_be_io_err},
        packed,
    },
    Target,
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

    /// For details, see [Reference::peel_to_id_in_place()].
    fn peel_to_id_in_place<E: std::error::Error + Send + Sync + 'static>(
        &mut self,
        store: &file::Store,
        packed: Option<&packed::Buffer>,
        find: impl FnMut(git_hash::ObjectId, &mut Vec<u8>) -> Result<Option<(git_object::Kind, &[u8])>, E>,
    ) -> Result<ObjectId, peel::to_id::Error>;

    /// Follow this symbolic reference one level and return the ref it refers to,
    /// possibly providing access to `packed` references for lookup if it contains the referent.
    ///
    /// Returns `None` if this is not a symbolic reference, hence the leaf of the chain.
    fn follow(
        &self,
        store: &file::Store,
        packed: Option<&packed::Buffer>,
    ) -> Option<Result<Reference, file::find::existing::Error>>;
}

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

    fn peel_to_id_in_place<E: std::error::Error + Send + Sync + 'static>(
        &mut self,
        store: &file::Store,
        packed: Option<&packed::Buffer>,
        mut find: impl FnMut(git_hash::ObjectId, &mut Vec<u8>) -> Result<Option<(git_object::Kind, &[u8])>, E>,
    ) -> Result<ObjectId, peel::to_id::Error> {
        match self.peeled {
            Some(peeled) => {
                self.target = Target::Peeled(peeled.to_owned());
                Ok(peeled)
            }
            None => {
                if self.target.kind() == crate::Kind::Symbolic {
                    let mut seen = BTreeSet::new();
                    let cursor = &mut *self;
                    while let Some(next) = cursor.follow(store, packed) {
                        let next = next?;
                        if seen.contains(&next.name) {
                            return Err(peel::to_id::Error::Cycle(store.base.join(cursor.name.to_path())));
                        }
                        *cursor = next;
                        seen.insert(cursor.name.clone());
                        const MAX_REF_DEPTH: usize = 5;
                        if seen.len() == MAX_REF_DEPTH {
                            return Err(peel::to_id::Error::DepthLimitExceeded {
                                max_depth: MAX_REF_DEPTH,
                            });
                        }
                    }
                };
                let mut buf = Vec::new();
                let mut oid = self.target.as_id().expect("peeled ref").to_owned();
                let peeled_id = loop {
                    let (kind, data) = find(oid, &mut buf)
                        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)?
                        .ok_or_else(|| peel::to_id::Error::NotFound {
                            oid,
                            name: self.name.0.clone(),
                        })?;
                    match kind {
                        git_object::Kind::Tag => {
                            oid = git_object::TagRefIter::from_bytes(data).target_id().ok_or_else(|| {
                                peel::to_id::Error::NotFound {
                                    oid,
                                    name: self.name.0.clone(),
                                }
                            })?;
                        }
                        _ => break oid,
                    };
                };
                self.peeled = Some(peeled_id);
                self.target = Target::Peeled(peeled_id);
                Ok(peeled_id)
            }
        }
    }

    fn follow(
        &self,
        store: &file::Store,
        packed: Option<&packed::Buffer>,
    ) -> Option<Result<Reference, file::find::existing::Error>> {
        match self.peeled {
            Some(peeled) => Some(Ok(Reference {
                name: self.name.clone(),
                target: Target::Peeled(peeled),
                peeled: None,
            })),
            None => match &self.target {
                Target::Peeled(_) => None,
                Target::Symbolic(full_name) => {
                    let path = full_name.to_path();
                    match store.find_one_with_verified_input(path.as_ref(), packed) {
                        Ok(Some(next)) => Some(Ok(next)),
                        Ok(None) => Some(Err(file::find::existing::Error::NotFound(path.into_owned()))),
                        Err(err) => Some(Err(file::find::existing::Error::Find(err))),
                    }
                }
            },
        }
    }
}
