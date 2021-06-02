#[cfg(feature = "blocking-client")]
mod blocking_io {
    use crate::client::{ExtendedBufRead, MessageKind, WriteMode};
    use std::io;

    /// A [`Write`][io::Write] implementation optimized for writing packet lines.
    /// A type implementing `Write` for packet lines, which when done can be transformed into a `Read` for
    /// obtaining the response.
    pub struct RequestWriter<'a> {
        on_into_read: MessageKind,
        pub(crate) writer: git_packetline::Writer<Box<dyn io::Write + 'a>>,
        pub(crate) reader: Box<dyn ExtendedBufRead + 'a>,
    }

    impl<'a> io::Write for RequestWriter<'a> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.writer.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.writer.flush()
        }
    }

    /// methods with bonds to IO
    impl<'a> RequestWriter<'a> {
        /// Create a new instance from a `writer` (commonly a socket), a `reader` into which to transform once the
        /// writes are finished, along with configuration for the `write_mode` and information about which message to write
        /// when this instance is converted into a `reader` to read the request's response.
        pub fn new_from_bufread<W: io::Write + 'a>(
            writer: W,
            reader: Box<dyn ExtendedBufRead + 'a>,
            write_mode: WriteMode,
            on_into_read: MessageKind,
        ) -> Self {
            let mut writer = git_packetline::Writer::new(Box::new(writer) as Box<dyn io::Write>);
            match write_mode {
                WriteMode::Binary => writer.enable_binary_mode(),
                WriteMode::OneLfTerminatedLinePerWriteCall => writer.enable_text_mode(),
            }
            RequestWriter {
                on_into_read,
                writer,
                reader,
            }
        }

        /// Write the given message as packet line.
        pub fn write_message(&mut self, message: MessageKind) -> io::Result<()> {
            match message {
                MessageKind::Flush => git_packetline::PacketLine::Flush.to_write(self.writer.inner_mut()),
                MessageKind::Delimiter => git_packetline::PacketLine::Delimiter.to_write(self.writer.inner_mut()),
                MessageKind::ResponseEnd => git_packetline::PacketLine::ResponseEnd.to_write(self.writer.inner_mut()),
                MessageKind::Text(t) => git_packetline::immutable::Text::from(t).to_write(self.writer.inner_mut()),
            }
            .map(|_| ())
        }

        /// Discard the ability to write and turn this instance into the reader for obtaining the other side's response.
        pub fn into_read(mut self) -> std::io::Result<Box<dyn ExtendedBufRead + 'a>> {
            self.write_message(self.on_into_read)?;
            Ok(self.reader)
        }
    }
}
#[cfg(feature = "blocking-client")]
pub use blocking_io::RequestWriter;

#[cfg(all(not(feature = "blocking-client"), feature = "async-client"))]
mod async_io {
    use crate::client::{ExtendedBufRead, MessageKind};
    use futures_io::AsyncWrite;
    use pin_project_lite::pin_project;
    use std::{
        io,
        pin::Pin,
        task::{Context, Poll},
    };

    pin_project! {
        /// A [`Write`][io::Write] implementation optimized for writing packet lines.
        /// A type implementing `Write` for packet lines, which when done can be transformed into a `Read` for
        /// obtaining the response.
        pub struct RequestWriter<'a> {
            on_into_read: MessageKind,
            #[pin]
            pub(crate) writer: git_packetline::Writer<Box<dyn AsyncWrite + Unpin + 'a>>,
            pub(crate) reader: Box<dyn ExtendedBufRead + 'a>,
        }
    }
    impl<'a> futures_io::AsyncWrite for RequestWriter<'a> {
        fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
            self.project().writer.poll_write(cx, buf)
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            self.project().writer.poll_flush(cx)
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            self.project().writer.poll_close(cx)
        }
    }

    // /// methods with bonds to IO
    // impl<'a> RequestWriter<'a> {
    //     /// Create a new instance from a `writer` (commonly a socket), a `reader` into which to transform once the
    //     /// writes are finished, along with configuration for the `write_mode` and information about which message to write
    //     /// when this instance is converted into a `reader` to read the request's response.
    //     pub fn new_from_bufread<W: AsyncWrite + 'a>(
    //         writer: W,
    //         reader: Box<dyn ExtendedBufRead + 'a>,
    //         write_mode: WriteMode,
    //         on_into_read: MessageKind,
    //     ) -> Self {
    //         let mut writer = git_packetline::Writer::new(Box::new(writer) as Box<dyn io::Write>);
    //         match write_mode {
    //             WriteMode::Binary => writer.enable_binary_mode(),
    //             WriteMode::OneLfTerminatedLinePerWriteCall => writer.enable_text_mode(),
    //         }
    //         RequestWriter {
    //             on_into_read,
    //             writer,
    //             reader,
    //         }
    //     }
    //
    //     /// Write the given message as packet line.
    //     pub async fn write_message(&mut self, message: MessageKind) -> io::Result<()> {
    //         match message {
    //             MessageKind::Flush => git_packetline::PacketLine::Flush.to_write(self.writer.inner_mut()),
    //             MessageKind::Delimiter => git_packetline::PacketLine::Delimiter.to_write(self.writer.inner_mut()),
    //             MessageKind::ResponseEnd => git_packetline::PacketLine::ResponseEnd.to_write(self.writer.inner_mut()),
    //             MessageKind::Text(t) => git_packetline::immutable::Text::from(t).to_write(self.writer.inner_mut()),
    //         }
    //         .map(|_| ())
    //         .await
    //     }
    // /// Discard the ability to write and turn this instance into the reader for obtaining the other side's response.
    // pub fn into_read(mut self) -> std::io::Result<Box<dyn ExtendedBufRead + 'a>> {
    //     self.write_message(self.on_into_read)?;
    //     Ok(self.reader)
    // }
    // }
}
#[cfg(all(not(feature = "blocking-client"), feature = "async-client"))]
pub use async_io::RequestWriter;
