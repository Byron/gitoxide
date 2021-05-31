/// Configure how the [`RequestWriter`][crate::client::RequestWriter] behaves when writing bytes.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum WriteMode {
    /// Each [write()][Write::write()] call writes the bytes verbatim as one or more packet lines.
    Binary,
    /// Each [write()][Write::write()] call assumes text in the input, assures a trailing newline and writes it as single packet line.
    OneLfTerminatedLinePerWriteCall,
}

impl Default for WriteMode {
    fn default() -> Self {
        WriteMode::OneLfTerminatedLinePerWriteCall
    }
}

/// The kind of packet line to write when transforming a [`RequestWriter`][crate::client::RequestWriter] into an
/// [`ExtendedBufRead`][crate::client::ExtendedBufRead].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageKind {
    /// A `flush` packet.
    Flush,
    /// A V2 delimiter.
    Delimiter,
    /// The end of a response.
    ResponseEnd,
    /// The given text.
    Text(&'static [u8]),
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
/// An identity for use when authenticating the transport layer.
pub enum Identity {
    /// An account based identity
    Account {
        /// The user's name
        username: String,
        /// The user's password
        password: String,
    },
}

mod error {
    use crate::client::capabilities;
    #[cfg(feature = "http-client-curl")]
    use crate::client::http;
    #[cfg(feature = "http-client-curl")]
    type HttpError = http::Error;
    #[cfg(not(feature = "http-client-curl"))]
    type HttpError = std::convert::Infallible;

    /// The error used in most methods of the [`client`][crate::client] module
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("An IO error occurred when talking to the server")]
        Io {
            #[from]
            err: std::io::Error,
        },
        #[error("Capabilities could not be parsed")]
        Capabilities {
            #[from]
            err: capabilities::Error,
        },
        #[error("A packet line could not be decoded")]
        LineDecode {
            #[from]
            err: git_packetline::decode::Error,
        },
        #[error("A {0} line was expected, but there was none")]
        ExpectedLine(&'static str),
        #[error("Expected a data line, but got a delimiter")]
        ExpectedDataLine,
        #[error("The transport layer does not support authentication")]
        AuthenticationUnsupported,
        #[error("The transport layer refuses to use a given identity: {0}")]
        AuthenticationRefused(&'static str),
        #[error(transparent)]
        Http(#[from] HttpError),
    }
}

pub use error::Error;
