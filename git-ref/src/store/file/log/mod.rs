use bstr::BStr;

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
    pub signature: git_actor::immutable::Signature<'a>,
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
}

///
pub mod iter;
mod line;
