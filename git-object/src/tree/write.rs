use std::io;

use bstr::{BString, ByteSlice};
use quick_error::quick_error;

use crate::{mutable::SPACE, tree::Entry, Tree};

quick_error! {
    /// The Error used in [`Tree::write_to()`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        NewlineInFilename(name: BString) {
            display("Newlines are invalid in file paths: {:?}", name)
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

/// Serialization
impl Tree {
    /// Serialize this tree to `out` in the git internal format.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
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
                return Err(Error::NewlineInFilename(filename.to_owned()).into());
            }
            out.write_all(filename)?;
            out.write_all(&[b'\0'])?;

            out.write_all(oid.as_bytes())?;
        }
        Ok(())
    }
}
