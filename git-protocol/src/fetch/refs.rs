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
        Id(err: git_hash::decode::Error) {
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
        MalformedV2RefLine(line: String) {
            display("'{}' could not be parsed. A V2 ref line should be '<hex-hash> <path>[ (peeled|symref-target):<value>'.", line)
        }
        UnkownAttribute(attribute: String, line: String) {
            display("The ref attribute '{}' is unknown. Found in line '{}'", attribute, line)
        }
        InvariantViolation(message: &'static str) {
            display("{}", message)
        }
    }
}

/// A git reference, commonly referred to as 'ref', as returned by a git server before sending a pack.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Ref {
    /// A ref pointing to a `tag` object, which in turns points to an `object`, usually a commit
    Peeled {
        /// The path at which the ref is located, like `/refs/heads/main`.
        path: BString,
        /// The hash of the tag the ref points to.
        tag: owned::Id,
        /// The hash of the object the `tag` points to.
        object: owned::Id,
    },
    /// A ref pointing to a commit object
    Direct {
        /// The path at which the ref is located, like `/refs/heads/main`.
        path: BString,
        /// The hash of the object the ref points to.
        object: owned::Id,
    },
    /// A symbolic ref pointing to `target` ref, which in turn points to an `object`
    Symbolic {
        /// The path at which the symbolic ref is located, like `/refs/heads/main`.
        path: BString,
        /// The path of the ref the symbolic ref points to.
        target: BString,
        /// The hash of the object the `target` ref points to.
        object: owned::Id,
    },
}

impl Ref {
    /// Provide shared fields referring to the ref itself, namely `(path, object id)`.
    /// In case of peeled refs, the tag object itself is returned as it is what the path refers to.
    pub fn unpack(&self) -> (&BString, &owned::Id) {
        match self {
            Ref::Direct { path, object, .. }
            | Ref::Peeled { path, tag: object, .. } // the tag acts as reference
            | Ref::Symbolic { path, object, .. } => (path, object),
        }
    }
}

impl From<InternalRef> for Ref {
    fn from(v: InternalRef) -> Self {
        match v {
            InternalRef::Symbolic { path, target, object } => Ref::Symbolic { path, target, object },
            InternalRef::Peeled { path, tag, object } => Ref::Peeled { path, tag, object },
            InternalRef::Direct { path, object } => Ref::Direct { path, object },
            InternalRef::SymbolicForLookup { .. } => {
                unreachable!("this case should have been removed during processing")
            }
        }
    }
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug, Clone))]
pub(crate) enum InternalRef {
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

impl InternalRef {
    fn unpack_direct(self) -> Option<(BString, owned::Id)> {
        match self {
            InternalRef::Direct { path, object } => Some((path, object)),
            _ => None,
        }
    }
    fn lookup_symbol_has_path(&self, predicate_path: &str) -> bool {
        matches!(self, InternalRef::SymbolicForLookup { path, .. } if path == predicate_path)
    }
}

pub(crate) fn from_capabilities<'a>(
    out_refs: &mut Vec<InternalRef>,
    capabilities: impl Iterator<Item = git_transport::client::capabilities::Capability<'a>>,
) -> Result<(), Error> {
    let symref_values = capabilities.filter_map(|c| {
        if c.name() == b"symref".as_bstr() {
            c.value().map(ToOwned::to_owned)
        } else {
            None
        }
    });
    for symref in symref_values {
        let (left, right) = symref.split_at(
            symref
                .find_byte(b':')
                .ok_or_else(|| Error::MalformedSymref(symref.to_owned()))?,
        );
        if left.is_empty() || right.is_empty() {
            return Err(Error::MalformedSymref(symref.to_owned()));
        }
        out_refs.push(InternalRef::SymbolicForLookup {
            path: left.into(),
            target: right[1..].into(),
        })
    }
    Ok(())
}

pub(crate) fn from_v2_refs(out_refs: &mut Vec<Ref>, in_refs: &mut dyn io::BufRead) -> Result<(), Error> {
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = in_refs.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let trimmed = line.trim_end();
        let mut tokens = trimmed.splitn(3, ' ');
        match (tokens.next(), tokens.next()) {
            (Some(hex_hash), Some(path)) => {
                let id = owned::Id::from_40_bytes_in_hex(hex_hash.as_bytes())?;
                if path.is_empty() {
                    return Err(Error::MalformedV2RefLine(trimmed.to_owned()));
                }
                out_refs.push(if let Some(attribute) = tokens.next() {
                    let mut tokens = attribute.splitn(2, ':');
                    match (tokens.next(), tokens.next()) {
                        (Some(attribute), Some(value)) => {
                            if value.is_empty() {
                                return Err(Error::MalformedV2RefLine(trimmed.to_owned()));
                            }
                            match attribute {
                                "peeled" => Ref::Peeled {
                                    path: path.into(),
                                    object: owned::Id::from_40_bytes_in_hex(value.as_bytes())?,
                                    tag: id,
                                },
                                "symref-target" => Ref::Symbolic {
                                    path: path.into(),
                                    object: id,
                                    target: value.into(),
                                },
                                _ => return Err(Error::UnkownAttribute(attribute.to_owned(), trimmed.to_owned())),
                            }
                        }
                        _ => return Err(Error::MalformedV2RefLine(trimmed.to_owned())),
                    }
                } else {
                    Ref::Direct {
                        object: id,
                        path: path.into(),
                    }
                });
            }
            _ => return Err(Error::MalformedV2RefLine(trimmed.to_owned())),
        }
    }
    Ok(())
}

pub(crate) fn from_v1_refs_received_as_part_of_handshake(
    out_refs: &mut Vec<InternalRef>,
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
        match path.strip_suffix("^{}") {
            Some(stripped) => {
                let (previous_path, tag) =
                    out_refs
                        .pop()
                        .and_then(InternalRef::unpack_direct)
                        .ok_or(Error::InvariantViolation(
                            "Expecting peeled refs to be preceded by direct refs",
                        ))?;
                if previous_path != stripped {
                    return Err(Error::InvariantViolation(
                        "Expecting peeled refs to have the same base path as the previous, unpeeled one",
                    ));
                }
                out_refs.push(InternalRef::Peeled {
                    path: previous_path,
                    tag,
                    object: owned::Id::from_40_bytes_in_hex(hex_hash.as_bytes())?,
                });
            }
            None => {
                let object = owned::Id::from_40_bytes_in_hex(hex_hash.as_bytes())?;
                match out_refs
                    .iter()
                    .take(number_of_possible_symbolic_refs_for_lookup)
                    .position(|r| r.lookup_symbol_has_path(path))
                {
                    Some(position) => match out_refs.swap_remove(position) {
                        InternalRef::SymbolicForLookup { path: _, target } => out_refs.push(InternalRef::Symbolic {
                            path: path.into(),
                            object,
                            target,
                        }),
                        _ => unreachable!("Bug in lookup_symbol_has_path - must return lookup symbols"),
                    },
                    None => out_refs.push(InternalRef::Direct {
                        object,
                        path: path.into(),
                    }),
                };
            }
        }
    }
    Ok(())
}
