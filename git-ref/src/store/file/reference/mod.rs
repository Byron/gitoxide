use crate::{file::Reference, Kind, Target};
use bstr::BString;
use git_hash::{oid, ObjectId};

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub(in crate::file) enum State {
    Id(ObjectId),
    ValidatedPath(BString),
}

impl State {
    fn as_id(&self) -> Option<&oid> {
        match self {
            State::Id(id) => Some(id),
            State::ValidatedPath(_) => None,
        }
    }
}

impl<'a> Reference<'a> {
    /// Return the kind of ref.
    pub fn kind(&self) -> Kind {
        match self.state {
            State::ValidatedPath(_) => Kind::Symbolic,
            State::Id(_) => Kind::Peeled,
        }
    }
    /// Return the target to which this instance is pointing.
    pub fn target(&'a self) -> Target<'a> {
        match self.state {
            State::ValidatedPath(ref path) => Target::Symbolic(path.as_ref()),
            State::Id(ref oid) => Target::Peeled(oid.as_ref()),
        }
    }
}

mod iter {
    #![allow(missing_docs)]

    use crate::{
        store::file::{log, loose, Reference},
        FullName,
    };
    use bstr::ByteSlice;
    use std::{convert::TryInto, io::Read};

    impl<'a> Reference<'a> {
        pub fn log_iter<'b>(
            &self,
            buf: &'b mut Vec<u8>,
        ) -> Result<Option<impl Iterator<Item = Result<log::Line<'b>, log::iter::decode::Error>>>, loose::reflog::Error>
        {
            // NOTE: Have to repeat the implementation of store::reflog_iter here as borrow_check believes impl Iterator binds self
            use os_str_bytes::OsStrBytes;
            let name = self.relative_path.as_path().to_raw_bytes();
            let name: FullName<'_> = name.as_bstr().try_into().expect("infallible operation");
            match std::fs::File::open(self.parent.reflog_path(name)) {
                Ok(mut file) => {
                    buf.clear();
                    file.read_to_end(buf)?;
                }
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                Err(err) => return Err(err.into()),
            };
            Ok(Some(log::iter::forward(buf)))
        }
    }
}

///
pub mod peel;

///
pub mod decode;
