#[cfg(not(feature = "http-client-curl"))]
mod capabilities;
#[cfg(not(feature = "http-client-curl"))]
mod git;
#[cfg(feature = "http-client-curl")]
mod http;
