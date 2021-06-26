#[cfg(any(feature = "blocking-client", feature = "async-client"))]
use crate::client::{MessageKind, RequestWriter, WriteMode};
use crate::{
    client::{Error, Identity},
    Protocol,
};
use std::ops::{Deref, DerefMut};

/// Describes what to do right after the handshake when the servers supported protocol was advertised to the transport.
pub enum ProtocolDecision {
    /// Continue as normal, the server's protocol version is desired or acceptable.
    Continue,
    /// Close the connection immediately without reading any more.
    ///
    /// In V1 this means a potentially large list of advertised refs won't be read, instead the connection is closed
    /// leaving the server with a client who unexpectedly terminated the connection.
    CloseConnectionImmediately,
}

/// This trait represents all transport related functions that don't require any input/output to be done which helps
/// implementation to share more code across blocking and async programs.
pub trait TransportWithoutIO {
    /// If the handshake or subsequent reads failed with [std::io::ErrorKind::PermissionDenied], use this method to
    /// inform the transport layer about the identity to use for subsequent calls.
    /// If authentication continues to fail even with an identity set, consider communicating this to the provider
    /// of the identity in order to mark it as invalid. Otherwise the user might have difficulty updating obsolete
    /// credentials.
    /// Please note that most transport layers are unauthenticated and thus return [an error][Error::AuthenticationUnsupported] here.
    fn set_identity(&mut self, _identity: Identity) -> Result<(), Error> {
        Err(Error::AuthenticationUnsupported)
    }
    /// Get a writer for sending data and obtaining the response. It can be configured in various ways
    /// to support the task at hand.
    /// `write_mode` determines how calls to the `write(…)` method are interpreted, and `on_into_read` determines
    /// which message to write when the writer is turned into the response reader using [`into_read()`][RequestWriter::into_read()].
    #[cfg(any(feature = "blocking-client", feature = "async-client"))]
    fn request(&mut self, write_mode: WriteMode, on_into_read: MessageKind) -> Result<RequestWriter<'_>, Error>;

    /// Returns the canonical URL pointing to the destination of this transport.
    /// Please note that local paths may not be represented correctly, as they will go through a potentially lossy
    /// unicode conversion.
    fn to_url(&self) -> String;

    /// Verifies the actual protocol version supported by the server is acceptable and returns a decision on what to do next.
    ///
    /// Please note that the actual version might differ after the handshake was conducted in case the server
    /// did not support it and downgraded it instead.
    fn supports_advertised_version(&self, _actual_version: Protocol) -> ProtocolDecision {
        ProtocolDecision::Continue
    }

    /// Returns true if the transport is inherently stateful, or false otherwise.
    /// Not being stateful implies that certain information has to be resent on each 'turn'
    /// of the fetch negotiation when using protocol version 1.
    ///
    /// # Implementation Details
    ///
    /// This answer should not be based on the [Protocol] itself, which might enforce stateless
    /// interactions despite the connections staying intact which might imply statefulness.
    ///
    /// This means that HTTP transports generally operate in a stateless fashion independent of the
    /// protocol version.
    fn is_stateful(&self) -> bool;
}

// Would be nice if the box implementation could auto-forward to all implemented traits.
impl<T: TransportWithoutIO + ?Sized> TransportWithoutIO for Box<T> {
    fn set_identity(&mut self, identity: Identity) -> Result<(), Error> {
        self.deref_mut().set_identity(identity)
    }

    #[cfg(any(feature = "blocking-client", feature = "async-client"))]
    fn request(&mut self, write_mode: WriteMode, on_into_read: MessageKind) -> Result<RequestWriter<'_>, Error> {
        self.deref_mut().request(write_mode, on_into_read)
    }

    fn to_url(&self) -> String {
        self.deref().to_url()
    }

    fn supports_advertised_version(&self, actual_version: Protocol) -> ProtocolDecision {
        self.deref().supports_advertised_version(actual_version)
    }

    fn is_stateful(&self) -> bool {
        self.deref().is_stateful()
    }
}
