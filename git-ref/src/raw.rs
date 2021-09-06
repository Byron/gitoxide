use git_hash::ObjectId;

use crate::{FullName, Target};

/// A fully owned backend agnostic reference
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
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
    use git_object::bstr::ByteSlice;

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

        /// Strip the given namespace from our name as well as the name, but not the reference we point to.
        ///
        /// Symbolic link targets must remain as is or else the reference cannot be peeled without knowing the namespace.
        pub fn strip_namespace(&mut self, namespace: &Namespace) -> &mut Self {
            self.name.strip_namespace(namespace);
            self
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
