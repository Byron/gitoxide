use crate::{owned, owned::SPACE, TreeMode};
use bstr::{BString, ByteSlice};
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NoEntries {
            description("Trees must have at least one entry")
        }
        NewlineInFilename(name: BString){
            display("Newlines are invalid in file paths: {:?}", name)
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree {
    pub entries: Vec<Entry>,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    pub mode: TreeMode,
    pub filename: BString,
    pub oid: owned::Id,
}

impl TreeMode {
    pub fn as_bytes(&self) -> &'static [u8] {
        use TreeMode::*;
        match self {
            Tree => b"40000",
            Blob => b"100644",
            BlobExecutable => b"100755",
            Link => b"120000",
            Commit => b"160000",
        }
    }
}

impl Tree {
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        if self.entries.is_empty() {
            return Err(Error::NoEntries.into());
        }
        for Entry { mode, filename, oid } in &self.entries {
            out.write_all(mode.as_bytes())?;
            out.write_all(SPACE)?;

            if filename.find_byte(b'\n').is_some() {
                return Err(Error::NewlineInFilename(filename.to_owned()).into());
            }
            out.write_all(&filename)?;
            out.write_all(&[b'\0'])?;

            out.write_all(&oid[..])?;
        }
        Ok(())
    }
}
