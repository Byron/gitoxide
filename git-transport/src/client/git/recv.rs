use crate::client::Capabilities;
use std::io;

pub fn capabilties_and_possibly_refs<'a, T: io::Read>(
    rd: &'a mut git_packetline::Reader<T>,
) -> Result<(Capabilities, Option<Box<dyn io::BufRead + 'a>>), crate::client::Error> {
    rd.fail_on_err_lines(true);
    let capabilities = rd
        .peek_line()
        .ok_or(crate::client::Error::ExpectedLine("capabilities or version"))???;
    let (capabilities, delimiter_position) = Capabilities::from_bytes(
        capabilities
            .to_text()
            .ok_or(crate::client::Error::ExpectedDataLine)?
            .as_slice(),
    )?;
    rd.peek_buffer_replace_and_truncate(delimiter_position, b'\n');
    Ok((capabilities, Some(Box::new(rd.as_read()))))
}
