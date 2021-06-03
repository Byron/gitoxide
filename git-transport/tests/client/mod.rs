#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(not(feature = "http-client-curl"))]
mod capabilities;
mod git;
