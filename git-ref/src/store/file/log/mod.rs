use bstr::BStr;

pub use super::loose::reflog::{create_or_update, Error};

///
pub mod iter;
mod line;

/// A parsed ref log line.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct LineRef<'a> {
    /// The previous object id in hexadecimal. Use [`LineRef::previous_oid()`] to get a more usable form.
    pub previous_oid: &'a BStr,
    /// The new object id in hexadecimal. Use [`LineRef::new_oid()`] to get a more usable form.
    pub new_oid: &'a BStr,
    /// The signature of the currently configured committer.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub signature: git_actor::SignatureRef<'a>,
    /// The message providing details about the operation performed in this log line.
    pub message: &'a BStr,
}
