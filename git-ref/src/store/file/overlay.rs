use crate::{
    mutable,
    store::{file, packed},
};
use std::{convert::TryInto, io::Read, iter::Peekable, path::PathBuf};

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references
pub struct Overlay<'p, 's> {
    parent: &'s file::Store,
    packed: Peekable<packed::Iter<'p>>,
    loose: Peekable<file::loose::iter::SortedLoosePaths>,
    buf: Vec<u8>,
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

impl<'p, 's> Overlay<'p, 's> {
    fn convert_to_loose_ref(&mut self, path: PathBuf) -> Result<Reference<'p, 's>, Error> {
        std::fs::File::open(&path)
            .and_then(|mut f| {
                self.buf.clear();
                f.read_to_end(&mut self.buf)
            })
            .map_err(Error::ReadFileContents)?;
        let relative_path = path.strip_prefix(&self.parent.base).expect("bsae contains path");
        file::Reference::try_from_path(self.parent, relative_path, &self.buf)
            .map_err(|err| Error::ReferenceCreation {
                err,
                relative_path: relative_path.into(),
            })
            .map(Reference::Loose)
    }
}

impl<'p, 's> Iterator for Overlay<'p, 's> {
    type Item = Result<Reference<'p, 's>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.loose.peek(), self.packed.peek()) {
            (None, None) => None,
            (Some(_), None) => {
                let res = self.loose.next().expect("peeked value exists");
                Some(
                    res.map_err(Error::Traversal)
                        .and_then(|(refpath, _name)| self.convert_to_loose_ref(refpath)),
                )
            }
            (None, Some(_packed)) => todo!("packed only"),
            (Some(_loose), Some(_packed)) => todo!("some packed and some loose"),
        }
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
            parent: self,
            packed: packed
                .iter()
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                .peekable(),
            loose: file::loose::iter::SortedLoosePaths::at_root_with_names(self.refs_dir(), self.base.clone())
                .peekable(),
            buf: Vec::new(),
        })
    }
}

mod error {
    use crate::store::file;
    use quick_error::quick_error;
    use std::{io, path::PathBuf};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Traversal(err: io::Error) {
                display("The file system could not be traversed")
                source(err)
            }
            ReadFileContents(err: io::Error) {
                display("The ref file could not be read in full")
                source(err)
            }
            ReferenceCreation{ err: file::reference::decode::Error, relative_path: PathBuf } {
                display("The reference at '{}' could not be instantiated", relative_path.display())
                source(err)
            }
        }
    }
}
pub use error::Error;
