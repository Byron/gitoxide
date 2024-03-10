mod init;
///
#[allow(clippy::empty_docs)]
pub mod verify;

///
#[allow(clippy::empty_docs)]
pub mod decode;

/// The bytes used as header in a pack data file.
pub type Header = [u8; 12];
