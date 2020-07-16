//! Owned objects for use with serialization.

pub(crate) const NL: &[u8; 1] = b"\n";
pub(crate) const SPACE: &[u8; 1] = b" ";

mod convert;
mod ser;

mod id;
pub use id::*;
mod tag;
pub use tag::Tag;

pub mod tree {
    use crate::owned::SPACE;
    use crate::{owned, TreeMode};
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
}
pub use tree::Tree;

mod commit {
    use crate::owned::{self, ser, NL};
    use bstr::{BString, ByteSlice};
    use smallvec::SmallVec;
    use std::io;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Commit {
        pub tree: owned::Id,
        // SHA1 of each parent commit. Empty for first commit in repository.
        pub parents: SmallVec<[owned::Id; 1]>,
        pub author: owned::Signature,
        pub committer: owned::Signature,
        // The name of the message encoding, otherwise UTF-8 should be assumed.
        pub encoding: Option<BString>,
        pub message: BString,
        pub pgp_signature: Option<BString>,
    }

    impl Commit {
        pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
            ser::trusted_header_id(b"tree", &self.tree, &mut out)?;
            for parent in &self.parents {
                ser::trusted_header_id(b"parent", parent, &mut out)?;
            }
            ser::trusted_header_signature(b"author", &self.author, &mut out)?;
            ser::trusted_header_signature(b"committer", &self.committer, &mut out)?;
            if let Some(encoding) = self.encoding.as_ref() {
                ser::header_field(b"encoding", encoding, &mut out)?;
            }
            if let Some(signature) = self.pgp_signature.as_ref() {
                let has_newline = signature.find_byte(b'\n').is_some();
                if has_newline {
                    ser::header_field_multi_line(b"gpgsig", signature, &mut out)?;
                } else {
                    ser::trusted_header_field(b"gpgsig", signature, &mut out)?;
                }
            }
            out.write_all(NL)?;
            out.write_all(&self.message)
        }
    }
}
pub use commit::Commit;

mod object;
pub use object::*;
