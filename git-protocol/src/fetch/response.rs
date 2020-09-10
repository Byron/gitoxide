use crate::fetch::command::Feature;
use git_object::owned;
use git_transport::{client, Protocol};
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
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

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Acknowledgement {
    Common(owned::Id),
    Ready,
    NAK,
}

impl Acknowledgement {
    fn from_line(line: &str) -> Result<Acknowledgement, Error> {
        let mut tokens = line.trim_end().splitn(3, ' ');
        Ok(match (tokens.next(), tokens.next(), tokens.next()) {
            (Some(first), id, description) => match first {
                "ready" => Acknowledgement::Ready, // V2
                "NAK" => Acknowledgement::NAK,     // V1
                "ACK" => {
                    let id = match id {
                        Some(id) => owned::Id::from_40_bytes_in_hex(id.as_bytes())
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
    pub fn id(&self) -> Option<&owned::Id> {
        match self {
            Acknowledgement::Common(id) => Some(id),
            _ => None,
        }
    }
}

/// A representation of a complete fetch response
pub struct Response {
    acks: Vec<Acknowledgement>,
    has_pack: bool,
}

impl Response {
    pub fn has_pack(&self) -> bool {
        self.has_pack
    }
    pub fn check_required_features(version: Protocol, features: &[Feature]) -> Result<(), Error> {
        match version {
            Protocol::V1 => {
                let has = |name: &str| features.iter().any(|f| f.0 == name);
                // Let's focus on V2 standards, and simply not support old servers to keep our code simpler
                if !has("multi_ack_detailed") {
                    return Err(Error::MissingServerCapability("multi_ack_detailed"));
                }
                // It's easy to NOT do sideband for us, but then again, everyone supports it.
                if !has("side-band") && !has("side-band-64k") {
                    return Err(Error::MissingServerCapability("side-band OR side-band-64k"));
                }
            }
            Protocol::V2 => {}
        }
        Ok(())
    }
    pub fn from_line_reader(version: Protocol, reader: &mut impl client::ExtendedBufRead) -> Result<Response, Error> {
        match version {
            Protocol::V1 => {
                let mut line = String::new();
                let mut acks = Vec::<Acknowledgement>::new();
                let (acks, has_pack) = 'lines: loop {
                    line.clear();
                    let peeked_line = match reader.peek_data_line() {
                        Some(Ok(Ok(line))) => String::from_utf8_lossy(line),
                        // This special case (block) deals with a single NAK being a legitimate EOF sometimes
                        // Note that this might block forever in stateful connections as there it's not really clear
                        // if something will be following or not by just looking at the response. Instead you have to know
                        // the arguments sent to the server and count response lines based on intricate knowledge on how the
                        // server works.
                        // For now this is acceptable, as V2 can be used as a workaround, which also is the default.
                        Some(Err(err)) if err.kind() == io::ErrorKind::UnexpectedEof => break 'lines (acks, false),
                        Some(Err(err)) => return Err(err.into()),
                        Some(Ok(Err(err))) => return Err(err.into()),

                        None => break 'lines (acks, false), // EOF
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
                        Err(_) => break 'lines (acks, true),
                    };
                    assert_ne!(reader.read_line(&mut line)?, 0, "consuming a peeked line works");
                };
                Ok(Response { acks, has_pack })
            }
            Protocol::V2 => {
                // NOTE: We only read acknowledgements and scrub to the pack file, until we have use for the other features
                let mut line = String::new();
                reader.reset(Protocol::V2);
                let mut acks = None::<Vec<Acknowledgement>>;
                let (acks, has_pack) = 'section: loop {
                    line.clear();
                    if reader.read_line(&mut line)? == 0 {
                        return Err(Error::Io(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Could not read message headline",
                        )));
                    };

                    match line.trim_end() {
                        "acknowledgments" => {
                            let a = acks.get_or_insert_with(Vec::new);
                            line.clear();
                            while reader.read_line(&mut line)? != 0 {
                                a.push(Acknowledgement::from_line(&line)?);
                                line.clear();
                            }
                            // End of message, or end of section?
                            if reader.stopped_at() == Some(client::MessageKind::Delimiter) {
                                // try reading more sections
                                reader.reset(Protocol::V2);
                            } else {
                                // we are done, there is no pack
                                break 'section (acks.expect("initialized acknowledgements vector"), false);
                            }
                        }
                        "packfile" => {
                            // what follows is the packfile itself, which can be read with a sideband enabled reader
                            break 'section (acks.unwrap_or_default(), true);
                        }
                        _ => return Err(Error::UnknownSectionHeader(line)),
                    }
                };
                Ok(Response { acks, has_pack })
            }
        }
    }

    pub fn acknowledgements(&self) -> &[Acknowledgement] {
        &self.acks
    }
}
