use crate::fetch::command::Feature;
use git_transport::{client, Protocol};
use quick_error::quick_error;
use std::io;

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

impl ShallowUpdate {
    /// Parse a `ShallowUpdate` from a `line` as received to the server.
    fn from_line(line: &str) -> Result<ShallowUpdate, Error> {
        let mut tokens = line.trim_end().splitn(2, ' ');
        match (tokens.next(), tokens.next()) {
            (Some(prefix), Some(id)) => {
                let id =
                    git_hash::ObjectId::from_hex(id.as_bytes()).map_err(|_| Error::UnknownLineType(line.to_owned()))?;
                Ok(match prefix {
                    "shallow" => ShallowUpdate::Shallow(id),
                    "unshallow" => ShallowUpdate::Unshallow(id),
                    _ => return Err(Error::UnknownLineType(line.to_owned())),
                })
            }
            _ => unreachable!("cannot have an entirely empty line"),
        }
    }
}

impl Acknowledgement {
    /// Parse an `Acknowledgement` from a `line` as received to the server.
    fn from_line(line: &str) -> Result<Acknowledgement, Error> {
        let mut tokens = line.trim_end().splitn(3, ' ');
        Ok(match (tokens.next(), tokens.next(), tokens.next()) {
            (Some(first), id, description) => match first {
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
            },
            (None, _, _) => unreachable!("cannot have an entirely empty line"),
        })
    }
    /// Returns the hash of the acknowledged object if this instance acknowledges a common one.
    pub fn id(&self) -> Option<&git_hash::ObjectId> {
        match self {
            Acknowledgement::Common(id) => Some(id),
            _ => None,
        }
    }
}

/// A representation of a complete fetch response
pub struct Response {
    acks: Vec<Acknowledgement>,
    shallows: Vec<ShallowUpdate>,
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
    /// Parse a response of the given `version` of the protocol from `reader`.
    pub fn from_line_reader(version: Protocol, reader: &mut impl client::ExtendedBufRead) -> Result<Response, Error> {
        match version {
            Protocol::V1 => {
                let mut line = String::new();
                let mut acks = Vec::<Acknowledgement>::new();
                let mut shallows = Vec::<ShallowUpdate>::new();
                let has_pack = 'lines: loop {
                    line.clear();
                    let peeked_line = match reader.peek_data_line() {
                        Some(Ok(Ok(line))) => String::from_utf8_lossy(line),
                        // This special case (hang/block forver) deals with a single NAK being a legitimate EOF sometimes
                        // Note that this might block forever in stateful connections as there it's not really clear
                        // if something will be following or not by just looking at the response. Instead you have to know
                        // the arguments sent to the server and count response lines based on intricate knowledge on how the
                        // server works.
                        // For now this is acceptable, as V2 can be used as a workaround, which also is the default.
                        Some(Err(err)) if err.kind() == io::ErrorKind::UnexpectedEof => break 'lines false,
                        Some(Err(err)) => return Err(err.into()),
                        Some(Ok(Err(err))) => return Err(err.into()),
                        None => {
                            // maybe we saw a shallow flush packet, let's reset and retry
                            reader.read_line(&mut line)?;
                            reader.reset(Protocol::V1);
                            match reader.peek_data_line() {
                                Some(Ok(Ok(line))) => String::from_utf8_lossy(line),
                                Some(Err(err)) => return Err(err.into()),
                                Some(Ok(Err(err))) => return Err(err.into()),
                                None => break 'lines false, // EOF
                            }
                        }
                    };

                    // with a friendly server, we just assume that a non-ack line is a pack line
                    // which is our hint to stop here.
                    match Acknowledgement::from_line(&peeked_line) {
                        Ok(ack) => match ack.id() {
                            Some(id) => {
                                if !acks.iter().any(|a| a.id() == Some(id)) {
                                    acks.push(ack);
                                }
                            }
                            None => acks.push(ack),
                        },
                        Err(_) => match ShallowUpdate::from_line(&peeked_line) {
                            Ok(shallow) => {
                                shallows.push(shallow);
                            }
                            Err(_) => break 'lines true,
                        },
                    };
                    assert_ne!(reader.read_line(&mut line)?, 0, "consuming a peeked line works");
                };
                Ok(Response {
                    acks,
                    shallows,
                    has_pack,
                })
            }
            Protocol::V2 => {
                // NOTE: We only read acknowledgements and scrub to the pack file, until we have use for the other features
                let mut line = String::new();
                reader.reset(Protocol::V2);
                let mut acks = Vec::<Acknowledgement>::new();
                let mut shallows = Vec::<ShallowUpdate>::new();
                let has_pack = 'section: loop {
                    line.clear();
                    if reader.read_line(&mut line)? == 0 {
                        return Err(Error::Io(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Could not read message headline",
                        )));
                    };

                    match line.trim_end() {
                        "acknowledgments" => {
                            if parse_section(&mut line, reader, &mut acks, Acknowledgement::from_line)? {
                                break 'section false;
                            }
                        }
                        "shallow-info" => {
                            if parse_section(&mut line, reader, &mut shallows, ShallowUpdate::from_line)? {
                                break 'section false;
                            }
                        }
                        "packfile" => {
                            // what follows is the packfile itself, which can be read with a sideband enabled reader
                            break 'section true;
                        }
                        _ => return Err(Error::UnknownSectionHeader(line)),
                    }
                };
                Ok(Response {
                    acks,
                    shallows,
                    has_pack,
                })
            }
        }
    }

    /// Return all acknowledgements [parsed previously][Response::from_line_reader()].
    pub fn acknowledgements(&self) -> &[Acknowledgement] {
        &self.acks
    }

    /// Return all shallow update lines [parsed previously][Response::from_line_reader()].
    pub fn shallow_updates(&self) -> &[ShallowUpdate] {
        &self.shallows
    }
}

fn parse_section<T>(
    line: &mut String,
    reader: &mut impl client::ExtendedBufRead,
    res: &mut Vec<T>,
    parse: impl Fn(&str) -> Result<T, Error>,
) -> Result<bool, Error> {
    line.clear();
    while reader.read_line(line)? != 0 {
        res.push(parse(line)?);
        line.clear();
    }
    // End of message, or end of section?
    Ok(if reader.stopped_at() == Some(client::MessageKind::Delimiter) {
        // try reading more sections
        reader.reset(Protocol::V2);
        false
    } else {
        // we are done, there is no pack
        true
    })
}
