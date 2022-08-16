/// The direction of an operation carried out (or to be carried out) through a remote.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Direction {
    /// Push local changes to the remote.
    Push,
    /// Fetch changes from the remote to the local repository.
    Fetch,
}

impl Direction {
    /// Return ourselves as string suitable for use as verb in an english sentence.
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Push => "push",
            Direction::Fetch => "fetch",
        }
    }
}

mod build;
mod errors;
///
pub mod init;

#[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
pub mod connect;

#[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
mod connection {
    #![allow(missing_docs, dead_code)]
    use crate::remote;
    use crate::Remote;

    pub struct Connection<'repo, T> {
        pub(crate) remote: Remote<'repo>,
        pub(crate) direction: remote::Direction,
        pub(crate) transport: T,
    }
}
#[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
pub use connection::Connection;

pub use errors::find;

mod access;
pub(crate) mod url;
