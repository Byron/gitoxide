use bstr::{BStr, BString};

/// The kind of helper program to use.
pub enum Kind {
    /// The built-in git-credential helper program, part of any git distribution.
    GitCredential,
}

/// Additional context to be passed to the credentials helper.
// TODO: fill in what's needed per configuration
#[derive(Debug, Default, Clone)]
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
    /// parts were read (e.g., url=https://example.com would behave as if
    /// protocol=https and host=example.com had been provided). This can help callers avoid parsing URLs themselves.
    pub url: Option<BString>,
}

/// The action to perform by the credentials [helper][`crate::helper()`].
#[derive(Clone, Debug)]
pub enum Action<'a> {
    /// Provide credentials using the given repository URL (as &str) as context and pre-parsed url information as seen in [`Context`].
    Fill(&'a BStr),
    /// Approve the credentials as identified by the previous input provided as `BString`, containing information from [`Context`].
    Approve(BString),
    /// Reject the credentials as identified by the previous input provided as `BString`. containing information from [`Context`].
    Reject(BString),
}

impl<'a> Action<'a> {
    /// Returns true if this action expects output from the helper.
    pub fn expects_output(&self) -> bool {
        matches!(self, Action::Fill(_))
    }
    /// The name of the argument to describe this action. If `is_custom` is true, the target program is
    /// a custom credentials helper, not a built-in one.
    pub fn as_helper_arg(&self, is_custom: bool) -> &str {
        match self {
            Action::Fill(_) if is_custom => "get",
            Action::Fill(_) => "fill",
            Action::Approve(_) if is_custom => "store",
            Action::Approve(_) => "approve",
            Action::Reject(_) if is_custom => "erase",
            Action::Reject(_) => "reject",
        }
    }
}

/// A handle to [approve][NextAction::approve()] or [reject][NextAction::reject()] the outcome of the initial action.
#[derive(Clone, Debug)]
pub struct NextAction {
    previous_output: BString,
}

impl NextAction {
    /// Approve the result of the previous [Action].
    pub fn approve(self) -> Action<'static> {
        Action::Approve(self.previous_output)
    }
    /// Reject the result of the previous [Action].
    pub fn reject(self) -> Action<'static> {
        Action::Reject(self.previous_output)
    }
}

///
pub mod invoke;
pub use invoke::function::invoke;

///
pub mod message {
    use bstr::{BStr, BString, ByteSlice};

    /// Encode `url` to `out` for consumption by a `git credentials` helper program.
    pub fn encode(url: &BStr, mut out: impl std::io::Write) -> std::io::Result<()> {
        validate(url)?;
        writeln!(out, "url={}\n", url)
    }

    fn validate(url: &BStr) -> std::io::Result<()> {
        if url.contains(&0) || url.contains(&b'\n') {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "token to encode must not contain newlines or null bytes",
            ));
        }
        Ok(())
    }

    /// Decode all lines in `input` as key-value pairs produced by a `git credentials` helper program.
    pub fn decode(mut input: impl std::io::Read) -> std::io::Result<Vec<(String, BString)>> {
        let mut buf = Vec::<u8>::with_capacity(512);
        input.read_to_end(&mut buf)?;
        buf.lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut it = line.splitn(2, |b| *b == b'=');
                match (it.next().and_then(|k| k.to_str().ok()), it.next().map(|v| v.as_bstr())) {
                    (Some(key), Some(value)) => validate(key.into())
                        .and_then(|_| validate(value.into()))
                        .map(|_| (key.into(), value.into())),
                    _ => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Invalid format, expecting key=value, got {:?}", line.as_bstr()),
                    )),
                }
            })
            .collect::<std::io::Result<Vec<_>>>()
    }
}
