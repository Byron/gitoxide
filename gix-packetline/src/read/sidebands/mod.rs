#[cfg(feature = "blocking-io")]
mod blocking_io;
#[cfg(feature = "blocking-io")]
pub use blocking_io::WithSidebands;

#[cfg(all(not(feature = "blocking-io"), feature = "async-io"))]
mod async_io;
#[cfg(all(not(feature = "blocking-io"), feature = "async-io"))]
pub use async_io::WithSidebands;
