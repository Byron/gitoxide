use bstr::BStr;

pub use super::loose::reflog::{create_or_update, Error};

/// A parsed ref log line.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Line<'a> {
    /// The previous object id in hexadecimal. Use [`Line::previous_oid()`] to get a more usable form.
    pub previous_oid: &'a BStr,
    /// The new object id in hexadecimal. Use [`Line::new_oid()`] to get a more usable form.
    pub new_oid: &'a BStr,
    /// The signature of the currently configured committer.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub signature: git_actor::SignatureRef<'a>,
    /// The message providing details about the operation performed in this log line.
    pub message: &'a BStr,
}

///
pub mod mutable {
    use bstr::BString;
    use git_hash::ObjectId;

    /// A parsed ref log line that can be changed
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Line {
        /// The previous object id. Can be a null-sha to indicate this is a line for a new ref.
        pub previous_oid: ObjectId,
        /// The new object id. Can be a null-sha to indicate this ref is being deleted.
        pub new_oid: ObjectId,
        /// The signature of the currently configured committer.
        pub signature: git_actor::Signature,
        /// The message providing details about the operation performed in this log line.
        pub message: BString,
    }

    impl<'a> From<super::Line<'a>> for Line {
        fn from(v: super::Line<'a>) -> Self {
            Line {
                previous_oid: v.previous_oid(),
                new_oid: v.new_oid(),
                signature: v.signature.into(),
                message: v.message.into(),
            }
        }
    }

    impl<'a> super::Line<'a> {
        /// Convert this instance into its mutable counterpart
        pub fn to_mutable(&self) -> Line {
            self.clone().into()
        }
    }

    mod write {
        use std::io;

        use bstr::{BStr, ByteSlice};
        use quick_error::quick_error;

        use super::Line;

        quick_error! {
            /// The Error produced by [`Line::write_to()`] (but wrapped in an io error).
            #[derive(Debug)]
            #[allow(missing_docs)]
            enum Error {
                IllegalCharacter {
                    display("Messages must not contain newlines\\n")
                }
            }
        }

        impl From<Error> for io::Error {
            fn from(err: Error) -> Self {
                io::Error::new(io::ErrorKind::Other, err)
            }
        }

        /// Output
        impl Line {
            /// Serialize this instance to `out` in the git serialization format for ref log lines.
            pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
                write!(out, "{} {} ", self.previous_oid, self.new_oid)?;
                self.signature.write_to(&mut out)?;
                writeln!(out, "\t{}", check_newlines(self.message.as_ref())?)
            }
        }

        fn check_newlines(input: &BStr) -> Result<&BStr, Error> {
            if input.find_byte(b'\n').is_some() {
                return Err(Error::IllegalCharacter);
            }
            Ok(input)
        }
    }
}

///
pub mod iter;
mod line;
