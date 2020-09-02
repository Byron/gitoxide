use bstr::{BString, ByteSlice};
use git_object::owned;
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred while reading refs from the server")
            from()
            source(err)
        }
        Id(err: owned::Error) {
            display("Failed to hex-decode object hash")
            from()
            source(err)
        }
        MalformedSymref(symref: BString) {
            display("'{}' could not be parsed. A symref is expected to look like <NAME>:<target>.", symref)
        }
        MalformedV1RefLine(line: String) {
            display("'{}' could not be parsed. A V1 ref line should be '<hex-hash> <path>'.", line)
        }
        InvariantViolation(message: &'static str) {
            display("{}", message)
        }
    }
}

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
    fn lookup_symbol_has_path(&self, predicate_path: &str) -> bool {
        match self {
            Ref::SymbolicForLookup { path, .. } if path == predicate_path => true,
            _ => false,
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
        if left.is_empty() || right.is_empty() {
            return Err(Error::MalformedSymref(symref));
        }
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
    let number_of_possible_symbolic_refs_for_lookup = out_refs.len();
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = in_refs.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let trimmed = line.trim_end();
        let (hex_hash, path) = trimmed.split_at(
            trimmed
                .find(' ')
                .ok_or_else(|| Error::MalformedV1RefLine(trimmed.to_owned()))?,
        );
        let path = &path[1..];
        if path.is_empty() {
            return Err(Error::MalformedV1RefLine(trimmed.to_owned()));
        }
        if path.ends_with("^{}") {
            let (previous_path, tag) = out_refs
                .pop()
                .and_then(Ref::unpack_direct)
                .ok_or_else(|| Error::InvariantViolation("Expecting peeled refs to be preceeded by direct refs"))?;
            if previous_path != path[..path.len() - "^{}".len()] {
                return Err(Error::InvariantViolation(
                    "Expecting peeled refs to have the same base path as the previous, unpeeled one",
                ));
            }
            out_refs.push(Ref::Peeled {
                path: previous_path,
                tag,
                object: owned::Id::from_40_bytes_in_hex(hex_hash.as_bytes())?,
            });
        } else {
            let object = owned::Id::from_40_bytes_in_hex(hex_hash.as_bytes())?;
            match out_refs
                .iter()
                .take(number_of_possible_symbolic_refs_for_lookup)
                .position(|r| r.lookup_symbol_has_path(path))
            {
                Some(position) => match out_refs.swap_remove(position) {
                    Ref::SymbolicForLookup { path: _, target } => out_refs.push(Ref::Symbolic {
                        path: path.into(),
                        object,
                        target,
                    }),
                    _ => unreachable!("Bug in lookup_symbol_has_path - must return lookup symbols"),
                },
                None => out_refs.push(Ref::Direct {
                    object,
                    path: path.into(),
                }),
            };
        }
    }
    Ok(())
}
