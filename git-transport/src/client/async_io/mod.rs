use crate::{
    client::{Capabilities, Error, TransportWithoutIO},
    Protocol, Service,
};
use async_trait::async_trait;

mod bufread_ext;
pub use bufread_ext::{ExtendedBufRead, HandleProgress};

mod request;
pub use request::RequestWriter;

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

mod trait_ext;
pub use trait_ext::TransportV2Ext;

mod box_impl {
    use crate::{
        client::{self, Error, SetServiceResponse},
        Service,
    };
    use async_trait::async_trait;
    use std::ops::DerefMut;

    // Would be nice if the box implementation could auto-forward to all implemented traits.
    #[async_trait]
    impl<T: client::Transport + ?Sized + Send> client::Transport for Box<T> {
        async fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, Error> {
            self.deref_mut().handshake(service).await
        }

        async fn close(&mut self) -> Result<(), Error> {
            self.deref_mut().close().await
        }
    }
}
