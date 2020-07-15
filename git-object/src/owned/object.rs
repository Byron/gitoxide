use crate::Time;
use bstr::BString;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature {
    pub name: BString,
    pub email: BString,
    pub time: Time,
}
