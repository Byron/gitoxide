use crate::{
    mutable,
    store::{file, packed},
};
use std::convert::TryInto;

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references
pub struct Overlay<'p, 's> {
    _parent: &'s file::Store,
    _packed: packed::Iter<'p>,
    _loose: file::loose::iter::SortedLoosePaths,
}

/// A reference returned by the [`Overlay`] iterator.
pub enum Reference<'p, 's> {
    Packed(packed::Reference<'p>),
    Loose(file::Reference<'s>),
}

impl<'p, 's> Reference<'p, 's> {
    /// Returns true if this ref is located in a packed ref buffer.
    pub fn is_packed(&self) -> bool {
        match self {
            Reference::Packed(_) => true,
            Reference::Loose(_) => false,
        }
    }

    /// Return the full validated name of the reference. Please note that if the reference is packed, validation can fail here.
    pub fn name(&self) -> Result<mutable::FullName, git_validate::refname::Error> {
        match self {
            Reference::Packed(p) => p.full_name.try_into(),
            Reference::Loose(l) => Ok(l.name()),
        }
    }
}

impl<'p, 's> Iterator for Overlay<'p, 's> {
    type Item = Result<Reference<'p, 's>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("next item in the iteration")
    }
}

impl file::Store {
    /// Return an iterator over all references, loose or `packed`, sorted by their name.
    ///
    /// Note that the caller is responsible for the freshness of the `packed` references buffer.
    /// If a reference cannot be parsed or read, the error will be visible to the caller and the iteration
    /// continues.
    /// However, as the loose ref iteration has to be performed in advance, the operation will abort if the loose file iteartion
    /// fails. If this is undesirable, traverse loose refs directly using [`loose_iter()`][file::Store::loose_iter()].
    pub fn iter<'p, 's>(&'s self, packed: &'p packed::Buffer) -> std::io::Result<Overlay<'p, 's>> {
        Ok(Overlay {
            _parent: self,
            _packed: packed
                .iter()
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?,
            _loose: file::loose::iter::SortedLoosePaths::at_root_with_names(self.refs_dir(), self.base.clone()),
        })
    }
}

mod error {
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            TBD
        }
    }
}
pub use error::Error;
