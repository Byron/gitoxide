use crate::packet_line::Borrowed::Flush;
use crate::packet_line::{self, FLUSH_LINE, MAX_DATA_LEN, MAX_LINE_LEN, U16_HEX_BYTES};
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
        Line(data: BString) {
            display("{}", data)
        }
    }
}

pub enum ParseResult<'a> {
    Complete {
        result: Result<packet_line::Borrowed<'a>, Error>,
        bytes_consumed: usize,
    },
    Incomplete {
        bytes_needed: usize,
    },
}

pub fn streaming(data: &[u8]) -> ParseResult {
    let data_len = data.len();
    if data_len < U16_HEX_BYTES {
        return ParseResult::Incomplete {
            bytes_needed: U16_HEX_BYTES - data_len,
        };
    }
    let hex_bytes = &data[..U16_HEX_BYTES];
    if hex_bytes == FLUSH_LINE {
        return ParseResult::Complete {
            result: Ok(packet_line::Borrowed::Flush),
            bytes_consumed: 4,
        };
    }

    let mut buf = [0u8; U16_HEX_BYTES / 2];
    hex::decode_to_slice(hex_bytes, &mut buf)?;
    let wanted_bytes = u16::from_be_bytes(buf) as usize;
    if data_len < wanted_bytes {
        return ParseResult::Incomplete {
            bytes_needed: wanted_bytes,
        };
    }
    if wanted_bytes > MAX_LINE_LEN {
        return Err(Error::DataLengthLimitExceeded(wanted_bytes));
    }
    ParseResult::Complete { result }
}
