use crate::{
    packet_line::{
        DELIMITER_LINE, ERR_PREFIX, FLUSH_LINE, MAX_DATA_LEN, MAX_LINE_LEN, RESPONSE_END_LINE, U16_HEX_BYTES,
    },
    PacketLine,
};
use bstr::BString;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        HexDecode(err: hex::FromHexError) {
            display("Failed to decode the first four hex bytes indicating the line length")
            from()
            source(err)
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
    }
}

#[derive(Debug, Clone)]
pub enum Stream<'a> {
    Complete {
        line: PacketLine<'a>,
        bytes_consumed: usize,
    },
    Incomplete {
        /// The amount of additional bytes needed for the parsing to complete
        bytes_needed: usize,
    },
}

pub enum PacketLineOrWantedSize<'a> {
    Line(PacketLine<'a>),
    Wanted(u16),
}

pub fn hex_prefix(four_bytes: &[u8]) -> Result<PacketLineOrWantedSize, Error> {
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
    hex::decode_to_slice(four_bytes, &mut buf)?;
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

pub fn to_data_line(data: &[u8]) -> Result<PacketLine, Error> {
    if data.len() > MAX_LINE_LEN {
        return Err(Error::DataLengthLimitExceeded(data.len()));
    }

    if data.len() >= ERR_PREFIX.len() && &data[..ERR_PREFIX.len()] == ERR_PREFIX {
        return Err(Error::Line(data[ERR_PREFIX.len()..].into(), data.len()));
    }
    Ok(PacketLine::Data(data))
}

// TODO: verify this one is actually needed in the end
pub fn streaming(data: &[u8]) -> Result<Stream, Error> {
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
