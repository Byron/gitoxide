#![forbid(unsafe_code, rust_2018_idioms)]

use git_hash::{oid, ObjectId};
use git_object::bstr::BStr;
use std::borrow::Cow;
use std::collections::HashMap;

pub mod describe;

#[allow(clippy::result_unit_err)]
pub fn describe<'a>(
    commit: &oid,
    hex_len: usize,
    name_set: &HashMap<ObjectId, Cow<'a, BStr>>,
) -> Result<Option<describe::Outcome<'a>>, describe::Error> {
    if let Some(name) = name_set.get(commit) {
        return Ok(Some(describe::Outcome {
            name: name.to_owned(),
            id: commit.to_owned(),
            hex_len,
            depth: 0,
            long: false,
            dirty_suffix: None,
        }));
    }
    todo!("actually search for it")
}
