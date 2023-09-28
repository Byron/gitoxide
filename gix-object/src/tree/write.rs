use std::io;

use bstr::{BString, ByteSlice};

use crate::{
    encode::SPACE,
    tree::{Entry, EntryRef},
    Kind, Tree, TreeRef,
};

/// The Error used in [`Tree::write_to()`][crate::WriteTo::write_to()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Newlines are invalid in file paths: {name:?}")]
    NewlineInFilename { name: BString },
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

/// Serialization
impl crate::WriteTo for Tree {
    /// Serialize this tree to `out` in the git internal format.
    fn write_to(&self, out: &mut dyn io::Write) -> io::Result<()> {
        debug_assert_eq!(
            &{
                let mut entries_sorted = self.entries.clone();
                entries_sorted.sort();
                entries_sorted
            },
            &self.entries,
            "entries for serialization must be sorted by filename"
        );
        for Entry { mode, filename, oid } in &self.entries {
            out.write_all(mode.as_bytes())?;
            out.write_all(SPACE)?;

            if filename.find_byte(b'\n').is_some() {
                return Err(Error::NewlineInFilename {
                    name: (*filename).to_owned(),
                }
                .into());
            }
            out.write_all(filename)?;
            out.write_all(&[b'\0'])?;

            out.write_all(oid.as_bytes())?;
        }
        Ok(())
    }

    fn kind(&self) -> Kind {
        Kind::Tree
    }

    fn size(&self) -> u64 {
        self.entries
            .iter()
            .map(|Entry { mode, filename, oid }| {
                (mode.as_bytes().len() + 1 + filename.len() + 1 + oid.as_bytes().len()) as u64
            })
            .sum()
    }
}

/// Serialization
impl<'a> crate::WriteTo for TreeRef<'a> {
    /// Serialize this tree to `out` in the git internal format.
    fn write_to(&self, out: &mut dyn io::Write) -> io::Result<()> {
        debug_assert_eq!(
            &{
                let mut entries_sorted = self.entries.clone();
                entries_sorted.sort();
                entries_sorted
            },
            &self.entries,
            "entries for serialization must be sorted by filename"
        );
        for EntryRef { mode, filename, oid } in &self.entries {
            out.write_all(mode.as_bytes())?;
            out.write_all(SPACE)?;

            if filename.find_byte(b'\n').is_some() {
                return Err(Error::NewlineInFilename {
                    name: (*filename).to_owned(),
                }
                .into());
            }
            out.write_all(filename)?;
            out.write_all(&[b'\0'])?;

            out.write_all(oid.as_bytes())?;
        }
        Ok(())
    }

    fn kind(&self) -> Kind {
        Kind::Tree
    }

    fn size(&self) -> u64 {
        self.entries
            .iter()
            .map(|EntryRef { mode, filename, oid }| {
                (mode.as_bytes().len() + 1 + filename.len() + 1 + oid.as_bytes().len()) as u64
            })
            .sum()
    }
}
