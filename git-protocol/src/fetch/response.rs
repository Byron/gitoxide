use crate::fetch::command::Feature;
use git_object::owned;
use git_transport::{client, Protocol};
use quick_error::quick_error;
use std::{io, io::BufRead};

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
}

impl Response {
    pub fn check_required_features(features: &[Feature]) -> Result<(), Error> {
        let has = |name: &str| features.iter().any(|f| f.0 == name);
        // Let's focus on V2 standards, and simply not support old servers to keep our code simpler
        if !has("multi_ack_detailed") {
            return Err(Error::MissingServerCapability("multi_ack_detailed"));
        }
        // It's easy to NOT do sideband for us, but then again, everyone supports it.
        if !has("side-band") && !has("side-band-64k") {
            return Err(Error::MissingServerCapability("side-band OR side-band-64k"));
        }
        Ok(())
    }
    pub fn from_line_reader(
        version: Protocol,
        mut reader: Box<dyn client::ExtendedBufRead + '_>,
    ) -> Result<Response, Error> {
        match version {
            Protocol::V1 => {
                enum State {
                    AtFirstCheckClone,
                    ParseAcks(Vec<Acknowledgement>),
                }
                let mut state = State::AtFirstCheckClone;
                let mut line = String::new();
                let acks = loop {
                    line.clear();
                    match state {
                        State::AtFirstCheckClone => {
                            if reader.read_line(&mut line)? == 0 {
                                return Err(Error::Io(io::Error::new(
                                    io::ErrorKind::UnexpectedEof,
                                    "Could not read a single line",
                                )));
                            };
                            let acks = vec![Acknowledgement::from_line(&line)?];
                            match acks.last().expect("one ack present") {
                                Acknowledgement::NAK => {
                                    // we are a clone (or so we think) as the first line is a NAK
                                    // We expect the following to be the pack
                                    break acks;
                                }
                                _ => {
                                    state = State::ParseAcks(acks);
                                }
                            }
                        }
                        State::ParseAcks(mut acks) => {
                            let peeked_line = match reader.peek_data_line() {
                                Some(line) => String::from_utf8_lossy(line??),
                                None => break acks, // EOF
                            };
                            // assuming a cooperative server, we just assume that a non-ack line is a pack line
                            // which is our hint to stop here.
                            let ack = match Acknowledgement::from_line(&peeked_line) {
                                Ok(ack) => ack,
                                Err(_) => break acks,
                            };
                            assert_ne!(reader.read_line(&mut line)?, 0, "consuming a peeked line works");
                            match ack.id() {
                                Some(id) => {
                                    if !acks.iter().any(|a| a.id() == Some(id)) {
                                        acks.push(ack);
                                    }
                                }
                                None => acks.push(ack),
                            }
                            state = State::ParseAcks(acks)
                        }
                    }
                };
                Ok(Response { acks })
            }
            Protocol::V2 => {
                // NOTE: We only read acknowledgements and scrub to the pack file, until we have use for the other features
                let mut line = String::new();
                let acks = loop {
                    let mut acks = None::<Vec<Acknowledgement>>;
                    line.clear();
                    if reader.read_line(&mut line)? == 0 {
                        return Err(Error::Io(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Could not read message headline",
                        )));
                    };

                    match line.trim_end() {
                        "acknowledgments" => {
                            let acks = acks.get_or_insert_with(Vec::new);
                            line.clear();
                            // reader.reset_with(Some(client::MessageKind::Delimiter));
                            while reader.read_line(&mut line)? != 0 {
                                acks.push(Acknowledgement::from_line(&line)?);
                                line.clear();
                            }
                        }
                        "packfile" => {
                            // what follows is the packfile itself, which can be read with a sideband enabled reader
                            break acks.unwrap_or_default();
                        }
                        _ => return Err(Error::UnknownSectionHeader(line)),
                    }
                };
                Ok(Response { acks })
            }
        }
    }

    pub fn acknowledgements(&self) -> &[Acknowledgement] {
        &self.acks
    }
}
