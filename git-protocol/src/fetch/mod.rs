#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(feature = "blocking-client")]
pub use blocking_io::{
    arguments::Arguments,
    command::Command,
    delegate::{Action, Delegate},
    fetch, refs,
    refs::Ref,
    response,
    response::Response,
    Error,
};

/// Returns the name of the agent as key-value pair, commonly used in HTTP headers.
pub fn agent() -> (&'static str, Option<&'static str>) {
    ("agent", Some(concat!("git/oxide-", env!("CARGO_PKG_VERSION"))))
}
