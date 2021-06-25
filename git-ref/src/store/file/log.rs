#![allow(missing_docs, unused)]

use bstr::BStr;
use git_hash::ObjectId;

pub struct Line<'a> {
    previous_oid: ObjectId,
    new_oid: ObjectId,
    signature: (), // get from git-object including parsing
    message: &'a BStr,
}

mod decode {
    use crate::file::log::Line;
    use nom::IResult;

    pub fn line<'a>(bytes: &'a [u8]) -> IResult<&[u8], Line<'a>> {
        todo!("line parsing")
    }
}
