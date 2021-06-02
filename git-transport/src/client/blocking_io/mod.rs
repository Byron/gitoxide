use std::io;

pub use crate::client::RequestWriter;
use crate::{
    client::{capabilities, Capabilities, Error, Identity, MessageKind, WriteMode},
    Protocol, Service,
};

///
pub mod connect;

///
pub mod file;
///
pub mod git;
///
#[cfg(feature = "http-client-curl")]
pub mod http;

mod bufread_ext;
pub use bufread_ext::ExtendedBufRead;

///
pub mod ssh;

/// The response of the [`handshake()`][Transport::handshake()] method.
pub struct SetServiceResponse<'a> {
    /// The protocol the service can provide. May be different from the requested one
    pub actual_protocol: Protocol,
    /// The capabilities parsed from the server response.
    pub capabilities: Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    pub refs: Option<Box<dyn io::BufRead + 'a>>,
}

/// All methods provided here must be called in the correct order according to the [communication protocol][Protocol]
/// used to connect to them.
/// It does, however, know just enough to be able to provide a higher-level interface than would otherwise be possible.
/// Thus the consumer of this trait will not have to deal with packet lines at all.
/// **Note that**  whenever a `Read` trait or `Write` trait is produced, it must be exhausted.
pub trait Transport {
    /// Initiate connection to the given service.
    /// Returns the service capabilities according according to the actual [Protocol] it supports,
    /// and possibly a list of refs to be obtained.
    /// This means that asking for an unsupported protocol will result in a protocol downgrade to the given one.
    /// using the `read_line(…)` function of the given [BufReader][SetServiceResponse::refs].
    /// It must be exhausted, that is, read to the end before the next method can be invoked.
    fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, Error>;

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

    /// Closes the connection to indicate no further requests will be made.
    fn close(&mut self) -> Result<(), Error>;

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

mod trait_ext {
    use std::io::Write;

    use bstr::BString;

    use crate::client::{Error, ExtendedBufRead, MessageKind, Transport, WriteMode};

    /// An extension trait to add more methods to everything implementing [`Transport`].
    pub trait TransportV2Ext {
        /// Invoke a protocol V2 style `command` with given `capabilities` and optional command specific `arguments`.
        /// The `capabilities` were communicated during the handshake.
        /// _Note:_ panics if [handshake][Transport::handshake()] wasn't performed beforehand.
        fn invoke<'a>(
            &mut self,
            command: &str,
            capabilities: impl IntoIterator<Item = (&'a str, Option<&'a str>)>,
            arguments: Option<impl IntoIterator<Item = bstr::BString>>,
        ) -> Result<Box<dyn ExtendedBufRead + '_>, Error>;
    }

    impl<T: Transport> TransportV2Ext for T {
        fn invoke<'a>(
            &mut self,
            command: &str,
            capabilities: impl IntoIterator<Item = (&'a str, Option<&'a str>)>,
            arguments: Option<impl IntoIterator<Item = BString>>,
        ) -> Result<Box<dyn ExtendedBufRead + '_>, Error> {
            let mut writer = self.request(WriteMode::OneLfTerminatedLinePerWriteCall, MessageKind::Flush)?;
            writer.write_all(format!("command={}", command).as_bytes())?;
            for (name, value) in capabilities {
                match value {
                    Some(value) => writer.write_all(format!("{}={}", name, value).as_bytes()),
                    None => writer.write_all(name.as_bytes()),
                }?;
            }
            if let Some(arguments) = arguments {
                writer.write_message(MessageKind::Delimiter)?;
                for argument in arguments {
                    writer.write_all(argument.as_ref())?;
                }
            }
            Ok(writer.into_read()?)
        }
    }
}
pub use trait_ext::TransportV2Ext;

mod box_impl {
    use std::ops::{Deref, DerefMut};

    use crate::{
        client::{self, Error, Identity, MessageKind, RequestWriter, SetServiceResponse, WriteMode},
        Protocol, Service,
    };

    // Would be nice if the box implementation could auto-forward to all implemented traits.
    impl<T: client::Transport + ?Sized> client::Transport for Box<T> {
        fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, Error> {
            self.deref_mut().handshake(service)
        }

        fn set_identity(&mut self, identity: Identity) -> Result<(), Error> {
            self.deref_mut().set_identity(identity)
        }

        fn request(&mut self, write_mode: WriteMode, on_into_read: MessageKind) -> Result<RequestWriter<'_>, Error> {
            self.deref_mut().request(write_mode, on_into_read)
        }

        fn close(&mut self) -> Result<(), Error> {
            self.deref_mut().close()
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
