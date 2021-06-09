use crate::{
    client::{Capabilities, Error, ExtendedBufRead, MessageKind, TransportWithoutIO, WriteMode},
    Protocol, Service,
};
use async_trait::async_trait;
use bstr::BString;
use futures_lite::io::AsyncWriteExt;
use std::ops::DerefMut;

/// The response of the [`handshake()`][Transport::handshake()] method.
pub struct SetServiceResponse<'a> {
    /// The protocol the service can provide. May be different from the requested one
    pub actual_protocol: Protocol,
    /// The capabilities parsed from the server response.
    pub capabilities: Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    pub refs: Option<Box<dyn futures_io::AsyncBufRead + Unpin + 'a>>,
}

/// All methods provided here must be called in the correct order according to the [communication protocol][Protocol]
/// used to connect to them.
/// It does, however, know just enough to be able to provide a higher-level interface than would otherwise be possible.
/// Thus the consumer of this trait will not have to deal with packet lines at all.
/// **Note that**  whenever a `Read` trait or `Write` trait is produced, it must be exhausted.
#[async_trait]
pub trait Transport: TransportWithoutIO {
    /// Initiate connection to the given service.
    /// Returns the service capabilities according according to the actual [Protocol] it supports,
    /// and possibly a list of refs to be obtained.
    /// This means that asking for an unsupported protocol will result in a protocol downgrade to the given one.
    /// using the `read_line(…)` function of the given [BufReader][SetServiceResponse::refs].
    /// It must be exhausted, that is, read to the end before the next method can be invoked.
    async fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, Error>;

    /// Closes the connection to indicate no further requests will be made.
    async fn close(&mut self) -> Result<(), Error>;
}

// Would be nice if the box implementation could auto-forward to all implemented traits.
#[async_trait]
impl<T: Transport + ?Sized + Send> Transport for Box<T> {
    async fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, Error> {
        self.deref_mut().handshake(service).await
    }

    async fn close(&mut self) -> Result<(), Error> {
        self.deref_mut().close().await
    }
}

/// An extension trait to add more methods to everything implementing [`Transport`].
#[async_trait]
pub trait TransportV2Ext {
    /// Invoke a protocol V2 style `command` with given `capabilities` and optional command specific `arguments`.
    /// The `capabilities` were communicated during the handshake.
    /// _Note:_ panics if [handshake][Transport::handshake()] wasn't performed beforehand.
    async fn invoke<'a>(
        &mut self,
        command: &str,
        capabilities: impl Iterator<Item = (&'a str, Option<&'a str>)> + 'a + Send,
        arguments: Option<impl Iterator<Item = bstr::BString> + 'a + Send>,
    ) -> Result<Box<dyn ExtendedBufRead + Unpin + '_ + Send>, Error>;
}

#[async_trait]
impl<T: Transport + Send> TransportV2Ext for T {
    async fn invoke<'a>(
        &mut self,
        command: &str,
        capabilities: impl Iterator<Item = (&'a str, Option<&'a str>)> + 'a + Send,
        arguments: Option<impl Iterator<Item = BString> + 'a + Send>,
    ) -> Result<Box<dyn ExtendedBufRead + Unpin + '_ + Send>, Error> {
        let mut writer = self.request(WriteMode::OneLfTerminatedLinePerWriteCall, MessageKind::Flush)?;
        writer.write_all(format!("command={}", command).as_bytes()).await?;
        for (name, value) in capabilities {
            match value {
                Some(value) => writer.write_all(format!("{}={}", name, value).as_bytes()).await,
                None => writer.write_all(name.as_bytes()).await,
            }?;
        }
        if let Some(arguments) = arguments {
            writer.write_message(MessageKind::Delimiter).await?;
            for argument in arguments {
                writer.write_all(argument.as_ref()).await?;
            }
        }
        Ok(writer.into_read().await?)
    }
}
