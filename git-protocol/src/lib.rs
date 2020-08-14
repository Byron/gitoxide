#![forbid(unsafe_code)]

pub mod packet_line;

#[doc(inline)]
pub use packet_line::Borrowed as PacketLine;
