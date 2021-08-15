use std::ops::{Deref, DerefMut};

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
use crate::client::{MessageKind, RequestWriter, WriteMode};
use crate::{
    client::{Error, Identity},
    Protocol,
};

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

    /// If the actually advertised server version is contained in the returned slice or empty, continue as normal,
    /// assume the server's protocol version is desired or acceptable.
    ///
    /// Otherwise, abort the fetch operation with an error to avoid continuing any interaction with the transport.
    ///
    /// In V1 this means a potentially large list of advertised refs won't be read, instead the connection is ignored
    /// leaving the server with a client who potentially unexpectedly terminated the connection.
    ///
    /// Note that `transport.close()` is not called explicitly.
    fn supported_protocol_versions(&self) -> &[Protocol] {
        &[]
    }

    /// Returns true if the transport provides persistent connections across multiple requests, or false otherwise.
    /// Not being persistent implies that certain information has to be resent on each 'turn'
    /// of the fetch negotiation or that the end of interaction (i.e. no further request will be made) has to be indicated
    /// to the server for most graceful termination of the connection.
    fn connection_persists_across_multiple_requests(&self) -> bool;
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

    fn supported_protocol_versions(&self) -> &[Protocol] {
        self.deref().supported_protocol_versions()
    }

    fn connection_persists_across_multiple_requests(&self) -> bool {
        self.deref().connection_persists_across_multiple_requests()
    }
}

impl<T: TransportWithoutIO + ?Sized> TransportWithoutIO for &mut T {
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

    fn supported_protocol_versions(&self) -> &[Protocol] {
        self.deref().supported_protocol_versions()
    }

    fn connection_persists_across_multiple_requests(&self) -> bool {
        self.deref().connection_persists_across_multiple_requests()
    }
}
