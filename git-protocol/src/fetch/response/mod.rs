use std::io;

use bstr::BString;
use git_transport::{client, Protocol};
use quick_error::quick_error;

use crate::fetch::command::Feature;

quick_error! {
    /// The error used in the [response module][crate::fetch::response].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        Io(err: io::Error) {
            display("Failed to read from line reader")
            from()
            source(err)
        }
        Transport(err: client::Error) {
            display("An error occurred when decoding a line")
            from()
            source(err)
        }
        MissingServerCapability(feature: &'static str) {
            display("Currently we require feature '{}', which is not supported by the server", feature)
        }
        UnknownLineType(line: String) {
            display("Encountered an unknown line prefix in '{}'", line)
        }
        UnknownSectionHeader(header: String) {
            display("Unknown or unsupported header: '{}'", header)
        }
    }
}

/// An 'ACK' line received from the server.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Acknowledgement {
    /// The contained `id` is in common.
    Common(git_hash::ObjectId),
    /// The server is ready to receive more lines.
    Ready,
    /// The server isn't ready yet.
    Nak,
}

/// A shallow line received from the server.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ShallowUpdate {
    /// Shallow the given `id`.
    Shallow(git_hash::ObjectId),
    /// Don't shallow the given `id` anymore.
    Unshallow(git_hash::ObjectId),
}

/// A wanted-ref line received from the server.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct WantedRef {
    /// The object id of the wanted ref, as seen by the server.
    pub id: git_hash::ObjectId,
    /// The name of the ref, as requested by the client as a `want-ref` argument.
    pub path: BString,
}

impl ShallowUpdate {
    /// Parse a `ShallowUpdate` from a `line` as received to the server.
    pub fn from_line(line: &str) -> Result<ShallowUpdate, Error> {
        match line.trim_end().split_once(' ') {
            Some((prefix, id)) => {
                let id =
                    git_hash::ObjectId::from_hex(id.as_bytes()).map_err(|_| Error::UnknownLineType(line.to_owned()))?;
                Ok(match prefix {
                    "shallow" => ShallowUpdate::Shallow(id),
                    "unshallow" => ShallowUpdate::Unshallow(id),
                    _ => return Err(Error::UnknownLineType(line.to_owned())),
                })
            }
            None => Err(Error::UnknownLineType(line.to_owned())),
        }
    }
}

impl Acknowledgement {
    /// Parse an `Acknowledgement` from a `line` as received to the server.
    pub fn from_line(line: &str) -> Result<Acknowledgement, Error> {
        let mut tokens = line.trim_end().splitn(3, ' ');
        match (tokens.next(), tokens.next(), tokens.next()) {
            (Some(first), id, description) => Ok(match first {
                "ready" => Acknowledgement::Ready, // V2
                "NAK" => Acknowledgement::Nak,     // V1
                "ACK" => {
                    let id = match id {
                        Some(id) => git_hash::ObjectId::from_hex(id.as_bytes())
                            .map_err(|_| Error::UnknownLineType(line.to_owned()))?,
                        None => return Err(Error::UnknownLineType(line.to_owned())),
                    };
                    if let Some(description) = description {
                        match description {
                            "common" => {}
                            "ready" => return Ok(Acknowledgement::Ready),
                            _ => return Err(Error::UnknownLineType(line.to_owned())),
                        }
                    }
                    Acknowledgement::Common(id)
                }
                _ => return Err(Error::UnknownLineType(line.to_owned())),
            }),
            (None, _, _) => Err(Error::UnknownLineType(line.to_owned())),
        }
    }
    /// Returns the hash of the acknowledged object if this instance acknowledges a common one.
    pub fn id(&self) -> Option<&git_hash::ObjectId> {
        match self {
            Acknowledgement::Common(id) => Some(id),
            _ => None,
        }
    }
}

impl WantedRef {
    /// Parse a `WantedRef` from a `line` as received from the server.
    pub fn from_line(line: &str) -> Result<WantedRef, Error> {
        match line.trim_end().split_once(' ') {
            Some((id, path)) => {
                let id =
                    git_hash::ObjectId::from_hex(id.as_bytes()).map_err(|_| Error::UnknownLineType(line.to_owned()))?;
                Ok(WantedRef { id, path: path.into() })
            }
            None => Err(Error::UnknownLineType(line.to_owned())),
        }
    }
}

/// A representation of a complete fetch response
pub struct Response {
    acks: Vec<Acknowledgement>,
    shallows: Vec<ShallowUpdate>,
    wanted_refs: Vec<WantedRef>,
    has_pack: bool,
}

impl Response {
    /// Return true if the response has a pack which can be read next.
    pub fn has_pack(&self) -> bool {
        self.has_pack
    }

    /// Return an error if the given `features` don't contain the required ones for the given `version` of the protocol.
    ///
    /// Even though technically any set of features supported by the server could work, we only implement the ones that
    /// make it easy to maintain all versions with a single code base that aims to be and remain maintainable.
    pub fn check_required_features(version: Protocol, features: &[Feature]) -> Result<(), Error> {
        match version {
            Protocol::V1 => {
                let has = |name: &str| features.iter().any(|f| f.0 == name);
                // Let's focus on V2 standards, and simply not support old servers to keep our code simpler
                if !has("multi_ack_detailed") {
                    return Err(Error::MissingServerCapability("multi_ack_detailed"));
                }
                // It's easy to NOT do sideband for us, but then again, everyone supports it.
                // CORRECTION: If side-band is off, it would send the packfile without packet line encoding,
                // which is nothing we ever want to deal with (despite it being more efficient). In V2, this
                // is not even an option anymore, sidebands are always present.
                if !has("side-band") && !has("side-band-64k") {
                    return Err(Error::MissingServerCapability("side-band OR side-band-64k"));
                }
            }
            Protocol::V2 => {}
        }
        Ok(())
    }

    /// Return all acknowledgements [parsed previously][Response::from_line_reader()].
    pub fn acknowledgements(&self) -> &[Acknowledgement] {
        &self.acks
    }

    /// Return all shallow update lines [parsed previously][Response::from_line_reader()].
    pub fn shallow_updates(&self) -> &[ShallowUpdate] {
        &self.shallows
    }

    /// Return all wanted-refs [parsed previously][Response::from_line_reader()].
    pub fn wanted_refs(&self) -> &[WantedRef] {
        &self.wanted_refs
    }
}

#[cfg(any(feature = "async-client", feature = "blocking-client"))]
impl Response {
    /// with a friendly server, we just assume that a non-ack line is a pack line
    /// which is our hint to stop here.
    fn parse_v1_ack_or_shallow_or_assume_pack(
        acks: &mut Vec<Acknowledgement>,
        shallows: &mut Vec<ShallowUpdate>,
        peeked_line: &str,
    ) -> bool {
        match Acknowledgement::from_line(peeked_line) {
            Ok(ack) => match ack.id() {
                Some(id) => {
                    if !acks.iter().any(|a| a.id() == Some(id)) {
                        acks.push(ack);
                    }
                }
                None => acks.push(ack),
            },
            Err(_) => match ShallowUpdate::from_line(peeked_line) {
                Ok(shallow) => {
                    shallows.push(shallow);
                }
                Err(_) => return true,
            },
        };
        false
    }
}

#[cfg(feature = "async-client")]
mod async_io;
#[cfg(feature = "blocking-client")]
mod blocking_io;
