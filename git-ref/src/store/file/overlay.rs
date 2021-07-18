use crate::store::{file, packed};
use bstr::BString;

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references
pub struct Overlay<'p, 's> {
    parent: &'s file::Store,
    packed: packed::Iter<'p>,
    loose_paths_sorted: Vec<BString>,
}

impl file::Store {
    /// Return an iterator over all references, loose or `packed`, sorted by their name.
    ///
    /// If a reference cannot be parsed or read, the error will be visible to the caller and the iteration
    /// continues.
    pub fn iter<'p, 's>(&'s self, packed: &'p packed::Buffer) -> std::io::Result<Overlay<'p, 's>> {
        Ok(Overlay {
            parent: self,
            packed: packed
                .iter()
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?,
            loose_paths_sorted: self.loose_full_names_sorted()?,
        })
    }

    fn loose_full_names_sorted(&self) -> std::io::Result<Vec<BString>> {
        // file::loose::iter::LoosePaths::at_root(self.refs_dir(), self.base.clone())
        todo!("figure this out")
    }
}
