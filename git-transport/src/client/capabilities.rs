use std::io;

use bstr::{BStr, BString, ByteSlice};
use quick_error::quick_error;

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
use crate::client;
use crate::Protocol;

quick_error! {
    /// The error used in [`Capabilities::from_bytes()`] and [`Capabilities::from_lines()`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        MissingDelimitingNullByte {
            display("Capabilities were missing entirely as there was no 0 byte")
        }
        NoCapabilities {
            display("there was not a single capability behind the delimiter")
        }
        MissingVersionLine {
            display("a version line was expected, but none was retrieved")
        }
        MalformattedVersionLine(actual: String) {
            display("expected 'version X', got '{}'", actual)
        }
        UnsupportedVersion(wanted: Protocol, got: String) {
            display("Got unsupported version '{}', expected '{}'", got, *wanted as usize)
        }
        Io(err: io::Error) {
            display("An IO error occurred while reading V2 lines")
            from()
            source(err)
        }
    }
}

/// A structure to represent multiple [capabilities][Capability] or features supported by the server.
#[derive(Debug, Clone)]
pub struct Capabilities {
    data: BString,
    value_sep: u8,
}

/// The name of a single capability.
pub struct Capability<'a>(&'a BStr);

impl<'a> Capability<'a> {
    /// Returns the name of the capability.
    ///
    /// Most capabilities only consist of a name, making them appear like a feature toggle.
    pub fn name(&self) -> &BStr {
        self.0
            .splitn(2, |b| *b == b'=')
            .next()
            .expect("there is always a single item")
            .as_bstr()
    }
    /// Returns the value associated with the capability.
    ///
    /// Note that the caller must know whether a single or multiple values are expected, in which
    /// case [`values()`][Capability::values()] should be called.
    pub fn value(&self) -> Option<&BStr> {
        self.0.splitn(2, |b| *b == b'=').nth(1).map(|s| s.as_bstr())
    }
    /// Returns the values of a capability if its [`value()`][Capability::value()] is space separated.
    pub fn values(&self) -> Option<impl Iterator<Item = &BStr>> {
        self.value().map(|v| v.split(|b| *b == b' ').map(|s| s.as_bstr()))
    }
    /// Returns true if its space-separated [`value()`][Capability::value()] contains the given `want`ed capability.
    pub fn supports(&self, want: impl Into<&'a BStr>) -> Option<bool> {
        let want = want.into();
        self.values().map(|mut iter| iter.any(|v| v == want))
    }
}

impl Capabilities {
    /// Parse capabilities from the given `bytes`.
    ///
    /// Useful in case they are encoded within a `ref` behind a null byte.
    pub fn from_bytes(bytes: &[u8]) -> Result<(Capabilities, usize), Error> {
        let delimiter_pos = bytes.find_byte(0).ok_or(Error::MissingDelimitingNullByte)?;
        if delimiter_pos + 1 == bytes.len() {
            return Err(Error::NoCapabilities);
        }
        let capabilities = &bytes[delimiter_pos + 1..];
        Ok((
            Capabilities {
                data: capabilities.as_bstr().to_owned(),
                value_sep: b' ',
            },
            delimiter_pos,
        ))
    }

    /// Parse capabilities from the given a `first_line` and the rest of the lines as single newline
    /// separated string via `remaining_lines`.
    ///
    /// Useful for parsing capabilities from a data sent from a server, and to avoid having to deal with
    /// blocking and async traits for as long as possible. There is no value in parsing a few bytes
    /// in a non-blocking fashion.
    pub fn from_lines(
        first_line: Option<impl Into<std::io::Result<String>>>,
        remaining_lines: impl Into<String>,
    ) -> Result<Capabilities, Error> {
        let version_line = first_line.map(Into::into).ok_or(Error::MissingVersionLine)??;
        let (name, value) = version_line.split_at(
            version_line
                .find(' ')
                .ok_or_else(|| Error::MalformattedVersionLine(version_line.clone()))?,
        );
        if name != "version" {
            return Err(Error::MalformattedVersionLine(version_line));
        }
        if value != " 2" {
            return Err(Error::UnsupportedVersion(Protocol::V2, value.to_owned()));
        }
        Ok(Capabilities {
            value_sep: b'\n',
            data: remaining_lines.into().into(),
        })
    }

    /// Returns true of the given `feature` is mentioned in this list of capabilities.
    pub fn contains(&self, feature: &str) -> bool {
        self.capability(feature).is_some()
    }

    /// Returns the capability with `name`.
    pub fn capability(&self, name: &str) -> Option<Capability<'_>> {
        self.iter().find(|c| c.name() == name.as_bytes().as_bstr())
    }

    /// Returns an iterator over all capabilities.
    pub fn iter(&self) -> impl Iterator<Item = Capability<'_>> {
        self.data
            .split(move |b| *b == self.value_sep)
            .map(|c| Capability(c.as_bstr()))
    }
}

/// internal use
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
impl Capabilities {
    fn extract_protocol<'a>(
        capabilities_or_version: &'a git_packetline::PacketLine<'_>,
    ) -> Result<(git_packetline::immutable::Text<'a>, Protocol), client::Error> {
        let first_line = capabilities_or_version
            .as_text()
            .ok_or(client::Error::ExpectedLine("text"))?;

        let line = first_line.as_bstr();
        let version = if line.starts_with_str("version ") {
            if line.len() != "version X".len() {
                return Err(client::Error::UnsupportedProtocolVersion(line.as_bstr().into()));
            }
            match line {
                line if line.ends_with_str("1") => Protocol::V1,
                line if line.ends_with_str("2") => Protocol::V2,
                _ => return Err(client::Error::UnsupportedProtocolVersion(line.as_bstr().into())),
            }
        } else {
            Protocol::V1
        };
        Ok((first_line, version))
    }
}

#[cfg(feature = "blocking-client")]
///
pub mod recv {
    use std::{io, io::BufRead};

    use crate::{client, client::Capabilities, Protocol};

    /// Success outcome of [`Capabilities::from_lines_with_version_detection`].
    pub struct Outcome<'a> {
        /// The [`Capabilities`] the remote advertised.
        pub capabilities: Capabilities,
        /// The remote refs as a [`io::BufRead`].
        ///
        /// This is `Some` only when protocol v1 is used. The [`io::BufRead`] must be exhausted by
        /// the caller.
        pub refs: Option<Box<dyn io::BufRead + 'a>>,
        /// The [`Protocol`] the remote advertised.
        pub protocol: Protocol,
    }

    impl Capabilities {
        /// Read the capabilities and version advertisement from the given packetline reader.
        ///
        /// If [`Protocol::V1`] was requested, or the remote decided to downgrade, the remote refs
        /// advertisement will also be included in the [`Outcome`].
        pub fn from_lines_with_version_detection<T: io::Read>(
            rd: &mut git_packetline::StreamingPeekableIter<T>,
        ) -> Result<Outcome<'_>, client::Error> {
            // NOTE that this is vitally important - it is turned on and stays on for all following requests so
            // we automatically abort if the server sends an ERR line anywhere.
            // We are sure this can't clash with binary data when sent due to the way the PACK
            // format looks like, thus there is no binary blob that could ever look like an ERR line by accident.
            rd.fail_on_err_lines(true);

            let capabilities_or_version = rd
                .peek_line()
                .ok_or(client::Error::ExpectedLine("capabilities or version"))???;

            let (first_line, version) = Capabilities::extract_protocol(&capabilities_or_version)?;
            match version {
                Protocol::V1 => {
                    let (capabilities, delimiter_position) = Capabilities::from_bytes(first_line.0)?;
                    rd.peek_buffer_replace_and_truncate(delimiter_position, b'\n');
                    Ok(Outcome {
                        capabilities,
                        refs: Some(Box::new(rd.as_read())),
                        protocol: Protocol::V1,
                    })
                }
                Protocol::V2 => Ok(Outcome {
                    capabilities: {
                        let rd = rd.as_read();
                        let mut lines = rd.lines();
                        Capabilities::from_lines(lines.next(), lines.collect::<Result<Vec<_>, _>>()?.join("\n"))?
                    },
                    refs: None,
                    protocol: Protocol::V2,
                }),
            }
        }
    }
}

#[cfg(feature = "async-client")]
#[allow(missing_docs)]
///
pub mod recv {
    use futures_io::{AsyncBufRead, AsyncRead};
    use futures_lite::{AsyncBufReadExt, StreamExt};

    use crate::{client, client::Capabilities, Protocol};

    /// Success outcome of [`Capabilities::from_lines_with_version_detection`].
    pub struct Outcome<'a> {
        /// The [`Capabilities`] the remote advertised.
        pub capabilities: Capabilities,
        /// The remote refs as an [`AsyncBufRead`].
        ///
        /// This is `Some` only when protocol v1 is used. The [`AsyncBufRead`] must be exhausted by
        /// the caller.
        pub refs: Option<Box<dyn AsyncBufRead + Unpin + 'a>>,
        /// The [`Protocol`] the remote advertised.
        pub protocol: Protocol,
    }

    impl Capabilities {
        /// Read the capabilities and version advertisement from the given packetline reader.
        ///
        /// If [`Protocol::V1`] was requested, or the remote decided to downgrade, the remote refs
        /// advertisement will also be included in the [`Outcome`].
        pub async fn from_lines_with_version_detection<T: AsyncRead + Unpin>(
            rd: &mut git_packetline::StreamingPeekableIter<T>,
        ) -> Result<Outcome<'_>, client::Error> {
            // NOTE that this is vitally important - it is turned on and stays on for all following requests so
            // we automatically abort if the server sends an ERR line anywhere.
            // We are sure this can't clash with binary data when sent due to the way the PACK
            // format looks like, thus there is no binary blob that could ever look like an ERR line by accident.
            rd.fail_on_err_lines(true);

            let capabilities_or_version = rd
                .peek_line()
                .await
                .ok_or(client::Error::ExpectedLine("capabilities or version"))???;

            let (first_line, version) = Capabilities::extract_protocol(&capabilities_or_version)?;
            match version {
                Protocol::V1 => {
                    let (capabilities, delimiter_position) = Capabilities::from_bytes(first_line.0)?;
                    rd.peek_buffer_replace_and_truncate(delimiter_position, b'\n');
                    Ok(Outcome {
                        capabilities,
                        refs: Some(Box::new(rd.as_read())),
                        protocol: Protocol::V1,
                    })
                }
                Protocol::V2 => Ok(Outcome {
                    capabilities: {
                        let rd = rd.as_read();
                        let mut lines_with_err = rd.lines();
                        let mut lines = Vec::new();
                        while let Some(line) = lines_with_err.next().await {
                            lines.push(line?);
                        }
                        let mut lines = lines.into_iter();
                        Capabilities::from_lines(lines.next().map(Ok), lines.collect::<Vec<_>>().join("\n"))?
                    },
                    refs: None,
                    protocol: Protocol::V2,
                }),
            }
        }
    }
}
