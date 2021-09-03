use git_hash::ObjectId;

use crate::{FullName, Target};

/// A fully owned backend agnostic reference
#[derive(Debug, Clone)]
pub struct Reference {
    /// The path to uniquely identify this ref within its store.
    pub name: FullName,
    /// The target of the reference, either a symbolic reference by full name or a possibly intermediate object by its id.
    pub target: Target,
    /// The fully peeled object to which this reference ultimately points to
    peeled: Option<ObjectId>,
}

mod convert {
    use crate::raw::Reference;
    use crate::store::file::loose;
    use crate::store::packed;
    use crate::Target;
    use git_hash::ObjectId;

    impl From<Reference> for loose::Reference {
        fn from(value: Reference) -> Self {
            loose::Reference {
                name: value.name,
                target: value.target,
            }
        }
    }

    impl From<loose::Reference> for Reference {
        fn from(value: loose::Reference) -> Self {
            Reference {
                name: value.name,
                target: value.target,
                peeled: None,
            }
        }
    }

    impl<'p> From<packed::Reference<'p>> for Reference {
        fn from(value: packed::Reference<'p>) -> Self {
            Reference {
                name: value.name.into(),
                target: Target::Peeled(value.target()),
                peeled: value
                    .object
                    .map(|hex| ObjectId::from_hex(hex).expect("parser validation")),
            }
        }
    }
}

// TODO: peeling depends on file store, that should be generic but we don't have a trait for that yet
mod log {
    use crate::raw::Reference;
    use crate::store::file;
    use crate::store::file::log;
    use crate::store::file::loose::reference::logiter::must_be_io_err;

    impl Reference {
        /// Obtain a reverse iterator over logs of this reference. See [crate::file::loose::Reference::log_iter_rev()] for details.
        pub fn log_iter_rev<'b>(
            &self,
            store: &file::Store,
            buf: &'b mut [u8],
        ) -> std::io::Result<Option<log::iter::Reverse<'b, std::fs::File>>> {
            store.reflog_iter_rev(self.name.to_ref(), buf).map_err(must_be_io_err)
        }

        /// Obtain an iterator over logs of this reference. See [crate::file::loose::Reference::log_iter()] for details.
        pub fn log_iter<'a, 'b: 'a>(
            &'a self,
            store: &file::Store,
            buf: &'b mut Vec<u8>,
        ) -> std::io::Result<Option<impl Iterator<Item = Result<log::LineRef<'b>, log::iter::decode::Error>> + 'a>>
        {
            store.reflog_iter(self.name.to_ref(), buf).map_err(must_be_io_err)
        }

        /// For details, see [loose::Reference::log_exists()].
        pub fn log_exists(&self, store: &file::Store) -> bool {
            store
                .reflog_exists(self.name.to_ref())
                .expect("infallible name conversion")
        }
    }
}

mod access {
    use crate::raw::Reference;
    use crate::{FullNameRef, Namespace};
    use bstr::ByteSlice;

    impl Reference {
        /// Returns the kind of reference based on its target
        pub fn kind(&self) -> crate::Kind {
            self.target.kind()
        }

        /// Return the full validated name of the reference, which may include a namespace.
        pub fn name(&self) -> FullNameRef<'_> {
            self.name.to_ref()
        }

        /// Return the full validated name of the reference, with the given namespace stripped if possible.
        ///
        /// If the reference name wasn't prefixed with `namespace`, `None` is returned instead.
        pub fn name_without_namespace(&self, namespace: &Namespace) -> Option<FullNameRef<'_>> {
            self.name()
                .0
                .as_bstr()
                .strip_prefix(namespace.0.as_bstr().as_ref())
                .map(|stripped| FullNameRef(stripped.as_bstr()))
        }
    }
}

// TODO: peeling depends on file store, that should be generic but we don't have a trait for that yet
mod peel {
    use crate::raw::Reference;
    use crate::store::{file, file::loose, packed};
    use crate::{FullName, Target};
    use git_hash::ObjectId;

    impl Reference {
        /// For details, see [crate::file::loose::Reference::peel_to_id_in_place].
        pub fn peel_to_id_in_place<E: std::error::Error + Send + Sync + 'static>(
            &mut self,
            store: &file::Store,
            packed: Option<&packed::Buffer>,
            find: impl FnMut(git_hash::ObjectId, &mut Vec<u8>) -> Result<Option<(git_object::Kind, &[u8])>, E>,
        ) -> Result<ObjectId, crate::store::file::loose::reference::peel::to_id::Error> {
            match self.peeled.take() {
                Some(peeled) => {
                    self.target = Target::Peeled(peeled);
                    Ok(peeled)
                }
                None => {
                    let mut loose_self = loose::Reference {
                        name: FullName(std::mem::take(&mut self.name.0)),
                        target: std::mem::replace(&mut self.target, Target::Symbolic(FullName(Default::default()))),
                    };
                    let res = loose_self
                        .peel_to_id_in_place(store, packed, find)
                        .map(ToOwned::to_owned);
                    self.name = loose_self.name;
                    self.target = loose_self.target;
                    res
                }
            }
        }

        /// For details, see [crate::file::loose::Reference::follow_symbolic()].
        pub fn peel_one_level(
            &self,
            _store: &file::Store,
            _packed: Option<&packed::Buffer>,
        ) -> Option<Result<Reference, crate::store::file::loose::reference::peel::Error>> {
            match self.peeled {
                Some(peeled) => Some(Ok(Reference {
                    name: self.name.clone(),
                    target: Target::Peeled(peeled),
                    peeled: None,
                })),
                None => {
                    match self.target {
                        Target::Peeled(_) => None,
                        Target::Symbolic(_) => {
                            let _loose_self: loose::Reference = self.clone().into();
                            todo!("get rid of file::Reference")
                            // loose_self.follow_symbolic(store, packed)
                        }
                    }
                }
            }
        }
    }
}
