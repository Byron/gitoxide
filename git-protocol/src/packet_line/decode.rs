use crate::packet_line::{
    self, DELIMITER_LINE, ERR_PREFIX, FLUSH_LINE, MAX_DATA_LEN, MAX_LINE_LEN, RESPONSE_END_LINE, U16_HEX_BYTES,
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
        Line(data: BString, bytes_consumed: usize) {
            display("{}", data)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stream<'a> {
    Complete {
        line: packet_line::Borrowed<'a>,
        bytes_consumed: usize,
    },
    Incomplete {
        /// The amount of additional bytes needed for the parsing to complete
        bytes_needed: usize,
    },
}

pub fn streaming(data: &[u8]) -> Result<Stream, Error> {
    let data_len = data.len();
    if data_len < U16_HEX_BYTES {
        return Ok(Stream::Incomplete {
            bytes_needed: U16_HEX_BYTES - data_len,
        });
    }
    let hex_bytes = &data[..U16_HEX_BYTES];
    if hex_bytes == FLUSH_LINE {
        return Ok(Stream::Complete {
            line: packet_line::Borrowed::Flush,
            bytes_consumed: 4,
        });
    }
    if hex_bytes == DELIMITER_LINE {
        return Ok(Stream::Complete {
            line: packet_line::Borrowed::Delimiter,
            bytes_consumed: 4,
        });
    }
    if hex_bytes == RESPONSE_END_LINE {
        return Ok(Stream::Complete {
            line: packet_line::Borrowed::ResponseEnd,
            bytes_consumed: 4,
        });
    }

    let mut buf = [0u8; U16_HEX_BYTES / 2];
    hex::decode_to_slice(hex_bytes, &mut buf)?;
    let wanted_bytes = u16::from_be_bytes(buf) as usize;
    if wanted_bytes > MAX_LINE_LEN {
        return Err(Error::DataLengthLimitExceeded(wanted_bytes));
    }
    if data_len < wanted_bytes {
        return Ok(Stream::Incomplete {
            bytes_needed: wanted_bytes - data_len,
        });
    }

    if wanted_bytes == 4 {
        return Err(Error::DataIsEmpty);
    }

    let mut data = &data[U16_HEX_BYTES..wanted_bytes];
    if data[data.len() - 1] == b'\n' {
        data = &data[..data.len() - 1];
    }

    if data.len() >= ERR_PREFIX.len() && &data[..ERR_PREFIX.len()] == ERR_PREFIX {
        return Err(Error::Line(data[ERR_PREFIX.len()..].into(), wanted_bytes));
    }

    Ok(Stream::Complete {
        line: packet_line::Borrowed::Data(data),
        bytes_consumed: wanted_bytes,
    })
}
