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

impl Ref {
    fn unpack_direct(self) -> Option<(BString, owned::Id)> {
        match self {
            Ref::Direct { path, object } => Some((path, object)),
            _ => None,
        }
    }
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
    out_refs: &mut Vec<Ref>,
    in_refs: &mut dyn io::BufRead,
) -> Result<(), Error> {
    let index_to_possible_symbolic_refs_for_lookup = out_refs.len();
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = in_refs.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let (hex_hash, path) = line.split_at(line.find(' ').ok_or_else(|| Error::MalformedV1RefLine(line.clone()))?);
        if path.ends_with("^{}") {
            let (previous_path, tag) = out_refs
                .pop()
                .and_then(Ref::unpack_direct)
                .ok_or_else(|| Error::InvariantViolation("Expecting peeled refs to be preceeded by direct refs"))?;
            if previous_path != &path[..path.len() - "^{}".len()] {
                return Err(Error::InvariantViolation(
                    "Expecting peeled refs to have the same base path as the previous, unpeeled one",
                ));
            }
            out_refs.push(Ref::Peeled {
                path: previous_path,
                tag,
                object: owned::Id::from_40_bytes_in_hex(hex_hash.as_bytes())
                    .map_err(|_| Error::MalformedV1RefLine(line.clone()))?,
            });
        } else {
            out_refs.push(Ref::Direct {
                object: owned::Id::from_40_bytes_in_hex(hex_hash.as_bytes())
                    .map_err(|_| Error::MalformedV1RefLine(line.clone()))?,
                path: path.into(),
            });
        }
    }
    unimplemented!("todo")
}
