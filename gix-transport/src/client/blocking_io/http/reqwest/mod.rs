/// An implementation for HTTP requests via `reqwest`.
pub struct Remote {
    /// A worker thread which performs the actual request.
    handle: Option<std::thread::JoinHandle<Result<(), remote::Error>>>,
    /// A channel to send requests (work) to the worker thread.
    request: std::sync::mpsc::SyncSender<remote::Request>,
    /// A channel to receive the result of the prior request.
    response: std::sync::mpsc::Receiver<remote::Response>,
    /// A mechanism for configuring the remote.
    config: crate::client::http::Options,
}

/// A function to configure a single request prior to sending it, support most complex configuration beyond what's possible with
/// basic `git` http configuration.
pub type ConfigureRequestFn = dyn FnMut(&mut reqwest::blocking::Request) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    + Send
    + Sync
    + 'static;

/// Options to configure the reqwest HTTP handler.
#[derive(Default)]
pub struct Options {
    /// A function to configure the request that is about to be made.
    pub configure_request: Option<Box<ConfigureRequestFn>>,
}

///
pub mod remote;
