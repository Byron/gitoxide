use crate::store::{file, packed};
use bstr::BString;
use std::path::PathBuf;

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references
pub struct Overlay<'p, 's> {
    _parent: &'s file::Store,
    _packed: packed::Iter<'p>,
    _loose_paths_sorted: Vec<(PathBuf, BString)>,
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
            _loose_paths_sorted: self.loose_full_names_sorted()?,
        })
    }

    fn loose_full_names_sorted(&self) -> std::io::Result<Vec<(PathBuf, BString)>> {
        let mut names = file::loose::iter::LoosePaths::at_root_with_names(self.refs_dir(), self.base.clone())
            .map(|r| r.map(|(path, name)| (path, name.expect("name is set as we configured it"))))
            .collect::<Result<Vec<_>, _>>()?;
        names.sort();
        Ok(names)
    }
}
