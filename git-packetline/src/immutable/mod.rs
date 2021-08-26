use bstr::BStr;

use crate::{decode, Channel, PacketLineRef, ERR_PREFIX};

impl<'a> PacketLineRef<'a> {
    /// Return this instance as slice if it's [`Data`][PacketLine::Data].
    pub fn as_slice(&self) -> Option<&[u8]> {
        match self {
            PacketLineRef::Data(d) => Some(d),
            PacketLineRef::Flush | PacketLineRef::Delimiter | PacketLineRef::ResponseEnd => None,
        }
    }
    /// Return this instance's [`as_slice()`][PacketLine::as_slice()] as [`BStr`].
    pub fn as_bstr(&self) -> Option<&BStr> {
        self.as_slice().map(Into::into)
    }
    /// Interpret this instance's [`as_slice()`][PacketLine::as_slice()] as [`Error`].
    ///
    /// This works for any data received in an error [channel][crate::Channel].
    ///
    /// Note that this creates an unchecked error using the slice verbatim, which is useful to [serialize it][Error::write_to()].
    /// See [`check_error()`][PacketLine::check_error()] for a version that assures the error information is in the expected format.
    pub fn as_error(&self) -> Option<Error<'_>> {
        self.as_slice().map(Error)
    }
    /// Check this instance's [`as_slice()`][PacketLine::as_slice()] is a valid [`Error`] and return it.
    ///
    /// This works for any data received in an error [channel][crate::Channel].
    pub fn check_error(&self) -> Option<Error<'_>> {
        self.as_slice().and_then(|data| {
            if data.len() >= ERR_PREFIX.len() && &data[..ERR_PREFIX.len()] == ERR_PREFIX {
                Some(Error(&data[ERR_PREFIX.len()..]))
            } else {
                None
            }
        })
    }
    /// Return this instance as text, with the trailing newline truncated if present.
    pub fn as_text(&self) -> Option<Text<'_>> {
        self.as_slice().map(Into::into)
    }

    /// Interpret the data in this [`slice`][PacketLine::as_slice()] as [`Band`] according to the given `kind` of channel.
    ///
    /// Note that this is only relevant in a side-band channel.
    /// See [`decode_band()`][PacketLine::decode_band()] in case `kind` is unknown.
    pub fn as_band(&self, kind: Channel) -> Option<Band<'_>> {
        self.as_slice().map(|d| match kind {
            Channel::Data => Band::Data(d),
            Channel::Progress => Band::Progress(d),
            Channel::Error => Band::Error(d),
        })
    }

    /// Decode the band of this [`slice`][PacketLine::as_slice()], or panic if it is not actually a side-band line.
    pub fn decode_band(&self) -> Result<Band<'_>, decode::band::Error> {
        let d = self.as_slice().ok_or(decode::band::Error::NonDataLine)?;
        Ok(match d[0] {
            1 => Band::Data(&d[1..]),
            2 => Band::Progress(&d[1..]),
            3 => Band::Error(&d[1..]),
            band => return Err(decode::band::Error::InvalidSideBand(band)),
        })
    }
}

/// A packet line representing an Error in a side-band channel.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Error<'a>(pub &'a [u8]);

/// A packet line representing text, which may include a trailing newline.
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
    /// Return this instance's data.
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }
    /// Return this instance's data as [`BStr`].
    pub fn as_bstr(&self) -> &BStr {
        self.0.into()
    }
}

/// A band in a side-band channel.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Band<'a> {
    /// A band carrying data.
    Data(&'a [u8]),
    /// A band carrying user readable progress information.
    Progress(&'a [u8]),
    /// A band carrying user readable errors.
    Error(&'a [u8]),
}

#[cfg(all(not(feature = "blocking-io"), feature = "async-io"))]
mod async_io;
#[cfg(feature = "blocking-io")]
mod blocking_io;
