#![forbid(unsafe_code, rust_2018_idioms)]

use git_hash::{oid, ObjectId};
use git_object::bstr::BStr;
use std::borrow::Cow;
use std::collections::HashSet;

pub fn describe(_commit: &oid, _name_set: &HashSet<(ObjectId, Cow<'_, BStr>)>) {
    todo!()
}
