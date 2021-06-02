use crate::{
    client::{self, capabilities, git, Capabilities, SetServiceResponse},
    Protocol, Service,
};
use async_trait::async_trait;
use bstr::BString;
use futures_io::{AsyncRead, AsyncWrite};
use futures_lite::AsyncWriteExt;
use git_packetline::PacketLine;
use std::io;

#[async_trait]
impl<R, W> client::Transport for git::Connection<R, W>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
    async fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, client::Error> {
        if self.mode == git::ConnectMode::Daemon {
            let mut line_writer = git_packetline::Writer::new(&mut self.writer).binary_mode();
            line_writer
                .write_all(&git::message::connect(
                    service,
                    self.desired_version,
                    &self.path,
                    self.virtual_host.as_ref(),
                ))
                .await?;
            line_writer.flush().await?;
        }

        // let capabilities::recv::Outcome {
        //     capabilities,
        //     refs,
        //     protocol: actual_protocol,
        // } = Capabilities::from_lines_with_version_detection(&mut self.line_provider)?;
        // Ok(SetServiceResponse {
        //     actual_protocol,
        //     capabilities,
        //     refs,
        // })
        todo!("Capabilities::from_lines_with_version_detection")
    }

    fn request(
        &mut self,
        write_mode: client::WriteMode,
        on_into_read: client::MessageKind,
    ) -> Result<client::RequestWriter<'_>, client::Error> {
        Ok(client::RequestWriter::new_from_bufread(
            &mut self.writer,
            Box::new(self.line_provider.as_read_without_sidebands()),
            write_mode,
            on_into_read,
        ))
    }

    async fn close(&mut self) -> Result<(), client::Error> {
        git_packetline::encode::flush_to_write(&mut self.writer).await?;
        self.writer.flush().await?;
        Ok(())
    }

    fn to_url(&self) -> String {
        git_url::Url {
            scheme: git_url::Scheme::File,
            user: None,
            host: None,
            port: None,
            path: self.path.clone(),
        }
        .to_string()
    }

    fn desired_protocol_version(&self) -> Protocol {
        self.desired_version
    }

    fn is_stateful(&self) -> bool {
        true
    }
}
