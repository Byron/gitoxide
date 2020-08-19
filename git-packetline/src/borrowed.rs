use crate::{encode, Channel, ERR_PREFIX};
use bstr::BStr;
use std::io;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Borrowed<'a> {
    Data(&'a [u8]),
    Flush,
    Delimiter,
    ResponseEnd,
}

impl<'a> Borrowed<'a> {
    pub fn to_write(&self, out: impl io::Write) -> Result<usize, encode::Error> {
        match self {
            Borrowed::Data(d) => encode::data_to_write(d, out),
            Borrowed::Flush => encode::flush_to_write(out).map_err(Into::into),
            Borrowed::Delimiter => encode::delim_to_write(out).map_err(Into::into),
            Borrowed::ResponseEnd => encode::response_end_to_write(out).map_err(Into::into),
        }
    }

    pub fn as_slice(&self) -> Option<&[u8]> {
        match self {
            Borrowed::Data(d) => Some(d),
            Borrowed::Flush | Borrowed::Delimiter | Borrowed::ResponseEnd => None,
        }
    }
    pub fn as_bstr(&self) -> Option<&BStr> {
        self.as_slice().map(Into::into)
    }
    pub fn to_error(&self) -> Option<Error> {
        self.as_slice().map(Error)
    }
    pub fn check_error(&self) -> Option<Error> {
        self.as_slice().and_then(|data| {
            if data.len() >= ERR_PREFIX.len() && &data[..ERR_PREFIX.len()] == ERR_PREFIX {
                Some(Error(&data[ERR_PREFIX.len()..]))
            } else {
                None
            }
        })
    }
    pub fn to_text(&self) -> Option<Text> {
        self.as_slice().map(Into::into)
    }
    pub fn to_band(&self, kind: Channel) -> Option<Band> {
        self.as_slice().map(|d| match kind {
            Channel::Data => Band::Data(d),
            Channel::Progress => Band::Progress(d),
            Channel::Error => Band::Error(d),
        })
    }
    /// Decode the band of the line, or panic if it is not actually a side-band line
    pub fn decode_band(&self) -> Result<Band, DecodeBandError> {
        let d = self.as_slice().ok_or(DecodeBandError::NonDataLine)?;
        Ok(match d[0] {
            1 => Band::Data(&d[1..]),
            2 => Band::Progress(&d[1..]),
            3 => Band::Error(&d[1..]),
            band => return Err(DecodeBandError::InvalidSideBand(band)),
        })
    }
}

use quick_error::quick_error;
quick_error! {
    #[derive(Debug)]
    pub enum DecodeBandError {
        InvalidSideBand(band: u8) {
            display("attempt to decode a non-side channel line or input was malformed: {}", band)
        }
        NonDataLine {
            display("attempt to decode a non-data line into a side-channel band")
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Error<'a>(pub &'a [u8]);

impl<'a> Error<'a> {
    pub fn to_write(&self, out: impl io::Write) -> Result<usize, encode::Error> {
        encode::error_to_write(self.0, out)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Text<'a>(pub &'a [u8]);

impl<'a> From<&'a [u8]> for Text<'a> {
    fn from(d: &'a [u8]) -> Self {
        let d = if d[d.len() - 1] == b'\n' { &d[..d.len() - 1] } else { d };
        Text(d)
    }
}

impl<'a> Text<'a> {
    pub fn to_write(&self, out: impl io::Write) -> Result<usize, encode::Error> {
        encode::text_to_write(self.0, out)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Band<'a> {
    Data(&'a [u8]),
    Progress(&'a [u8]),
    Error(&'a [u8]),
}

impl<'a> Band<'a> {
    pub fn to_write(&self, out: impl io::Write) -> Result<usize, encode::Error> {
        match self {
            Band::Data(d) => encode::band_to_write(Channel::Data, d, out),
            Band::Progress(d) => encode::band_to_write(Channel::Progress, d, out),
            Band::Error(d) => encode::band_to_write(Channel::Error, d, out),
        }
    }
}
