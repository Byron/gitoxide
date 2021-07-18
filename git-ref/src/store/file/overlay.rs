use crate::{
    mutable,
    store::{file, packed},
};
use bstr::BString;
use std::{cmp::Ordering, convert::TryInto, io::Read, iter::Peekable, path::PathBuf};

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references.
///
/// All errors will be returned verbatim, while packed errors are depleted first if loose refs also error.
pub struct LooseThenPacked<'p, 's> {
    parent: &'s file::Store,
    packed: Peekable<packed::Iter<'p>>,
    loose: Peekable<file::loose::iter::SortedLoosePaths>,
    buf: Vec<u8>,
}

/// A reference returned by the [`LooseThenPacked`] iterator.
pub enum Reference<'p, 's> {
    /// A reference originating in a pack
    Packed(packed::Reference<'p>),
    /// A reference from the filesystem
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

    /// Return the target to which the reference points to.
    pub fn target(&self) -> mutable::Target {
        match self {
            Reference::Packed(p) => mutable::Target::Peeled(p.target()),
            Reference::Loose(l) => l.target().to_owned(),
        }
    }
}

impl<'p, 's> LooseThenPacked<'p, 's> {
    fn convert_packed(
        &mut self,
        packed: Result<packed::Reference<'p>, packed::iter::Error>,
    ) -> Result<Reference<'p, 's>, Error> {
        packed.map(Reference::Packed).map_err(|err| match err {
            packed::iter::Error::Reference {
                invalid_line,
                line_number,
            } => Error::PackedReference {
                invalid_line,
                line_number,
            },
            packed::iter::Error::Header { .. } => unreachable!("this one only happens on iteration creation"),
        })
    }

    fn convert_loose(&mut self, res: std::io::Result<(PathBuf, Option<BString>)>) -> Result<Reference<'p, 's>, Error> {
        let (refpath, _name) = res.map_err(Error::Traversal)?;
        std::fs::File::open(&refpath)
            .and_then(|mut f| {
                self.buf.clear();
                f.read_to_end(&mut self.buf)
            })
            .map_err(Error::ReadFileContents)?;
        let relative_path = refpath.strip_prefix(&self.parent.base).expect("base contains path");
        file::Reference::try_from_path(self.parent, relative_path, &self.buf)
            .map_err(|err| Error::ReferenceCreation {
                err,
                relative_path: relative_path.into(),
            })
            .map(Reference::Loose)
    }
}

impl<'p, 's> Iterator for LooseThenPacked<'p, 's> {
    type Item = Result<Reference<'p, 's>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.loose.peek(), self.packed.peek()) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), Some(Err(_))) => {
                let res = self.packed.next().expect("peeked value exists");
                Some(self.convert_packed(res))
            }
            (Some(_), None) | (Some(Err(_)), Some(_)) => {
                let res = self.loose.next().expect("peeked value exists");
                Some(self.convert_loose(res))
            }
            (Some(Ok(loose)), Some(Ok(packed))) => {
                let loose_name = loose.1.as_ref().expect("name retrieval configured");
                match loose_name.as_slice().cmp(packed.full_name) {
                    Ordering::Less => {
                        let res = self.loose.next().expect("name retrieval configured");
                        Some(self.convert_loose(res))
                    }
                    Ordering::Equal => {
                        drop(self.packed.next());
                        let res = self.loose.next().expect("peeked value exists");
                        Some(self.convert_loose(res))
                    }
                    Ordering::Greater => {
                        let res = self.packed.next().expect("name retrieval configured");
                        Some(self.convert_packed(res))
                    }
                }
            }
        }
    }
}

impl file::Store {
    /// Return an iterator over all references, loose or `packed`, sorted by their name.
    ///
    /// Note that the caller is responsible for the freshness of the `packed` references buffer.
    /// If a reference cannot be parsed or read, the error will be visible to the caller and the iteration
    /// continues.
    ///
    /// Errors are returned similarly to what would happen when loose and packed refs where iterated by themeselves.
    pub fn iter<'p, 's>(&'s self, packed: &'p packed::Buffer) -> std::io::Result<LooseThenPacked<'p, 's>> {
        Ok(LooseThenPacked {
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
    use bstr::BString;
    use quick_error::quick_error;
    use std::{io, path::PathBuf};

    quick_error! {
        /// The error returned by the [`LooseThenPacked`][super::LooseThenPacked] iterator.
        #[derive(Debug)]
        #[allow(missing_docs)]
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
            PackedReference { invalid_line: BString, line_number: usize } {
                display("Invalid reference in line {}: '{}'", line_number, invalid_line)
            }
        }
    }
}
pub use error::Error;
