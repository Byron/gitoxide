use crate::{
    PacketLine, {DELIMITER_LINE, FLUSH_LINE, MAX_DATA_LEN, MAX_LINE_LEN, RESPONSE_END_LINE, U16_HEX_BYTES},
};
use bstr::BString;
use quick_error::quick_error;

quick_error! {
    /// The error used in the [`decode`][crate::decode] module
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        HexDecode(err: String) {
            display("Failed to decode the first four hex bytes indicating the line length: {}", err)
        }
        DataLengthLimitExceeded(length_in_bytes: usize) {
            display("The data received claims to be larger than than the maximum allowed size: got {}, exceeds {}", length_in_bytes, MAX_DATA_LEN)
        }
        DataIsEmpty {
            display("Received an invalid empty line")
        }
        InvalidLineLength {
            display("Received an invalid line of length 3")
        }
        Line(data: BString, bytes_consumed: usize) {
            display("{}", data)
        }
        NotEnoughData(bytes_needed: usize) {
            display("Needing {} additional bytes to decode the line successfully", bytes_needed)
        }
    }
}

/// A utility return type to support incremental parsing of packet lines.
#[derive(Debug, Clone)]
pub enum Stream<'a> {
    /// Indicate a single packet line was parsed completely
    Complete {
        /// The parsed packet line
        line: PacketLine<'a>,
        /// The amount of bytes consumed from input
        bytes_consumed: usize,
    },
    /// A packet line could not yet be parsed to to missing bytes
    Incomplete {
        /// The amount of additional bytes needed for the parsing to complete
        bytes_needed: usize,
    },
}

/// The result of [`hex_prefix()`] indicating either a special packet line or the amount of wanted bytes
pub enum PacketLineOrWantedSize<'a> {
    /// The special kind of packet line decoded from the hex prefix. It never contains actual data.
    Line(PacketLine<'a>),
    /// The amount of bytes indicated by the hex prefix of the packet line.
    Wanted(u16),
}

/// Decode the `four_bytes` packet line prefix provided in hexadecimal form and check it for validity.
pub fn hex_prefix(four_bytes: &[u8]) -> Result<PacketLineOrWantedSize<'_>, Error> {
    debug_assert_eq!(four_bytes.len(), 4, "need four hex bytes");
    for (line_bytes, line_type) in &[
        (FLUSH_LINE, PacketLine::Flush),
        (DELIMITER_LINE, PacketLine::Delimiter),
        (RESPONSE_END_LINE, PacketLine::ResponseEnd),
    ] {
        if four_bytes == *line_bytes {
            return Ok(PacketLineOrWantedSize::Line(*line_type));
        }
    }

    let mut buf = [0u8; U16_HEX_BYTES / 2];
    hex::decode_to_slice(four_bytes, &mut buf).map_err(|err| Error::HexDecode(err.to_string()))?;
    let wanted_bytes = u16::from_be_bytes(buf);

    if wanted_bytes == 3 {
        return Err(Error::InvalidLineLength);
    }
    if wanted_bytes == 4 {
        return Err(Error::DataIsEmpty);
    }
    debug_assert!(
        wanted_bytes as usize > U16_HEX_BYTES,
        "by now there should be more wanted bytes than prefix bytes"
    );
    Ok(PacketLineOrWantedSize::Wanted(wanted_bytes - U16_HEX_BYTES as u16))
}

/// Obtain a `PacketLine` from `data` after assuring `data` is small enough to fit.
pub fn to_data_line(data: &[u8]) -> Result<PacketLine<'_>, Error> {
    if data.len() > MAX_LINE_LEN {
        return Err(Error::DataLengthLimitExceeded(data.len()));
    }

    Ok(PacketLine::Data(data))
}

/// Decode `data` as packet line while reporting whether the data is complete or not using a [`Stream`].
pub fn streaming(data: &[u8]) -> Result<Stream<'_>, Error> {
    let data_len = data.len();
    if data_len < U16_HEX_BYTES {
        return Ok(Stream::Incomplete {
            bytes_needed: U16_HEX_BYTES - data_len,
        });
    }
    let wanted_bytes = match hex_prefix(&data[..U16_HEX_BYTES])? {
        PacketLineOrWantedSize::Wanted(s) => s as usize,
        PacketLineOrWantedSize::Line(line) => {
            return Ok(Stream::Complete {
                line,
                bytes_consumed: 4,
            })
        }
    } + U16_HEX_BYTES;
    if wanted_bytes > MAX_LINE_LEN {
        return Err(Error::DataLengthLimitExceeded(wanted_bytes));
    }
    if data_len < wanted_bytes {
        return Ok(Stream::Incomplete {
            bytes_needed: wanted_bytes - data_len,
        });
    }

    Ok(Stream::Complete {
        line: to_data_line(&data[U16_HEX_BYTES..wanted_bytes])?,
        bytes_consumed: wanted_bytes,
    })
}

/// Decode an entire packet line from data or fail.
///
/// Note that failure also happens if there is not enough data to parse a complete packet line, as opposed to [`streaming()`] decoding
/// succeeds in that case, stating how much more bytes are required.
pub fn all_at_once(data: &[u8]) -> Result<PacketLine<'_>, Error> {
    match streaming(data)? {
        Stream::Complete { line, .. } => Ok(line),
        Stream::Incomplete { bytes_needed } => Err(Error::NotEnoughData(bytes_needed)),
    }
}
