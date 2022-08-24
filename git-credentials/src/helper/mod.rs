use bstr::BString;

/// The kind of helper program to use.
pub enum Kind {
    /// The built-in git-credential helper program, part of any git distribution.
    GitCredential,
}

/// Additional context to be passed to the credentials helper.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Context {
    /// The protocol over which the credential will be used (e.g., https).
    pub protocol: Option<String>,
    /// The remote hostname for a network credential. This includes the port number if one was specified (e.g., "example.com:8088").
    pub host: Option<String>,
    /// The path with which the credential will be used. E.g., for accessing a remote https repository, this will be the repository’s path on the server.
    /// It can also be a path on the file system.
    pub path: Option<BString>,
    /// The credential’s username, if we already have one (e.g., from a URL, the configuration, the user, or from a previously run helper).
    pub username: Option<String>,
    /// The credential’s password, if we are asking it to be stored.
    pub password: Option<String>,
    /// When this special attribute is read by git credential, the value is parsed as a URL and treated as if its constituent
    /// parts were read (e.g., url=<https://example.com> would behave as if
    /// protocol=https and host=example.com had been provided). This can help callers avoid parsing URLs themselves.
    pub url: Option<BString>,
}
///
pub mod context;

///
pub mod invoke;
pub use invoke::function::invoke;

///
pub mod main;
pub use main::function::main;
