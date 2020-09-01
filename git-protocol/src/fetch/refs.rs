use crate::fetch::Error;
use bstr::{BString, ByteSlice};
use git_object::owned;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Ref {
    Tag {
        path: BString,
        id: owned::Id,
    },
    Commit {
        path: BString,
        id: owned::Id,
    },
    Symbolic {
        path: BString,
        target: BString,
        id: owned::Id,
    },
    /// extracted from V1 capabilities, which contain some important symbolic refs along with their targets
    /// These don't contain the Id
    SymbolicForLookup {
        path: BString,
        target: BString,
    },
}

pub(crate) fn extract_symrefs(out_refs: &mut Vec<Ref>, symrefs: Vec<BString>) -> Result<(), Error> {
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
