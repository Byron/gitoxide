use std::{io::Write, ops::DerefMut};

use bstr::BString;

use crate::{
    client::{Capabilities, Error, ExtendedBufRead, MessageKind, TransportWithoutIO, WriteMode},
    Protocol, Service,
};

/// The response of the [`handshake()`][Transport::handshake()] method.
pub struct SetServiceResponse<'a> {
    /// The protocol the service can provide. May be different from the requested one
    pub actual_protocol: Protocol,
    /// The capabilities parsed from the server response.
    pub capabilities: Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    pub refs: Option<Box<dyn crate::client::ReadlineBufRead + 'a>>,
}

/// All methods provided here must be called in the correct order according to the [communication protocol][Protocol]
/// used to connect to them.
/// It does, however, know just enough to be able to provide a higher-level interface than would otherwise be possible.
/// Thus the consumer of this trait will not have to deal with packet lines at all.
/// **Note that**  whenever a `Read` trait or `Write` trait is produced, it must be exhausted.
pub trait Transport: TransportWithoutIO {
    /// Initiate connection to the given service and send the given `extra_parameters` along with it.
    ///
    /// `extra_parameters` are interpreted as `key=value` pairs if the second parameter is `Some` or as `key`
    /// if it is None.
    ///
    /// Returns the service capabilities according according to the actual [Protocol] it supports,
    /// and possibly a list of refs to be obtained.
    /// This means that asking for an unsupported protocol might result in a protocol downgrade to the given one
    /// if [`TransportWithoutIO::supported_protocol_versions()`] includes it.
    /// Exhaust the returned [`BufReader`][SetServiceResponse::refs] for a list of references in case of protocol V1
    /// before making another request.
    fn handshake<'a>(
        &mut self,
        service: Service,
        extra_parameters: &'a [(&'a str, Option<&'a str>)],
    ) -> Result<SetServiceResponse<'_>, Error>;
}

// Would be nice if the box implementation could auto-forward to all implemented traits.
impl<T: Transport + ?Sized> Transport for Box<T> {
    fn handshake<'a>(
        &mut self,
        service: Service,
        extra_parameters: &'a [(&'a str, Option<&'a str>)],
    ) -> Result<SetServiceResponse<'_>, Error> {
        self.deref_mut().handshake(service, extra_parameters)
    }
}

impl<T: Transport + ?Sized> Transport for &mut T {
    fn handshake<'a>(
        &mut self,
        service: Service,
        extra_parameters: &'a [(&'a str, Option<&'a str>)],
    ) -> Result<SetServiceResponse<'_>, Error> {
        self.deref_mut().handshake(service, extra_parameters)
    }
}

/// An extension trait to add more methods to everything implementing [`Transport`].
pub trait TransportV2Ext {
    /// Invoke a protocol V2 style `command` with given `capabilities` and optional command specific `arguments`.
    /// The `capabilities` were communicated during the handshake.
    /// If `trace` is `true`, then all packetlines written and received will be traced using facilities provided by the `gix_trace` crate.
    ///
    /// _Note:_ panics if [handshake][Transport::handshake()] wasn't performed beforehand.
    fn invoke<'a>(
        &mut self,
        command: &str,
        capabilities: impl Iterator<Item = (&'a str, Option<impl AsRef<str>>)> + 'a,
        arguments: Option<impl Iterator<Item = bstr::BString>>,
        trace: bool,
    ) -> Result<Box<dyn ExtendedBufRead + Unpin + '_>, Error>;
}

impl<T: Transport> TransportV2Ext for T {
    fn invoke<'a>(
        &mut self,
        command: &str,
        capabilities: impl Iterator<Item = (&'a str, Option<impl AsRef<str>>)> + 'a,
        arguments: Option<impl Iterator<Item = BString>>,
        trace: bool,
    ) -> Result<Box<dyn ExtendedBufRead + Unpin + '_>, Error> {
        let mut writer = self.request(WriteMode::OneLfTerminatedLinePerWriteCall, MessageKind::Flush, trace)?;
        writer.write_all(format!("command={command}").as_bytes())?;
        for (name, value) in capabilities {
            match value {
                Some(value) => writer.write_all(format!("{name}={}", value.as_ref()).as_bytes()),
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
