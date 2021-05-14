pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[cfg(feature = "blocking-io")]
mod blocking;
