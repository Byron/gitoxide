use git_hash::ObjectId;

use crate::{FullName, Target};

/// A fully owned backend agnostic reference
#[derive(Debug, Clone)]
pub struct Reference {
    /// The path to uniquely identify this ref within its store.
    pub name: FullName,
    /// The target of the reference, either a symbolic reference by full name or a possibly intermediate object by its id.
    pub target: Target,
    /// The fully peeled object to which this reference ultimately points to. Only guaranteed to be set after `peel_to_id_in_place()` was called.
    pub peeled: Option<ObjectId>,
}

mod convert {
    use git_hash::ObjectId;

    use crate::{
        raw::Reference,
        store::{file::loose, packed},
        Target,
    };

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

mod access {
    use bstr::ByteSlice;

    use crate::{raw::Reference, FullNameRef, Namespace};

    impl Reference {
        /// Returns the kind of reference based on its target
        pub fn kind(&self) -> crate::Kind {
            self.target.kind()
        }

        /// Return the full validated name of the reference, with the given namespace stripped if possible.
        ///
        /// If the reference name wasn't prefixed with `namespace`, `None` is returned instead.
        pub fn name_without_namespace(&self, namespace: &Namespace) -> Option<FullNameRef<'_>> {
            self.name
                .0
                .as_bstr()
                .strip_prefix(namespace.0.as_bstr().as_ref())
                .map(|stripped| FullNameRef(stripped.as_bstr()))
        }
    }
}

// TODO: peeling depends on file store, that should be generic but we don't have a trait for that yet
mod peel {
    use std::collections::BTreeSet;

    use git_hash::ObjectId;

    use crate::{
        peel,
        raw::Reference,
        store::{file, packed},
        Target,
    };

    impl Reference {
        /// For details, see [Reference::peel_to_id_in_place()].
        pub fn peel_to_id_in_place<E: std::error::Error + Send + Sync + 'static>(
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
                    Ok(self.target.as_id().expect("to be peeled").to_owned())
                }
            }
        }

        /// Follow this symbolic reference one level and return the ref it refers to,
        /// possibly providing access to `packed` references for lookup if it contains the referent.
        ///
        /// Returns `None` if this is not a symbolic reference, hence the leaf of the chain.
        pub fn follow(
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_reference() {
        assert_eq!(
            std::mem::size_of::<Reference>(),
            80,
            "let's not let it change size undetected"
        );
    }
}
