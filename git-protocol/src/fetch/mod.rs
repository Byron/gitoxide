#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(feature = "blocking-client")]
pub use blocking_io::{
    delegate::{Action, Delegate},
    fetch, response,
    response::Response,
    Error,
};

mod arguments;
pub use arguments::Arguments;

///
pub mod command;
pub use command::Command;

/// Returns the name of the agent as key-value pair, commonly used in HTTP headers.
pub fn agent() -> (&'static str, Option<&'static str>) {
    ("agent", Some(concat!("git/oxide-", env!("CARGO_PKG_VERSION"))))
}

///
pub mod refs;
pub use refs::Ref;

#[cfg(test)]
mod tests;
