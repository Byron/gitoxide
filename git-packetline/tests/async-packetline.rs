pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[cfg(all(feature = "async-io", not(feature = "blocking-io")))]
mod async_io;
