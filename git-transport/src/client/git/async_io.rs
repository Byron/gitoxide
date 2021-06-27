use crate::{
    client::{self, capabilities, git, Capabilities, SetServiceResponse},
    Protocol, Service,
};
use async_trait::async_trait;
use bstr::BString;
use futures_io::{AsyncRead, AsyncWrite};
use futures_lite::AsyncWriteExt;
use git_packetline::PacketLine;

impl<R, W> client::TransportWithoutIO for git::Connection<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
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
    fn to_url(&self) -> String {
        self.custom_url.as_ref().map_or_else(
            || {
                git_url::Url {
                    scheme: git_url::Scheme::File,
                    user: None,
                    host: None,
                    port: None,
                    path: self.path.clone(),
                }
                .to_string()
            },
            |url| url.clone(),
        )
    }

    /// We implement this in a paranoid and safe way, not allowing downgrade to V1 which
    /// could send large amounts of refs in case we didn't want to support V1.
    fn supported_protocol_versions(&self) -> &[Protocol] {
        if self.desired_version == Protocol::V1 {
            &[]
        } else {
            &self.supported_versions
        }
    }

    fn is_stateful(&self) -> bool {
        true
    }
}

#[async_trait(?Send)]
impl<R, W> client::Transport for git::Connection<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    async fn handshake<'a>(
        &mut self,
        service: Service,
        extra_parameters: &[(&str, Option<&str>)],
    ) -> Result<SetServiceResponse<'_>, client::Error> {
        if self.mode == git::ConnectMode::Daemon {
            let mut line_writer = git_packetline::Writer::new(&mut self.writer).binary_mode();
            line_writer
                .write_all(&git::message::connect(
                    service,
                    self.desired_version,
                    &self.path,
                    self.virtual_host.as_ref(),
                    extra_parameters,
                ))
                .await?;
            line_writer.flush().await?;
        }

        let capabilities::recv::Outcome {
            capabilities,
            refs,
            protocol: actual_protocol,
        } = Capabilities::from_lines_with_version_detection(&mut self.line_provider).await?;
        Ok(SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        })
    }

    async fn close(&mut self) -> Result<(), client::Error> {
        git_packetline::encode::flush_to_write(&mut self.writer).await?;
        self.writer.flush().await?;
        Ok(())
    }
}

impl<R, W> git::Connection<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    /// Create a connection from the given `read` and `write`, asking for `desired_version` as preferred protocol
    /// and the transfer of the repository at `repository_path`.
    ///
    /// `virtual_host` along with a port to which to connect to, while `mode` determines the kind of endpoint to connect to.
    pub fn new(
        read: R,
        write: W,
        desired_version: Protocol,
        repository_path: impl Into<BString>,
        virtual_host: Option<(impl Into<String>, Option<u16>)>,
        mode: git::ConnectMode,
    ) -> Self {
        git::Connection {
            writer: write,
            line_provider: git_packetline::StreamingPeekableIter::new(read, &[PacketLine::Flush]),
            path: repository_path.into(),
            virtual_host: virtual_host.map(|(h, p)| (h.into(), p)),
            desired_version,
            custom_url: None,
            supported_versions: [desired_version],
            mode,
        }
    }
}
