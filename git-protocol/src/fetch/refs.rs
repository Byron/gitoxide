use crate::fetch::Error;
use bstr::{BString, ByteSlice};
use git_object::owned;
use std::io;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Ref {
    /// A ref pointing to a `tag` object, which in turns points to an `object`, usually a commit
    Peeled {
        path: BString,
        tag: owned::Id,
        object: owned::Id,
    },
    /// A ref pointing to a commit object
    Direct { path: BString, object: owned::Id },
    /// A symbolic ref pointing to `target` ref, which in turn points to an `object`
    Symbolic {
        path: BString,
        target: BString,
        object: owned::Id,
    },
    /// extracted from V1 capabilities, which contain some important symbolic refs along with their targets
    /// These don't contain the Id
    SymbolicForLookup { path: BString, target: BString },
}

pub(crate) fn from_capabilities(out_refs: &mut Vec<Ref>, symrefs: Vec<BString>) -> Result<(), Error> {
    for symref in symrefs.into_iter() {
        let (left, right) = symref.split_at(
            symref
                .find_byte(b':')
                .ok_or_else(|| Error::MalformedSymref(symref.clone()))?,
        );
        out_refs.push(Ref::SymbolicForLookup {
            path: left.into(),
            target: right[1..].into(),
        })
    }
    Ok(())
}

pub(crate) fn from_v1_refs_received_as_part_of_handshake(
    _out_refs: &mut Vec<Ref>,
    _in_refs: &mut dyn io::BufRead,
) -> Result<(), Error> {
    unimplemented!("todo")
}
