use crate::fetch::{
    response,
    response::{Acknowledgement, ShallowUpdate, WantedRef},
    Response,
};
use futures_lite::AsyncBufReadExt;
use git_transport::{client, Protocol};
use std::io;

async fn parse_v2_section<T>(
    line: &mut String,
    reader: &mut (impl client::ExtendedBufRead + Unpin),
    res: &mut Vec<T>,
    parse: impl Fn(&str) -> Result<T, response::Error>,
) -> Result<bool, response::Error> {
    line.clear();
    while reader.read_line(line).await? != 0 {
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

impl Response {
    /// Parse a response of the given `version` of the protocol from `reader`.
    pub async fn from_line_reader(
        version: Protocol,
        reader: &mut (impl client::ExtendedBufRead + Unpin),
    ) -> Result<Response, response::Error> {
        match version {
            Protocol::V1 => {
                let mut line = String::new();
                let mut acks = Vec::<Acknowledgement>::new();
                let mut shallows = Vec::<ShallowUpdate>::new();
                let has_pack = 'lines: loop {
                    line.clear();
                    let peeked_line = match reader.peek_data_line().await {
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
                            debug_assert_eq!(
                                reader.stopped_at(),
                                Some(client::MessageKind::Flush),
                                "If this isn't a flush packet, we don't know what's going on"
                            );
                            reader.read_line(&mut line).await?;
                            reader.reset(Protocol::V1);
                            match reader.peek_data_line().await {
                                Some(Ok(Ok(line))) => String::from_utf8_lossy(line),
                                Some(Err(err)) => return Err(err.into()),
                                Some(Ok(Err(err))) => return Err(err.into()),
                                None => break 'lines false, // EOF
                            }
                        }
                    };

                    if Response::parse_v1_ack_or_shallow_or_assume_pack(&mut acks, &mut shallows, &peeked_line) {
                        break 'lines true;
                    }
                    assert_ne!(reader.read_line(&mut line).await?, 0, "consuming a peeked line works");
                };
                Ok(Response {
                    acks,
                    shallows,
                    wanted_refs: vec![],
                    has_pack,
                })
            }
            Protocol::V2 => {
                // NOTE: We only read acknowledgements and scrub to the pack file, until we have use for the other features
                let mut line = String::new();
                reader.reset(Protocol::V2);
                let mut acks = Vec::<Acknowledgement>::new();
                let mut shallows = Vec::<ShallowUpdate>::new();
                let mut wanted_refs = Vec::<WantedRef>::new();
                let has_pack = 'section: loop {
                    line.clear();
                    if reader.read_line(&mut line).await? == 0 {
                        return Err(response::Error::Io(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Could not read message headline",
                        )));
                    };

                    match line.trim_end() {
                        "acknowledgments" => {
                            if parse_v2_section(&mut line, reader, &mut acks, Acknowledgement::from_line).await? {
                                break 'section false;
                            }
                        }
                        "shallow-info" => {
                            if parse_v2_section(&mut line, reader, &mut shallows, ShallowUpdate::from_line).await? {
                                break 'section false;
                            }
                        }
                        "wanted-refs" => {
                            if parse_v2_section(&mut line, reader, &mut wanted_refs, WantedRef::from_line).await? {
                                break 'section false;
                            }
                        }
                        "packfile" => {
                            // what follows is the packfile itself, which can be read with a sideband enabled reader
                            break 'section true;
                        }
                        _ => return Err(response::Error::UnknownSectionHeader(line)),
                    }
                };
                Ok(Response {
                    acks,
                    shallows,
                    wanted_refs,
                    has_pack,
                })
            }
        }
    }
}
