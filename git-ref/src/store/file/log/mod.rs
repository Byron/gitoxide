#![allow(missing_docs, unused)]

use bstr::BStr;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Line<'a> {
    /// The previous object id in hexadecimal. Use [`Line::previous_oid()`] to get a more usable form.
    pub previous_oid: &'a BStr,
    /// The new object id in hexadecimal. Use [`Line::new_oid()`] to get a more usable form.
    pub new_oid: &'a BStr,
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub signature: git_actor::immutable::Signature<'a>,
    pub message: &'a BStr,
    _prevent_initialization: (),
}

mod line;
