use crate::client::{Error, ExtendedBufRead, MessageKind, Transport, WriteMode};
use async_trait::async_trait;
use bstr::BString;
use futures_lite::io::AsyncWriteExt;

/// An extension trait to add more methods to everything implementing [`Transport`].
#[async_trait]
pub trait TransportV2Ext {
    /// Invoke a protocol V2 style `command` with given `capabilities` and optional command specific `arguments`.
    /// The `capabilities` were communicated during the handshake.
    /// _Note:_ panics if [handshake][Transport::handshake()] wasn't performed beforehand.
    async fn invoke<'a>(
        &mut self,
        command: &str,
        capabilities: impl IntoIterator<Item = (&'a str, Option<&'a str>)> + Send + 'a,
        arguments: Option<impl IntoIterator<Item = bstr::BString> + Send + 'a>,
    ) -> Result<Box<dyn ExtendedBufRead + '_>, Error>;
}

#[async_trait]
impl<T: Transport + Send> TransportV2Ext for T {
    async fn invoke<'a>(
        &mut self,
        command: &str,
        capabilities: impl IntoIterator<Item = (&'a str, Option<&'a str>)> + Send + 'a,
        arguments: Option<impl IntoIterator<Item = BString> + Send + 'a>,
    ) -> Result<Box<dyn ExtendedBufRead + '_>, Error> {
        let mut writer = self.request(WriteMode::OneLfTerminatedLinePerWriteCall, MessageKind::Flush)?;
        writer.write_all(format!("command={}", command).as_bytes()).await?;
        // for (name, value) in capabilities {
        //     match value {
        //         Some(value) => writer.write_all(format!("{}={}", name, value).as_bytes()).await,
        //         None => writer.write_all(name.as_bytes()).await,
        //     }?;
        // }
        // if let Some(arguments) = arguments {
        //     writer.write_message(MessageKind::Delimiter).await?;
        //     for argument in arguments {
        //         writer.write_all(argument.as_ref()).await?;
        //     }
        // }
        Ok(writer.into_read().await?)
    }
}
