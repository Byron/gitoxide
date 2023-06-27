use std::collections::HashSet;

/// A handle to a client that allows communicating to a long-running process.
pub struct Client {
    /// The child process we are communicating with.
    child: std::process::Child,
    /// The names of the obtained capabilities after the handshake.
    capabilities: HashSet<String>,
    /// The negotiated version of the protocol.
    version: usize,
    /// A way to send packet-line encoded information to the process.
    input: gix_packetline::Writer<std::process::ChildStdin>,
    /// A way to read information sent to us by the process.
    out: gix_packetline::StreamingPeekableIter<std::process::ChildStdout>,
}

/// A handle to facilitate typical server interactions that include the handshake and command-invocations.
pub struct Server {
    /// The names of the capabilities we can expect the client to use.
    capabilities: HashSet<String>,
    /// The negotiated version of the protocol, it's the highest supported one.
    version: usize,
    /// A way to receive information from the client.
    input: gix_packetline::StreamingPeekableIter<std::io::StdinLock<'static>>,
    /// A way to send information to the client.
    out: gix_packetline::Writer<std::io::StdoutLock<'static>>,
}

/// The return status of an [invoked command][Client::invoke()].
#[derive(Debug, Clone)]
pub enum Status {
    /// No new status was set, and instead we are to assume the previous status is still in effect.
    Previous,
    /// Assume the given named status.
    Named(String),
}

/// Initialization
impl Status {
    /// Create a new instance that represents a successful operation.
    pub fn success() -> Self {
        Status::Named("success".into())
    }

    /// Create a new instance that represents an error with the given `message`.
    pub fn error(message: impl Into<String>) -> Self {
        Status::Named(message.into())
    }
}

/// Access
impl Status {
    /// Note that this is assumed true even if no new status is set, hence we assume that upon error, the caller will not continue
    /// interacting with the process.
    pub fn is_success(&self) -> bool {
        match self {
            Status::Previous => true,
            Status::Named(n) => n == "success",
        }
    }

    /// Return true if the status is explicitly set to indicated delayed output processing
    pub fn is_delayed(&self) -> bool {
        match self {
            Status::Previous => false,
            Status::Named(n) => n == "delayed",
        }
    }

    /// Return the status message if present.
    pub fn message(&self) -> Option<&str> {
        match self {
            Status::Previous => None,
            Status::Named(msg) => msg.as_str().into(),
        }
    }
}

///
pub mod client;

///
pub mod server;

type PacketlineReader<'a, T = std::process::ChildStdout> =
    gix_packetline::read::WithSidebands<'a, T, fn(bool, &[u8]) -> gix_packetline::read::ProgressAction>;
