#[cfg(feature = "blocking-client")]
pub use blocking_io::{
    delegate::{Action, Delegate},
    fetch, Error,
};

mod arguments;
pub use arguments::Arguments;

#[cfg(feature = "blocking-client")]
mod blocking_io;
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

///
pub mod response;
pub use response::Response;

#[cfg(test)]
mod tests;
