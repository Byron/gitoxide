use git_object::borrowed;

/// A borrowed object using a borrowed slice as backing buffer.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Object<'a> {
    pub kind: git_object::Kind,
    pub data: &'a [u8],
}

impl<'a> Object<'a> {
    pub fn decode(&self) -> Result<borrowed::Object, borrowed::Error> {
        Ok(match self.kind {
            git_object::Kind::Tree => borrowed::Object::Tree(borrowed::Tree::from_bytes(self.data)?),
            git_object::Kind::Blob => borrowed::Object::Blob(borrowed::Blob { data: self.data }),
            git_object::Kind::Commit => borrowed::Object::Commit(borrowed::Commit::from_bytes(self.data)?),
            git_object::Kind::Tag => borrowed::Object::Tag(borrowed::Tag::from_bytes(self.data)?),
        })
    }
}

pub mod verify {
    use crate::{hash, loose};
    use git_object::{borrowed, owned};
    use quick_error::quick_error;
    use std::io;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            ChecksumMismatch(desired: owned::Id, actual: owned::Id) {
                display("Object expected to have id {}, but actual id was {}", desired, actual)
            }
        }
    }

    impl crate::borrowed::Object<'_> {
        pub fn verify_checksum(&self, desired: borrowed::Id) -> Result<(), Error> {
            let mut sink = hash::Write::new(io::sink(), desired.kind());

            loose::object::header::encode(self.kind, self.data.len() as u64, &mut sink).expect("hash to always work");
            sink.hash.update(&self.data);

            let actual_id = owned::Id::from(sink.hash.digest());
            if desired != actual_id.to_borrowed() {
                return Err(Error::ChecksumMismatch(desired.into(), actual_id));
            }
            Ok(())
        }
    }
}
