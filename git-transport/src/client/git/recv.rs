use crate::{client, client::Capabilities, Protocol};
use std::io;

pub fn capabilties_and_possibly_refs<'a, T: io::Read>(
    rd: &'a mut git_packetline::Provider<T>,
    version: Protocol,
) -> Result<(Capabilities, Option<Box<dyn io::BufRead + 'a>>), client::Error> {
    rd.fail_on_err_lines(true);
    match version {
        Protocol::V1 => {
            let capabilities = rd
                .peek_line()
                .ok_or(client::Error::ExpectedLine("capabilities or version"))???;
            let (capabilities, delimiter_position) = Capabilities::from_bytes(
                capabilities
                    .to_text()
                    .ok_or(client::Error::ExpectedLine("text"))?
                    .as_slice(),
            )?;
            rd.peek_buffer_replace_and_truncate(delimiter_position, b'\n');
            Ok((capabilities, Some(Box::new(rd.as_read()))))
        }
        Protocol::V2 => Ok((Capabilities::from_lines(rd.as_read())?, None)),
    }
}
