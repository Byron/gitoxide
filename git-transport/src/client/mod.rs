#[cfg(all(not(feature = "blocking-client"), feature = "async-client"))]
mod async_io;
#[cfg(all(not(feature = "blocking-client"), feature = "async-client"))]
pub use async_io::{ExtendedBufRead, HandleProgress, RequestWriter, SetServiceResponse, Transport, TransportV2Ext};

mod traits {
    use crate::{
        client::{Error, Identity, MessageKind, RequestWriter, WriteMode},
        Protocol,
    };

    /// This trait represents all transport related functions that don't require any input/output to be done which helps
    /// implementation to share more code across blocking and async programs.
    pub trait TransportWithoutIO {
        /// If the handshake or subsequent reads failed with [io::ErrorKind::PermissionDenied], use this method to
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
        fn request(&mut self, write_mode: WriteMode, on_into_read: MessageKind) -> Result<RequestWriter<'_>, Error>;

        /// Returns the canonical URL pointing to the destination of this transport.
        /// Please note that local paths may not be represented correctly, as they will go through a potentially lossy
        /// unicode conversion.
        fn to_url(&self) -> String;

        /// Returns the protocol version that was initially desired upon connection
        /// Please note that the actual protocol might differ after the handshake was conducted in case the server
        /// did not support it.
        fn desired_protocol_version(&self) -> Protocol;

        /// Returns true if the transport is inherently stateful, or false otherwise.
        /// Not being stateful implies that certain information has to be resent on each 'turn'
        /// of the fetch negotiation.
        ///
        /// # Implementation Details
        ///
        /// This answer should not be based on the [Protocol] itself, which might enforce stateless
        /// interactions despite the connections staying intact which might imply statefulness.
        fn is_stateful(&self) -> bool;
    }

    mod box_impl {
        use crate::{
            client::{self, Error, Identity, MessageKind, RequestWriter, WriteMode},
            Protocol,
        };
        use std::ops::{Deref, DerefMut};

        // Would be nice if the box implementation could auto-forward to all implemented traits.
        impl<T: client::TransportWithoutIO + ?Sized> client::TransportWithoutIO for Box<T> {
            fn set_identity(&mut self, identity: Identity) -> Result<(), Error> {
                self.deref_mut().set_identity(identity)
            }

            fn request(
                &mut self,
                write_mode: WriteMode,
                on_into_read: MessageKind,
            ) -> Result<RequestWriter<'_>, Error> {
                self.deref_mut().request(write_mode, on_into_read)
            }

            fn to_url(&self) -> String {
                self.deref().to_url()
            }

            fn desired_protocol_version(&self) -> Protocol {
                self.deref().desired_protocol_version()
            }

            fn is_stateful(&self) -> bool {
                self.deref().is_stateful()
            }
        }
    }
}
pub use traits::TransportWithoutIO;

#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(all(feature = "blocking-client", feature = "http-client-curl"))]
pub use blocking_io::http;
#[cfg(feature = "blocking-client")]
pub use blocking_io::{
    connect, file, ssh, ExtendedBufRead, HandleProgress, RequestWriter, SetServiceResponse, Transport, TransportV2Ext,
};
#[cfg(feature = "blocking-client")]
#[doc(inline)]
pub use connect::connect;

///
pub mod capabilities;
#[doc(inline)]
pub use capabilities::Capabilities;

mod non_io_types;
pub use non_io_types::{Error, Identity, MessageKind, WriteMode};

///
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub mod git;
