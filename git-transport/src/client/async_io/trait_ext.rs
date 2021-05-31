use crate::client::{Error, ExtendedBufRead, MessageKind, Transport, WriteMode};
use bstr::BString;
use std::io::Write;

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
