use bstr::BString;
use quick_error::quick_error;
use std::io;

quick_error! {
    /// The error returned when parsing References/refs from the server response.
    #[derive(Debug)]
    #[allow(missing_docs)]
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
        tag: git_hash::ObjectId,
        /// The hash of the object the `tag` points to.
        object: git_hash::ObjectId,
    },
    /// A ref pointing to a commit object
    Direct {
        /// The path at which the ref is located, like `/refs/heads/main`.
        path: BString,
        /// The hash of the object the ref points to.
        object: git_hash::ObjectId,
    },
    /// A symbolic ref pointing to `target` ref, which in turn points to an `object`
    Symbolic {
        /// The path at which the symbolic ref is located, like `/refs/heads/main`.
        path: BString,
        /// The path of the ref the symbolic ref points to.
        target: BString,
        /// The hash of the object the `target` ref points to.
        object: git_hash::ObjectId,
    },
}

impl Ref {
    /// Provide shared fields referring to the ref itself, namely `(path, object id)`.
    /// In case of peeled refs, the tag object itself is returned as it is what the path refers to.
    pub fn unpack(&self) -> (&BString, &git_hash::ObjectId) {
        match self {
            Ref::Direct { path, object, .. }
            | Ref::Peeled { path, tag: object, .. } // the tag acts as reference
            | Ref::Symbolic { path, object, .. } => (path, object),
        }
    }
}

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub(crate) mod shared {
    use crate::fetch::{refs, Ref};
    use bstr::{BString, ByteSlice};

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
            tag: git_hash::ObjectId,
            object: git_hash::ObjectId,
        },
        /// A ref pointing to a commit object
        Direct { path: BString, object: git_hash::ObjectId },
        /// A symbolic ref pointing to `target` ref, which in turn points to an `object`
        Symbolic {
            path: BString,
            target: BString,
            object: git_hash::ObjectId,
        },
        /// extracted from V1 capabilities, which contain some important symbolic refs along with their targets
        /// These don't contain the Id
        SymbolicForLookup { path: BString, target: BString },
    }

    impl InternalRef {
        fn unpack_direct(self) -> Option<(BString, git_hash::ObjectId)> {
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
        capabilities: impl Iterator<Item = git_transport::client::capabilities::Capability<'a>>,
    ) -> Result<Vec<InternalRef>, refs::Error> {
        let mut out_refs = Vec::new();
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
                    .ok_or_else(|| refs::Error::MalformedSymref(symref.to_owned()))?,
            );
            if left.is_empty() || right.is_empty() {
                return Err(refs::Error::MalformedSymref(symref.to_owned()));
            }
            out_refs.push(InternalRef::SymbolicForLookup {
                path: left.into(),
                target: right[1..].into(),
            })
        }
        Ok(out_refs)
    }

    pub(in crate::fetch::refs) fn parse_v1(
        num_initial_out_refs: usize,
        out_refs: &mut Vec<InternalRef>,
        line: &str,
    ) -> Result<(), refs::Error> {
        let trimmed = line.trim_end();
        let (hex_hash, path) = trimmed.split_at(
            trimmed
                .find(' ')
                .ok_or_else(|| refs::Error::MalformedV1RefLine(trimmed.to_owned()))?,
        );
        let path = &path[1..];
        if path.is_empty() {
            return Err(refs::Error::MalformedV1RefLine(trimmed.to_owned()));
        }
        match path.strip_suffix("^{}") {
            Some(stripped) => {
                let (previous_path, tag) =
                    out_refs
                        .pop()
                        .and_then(InternalRef::unpack_direct)
                        .ok_or(refs::Error::InvariantViolation(
                            "Expecting peeled refs to be preceded by direct refs",
                        ))?;
                if previous_path != stripped {
                    return Err(refs::Error::InvariantViolation(
                        "Expecting peeled refs to have the same base path as the previous, unpeeled one",
                    ));
                }
                out_refs.push(InternalRef::Peeled {
                    path: previous_path,
                    tag,
                    object: git_hash::ObjectId::from_hex(hex_hash.as_bytes())?,
                });
            }
            None => {
                let object = git_hash::ObjectId::from_hex(hex_hash.as_bytes())?;
                match out_refs
                    .iter()
                    .take(num_initial_out_refs)
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
        Ok(())
    }

    pub(in crate::fetch::refs) fn parse_v2(line: &str) -> Result<Ref, refs::Error> {
        let trimmed = line.trim_end();
        let mut tokens = trimmed.splitn(3, ' ');
        match (tokens.next(), tokens.next()) {
            (Some(hex_hash), Some(path)) => {
                let id = git_hash::ObjectId::from_hex(hex_hash.as_bytes())?;
                if path.is_empty() {
                    return Err(refs::Error::MalformedV2RefLine(trimmed.to_owned()));
                }
                Ok(if let Some(attribute) = tokens.next() {
                    let mut tokens = attribute.splitn(2, ':');
                    match (tokens.next(), tokens.next()) {
                        (Some(attribute), Some(value)) => {
                            if value.is_empty() {
                                return Err(refs::Error::MalformedV2RefLine(trimmed.to_owned()));
                            }
                            match attribute {
                                "peeled" => Ref::Peeled {
                                    path: path.into(),
                                    object: git_hash::ObjectId::from_hex(value.as_bytes())?,
                                    tag: id,
                                },
                                "symref-target" => Ref::Symbolic {
                                    path: path.into(),
                                    object: id,
                                    target: value.into(),
                                },
                                _ => {
                                    return Err(refs::Error::UnkownAttribute(attribute.to_owned(), trimmed.to_owned()))
                                }
                            }
                        }
                        _ => return Err(refs::Error::MalformedV2RefLine(trimmed.to_owned())),
                    }
                } else {
                    Ref::Direct {
                        object: id,
                        path: path.into(),
                    }
                })
            }
            _ => Err(refs::Error::MalformedV2RefLine(trimmed.to_owned())),
        }
    }
}

#[cfg(feature = "async-client")]
mod async_io {
    use crate::fetch::{refs, Ref};
    use futures_io::AsyncBufRead;
    use futures_lite::AsyncBufReadExt;

    /// Parse refs from the given input line by line. Protocol V2 is required for this to succeed.
    pub async fn from_v2_refs(in_refs: &mut (dyn AsyncBufRead + Unpin)) -> Result<Vec<Ref>, refs::Error> {
        let mut out_refs = Vec::new();
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = in_refs.read_line(&mut line).await?;
            if bytes_read == 0 {
                break;
            }
            out_refs.push(refs::shared::parse_v2(&line)?);
        }
        Ok(out_refs)
    }

    /// Parse refs from the return stream of the handshake as well as the server capabilities, also received as part of the
    /// handshake.
    /// Together they form a complete set of refs.
    ///
    /// # Note
    ///
    /// Symbolic refs are shoe-horned into server capabilities whereas refs (without symbolic ones) are sent automatically as
    /// part of the handshake. Both symbolic and peeled refs need to be combined to fit into the [`Ref`] type provided here.
    pub async fn from_v1_refs_received_as_part_of_handshake_and_capabilities<'a>(
        in_refs: &mut (dyn AsyncBufRead + Unpin),
        capabilities: impl Iterator<Item = git_transport::client::capabilities::Capability<'a>>,
    ) -> Result<Vec<Ref>, refs::Error> {
        let mut out_refs = refs::shared::from_capabilities(capabilities)?;
        let number_of_possible_symbolic_refs_for_lookup = out_refs.len();
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = in_refs.read_line(&mut line).await?;
            if bytes_read == 0 {
                break;
            }
            refs::shared::parse_v1(number_of_possible_symbolic_refs_for_lookup, &mut out_refs, &line)?;
        }
        Ok(out_refs.into_iter().map(Into::into).collect())
    }
}
#[cfg(feature = "async-client")]
pub use async_io::{from_v1_refs_received_as_part_of_handshake_and_capabilities, from_v2_refs};

#[cfg(feature = "blocking-client")]
mod blocking_io {
    use crate::fetch::{refs, Ref};
    use std::io;

    /// Parse refs from the given input line by line. Protocol V2 is required for this to succeed.
    pub fn from_v2_refs(in_refs: &mut dyn io::BufRead) -> Result<Vec<Ref>, refs::Error> {
        let mut out_refs = Vec::new();
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = in_refs.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            out_refs.push(refs::shared::parse_v2(&line)?);
        }
        Ok(out_refs)
    }

    /// Parse refs from the return stream of the handshake as well as the server capabilities, also received as part of the
    /// handshake.
    /// Together they form a complete set of refs.
    ///
    /// # Note
    ///
    /// Symbolic refs are shoe-horned into server capabilities whereas refs (without symbolic ones) are sent automatically as
    /// part of the handshake. Both symbolic and peeled refs need to be combined to fit into the [`Ref`] type provided here.
    pub fn from_v1_refs_received_as_part_of_handshake_and_capabilities<'a>(
        in_refs: &mut dyn io::BufRead,
        capabilities: impl Iterator<Item = git_transport::client::capabilities::Capability<'a>>,
    ) -> Result<Vec<Ref>, refs::Error> {
        let mut out_refs = refs::shared::from_capabilities(capabilities)?;
        let number_of_possible_symbolic_refs_for_lookup = out_refs.len();
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = in_refs.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            refs::shared::parse_v1(number_of_possible_symbolic_refs_for_lookup, &mut out_refs, &line)?;
        }
        Ok(out_refs.into_iter().map(Into::into).collect())
    }
}
#[cfg(feature = "blocking-client")]
pub use blocking_io::{from_v1_refs_received_as_part_of_handshake_and_capabilities, from_v2_refs};
