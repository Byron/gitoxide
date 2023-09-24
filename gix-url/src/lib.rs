//! A library implementing a URL for use in git with access to its special capabilities.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

use bstr::{BStr, BString};
use std::borrow::Cow;

///
pub mod parse;
#[doc(inline)]
pub use parse::parse;

///
pub mod expand_path;
#[doc(inline)]
pub use expand_path::expand_path;

mod scheme;
pub use scheme::Scheme;

/// A URL with support for specialized git related capabilities.
///
/// Additionally there is support for [deserialization][Url::from_bytes()] and serialization
/// (_see the `Display::fmt()` implementation_).
///
/// # Deviation
///
/// Note that we do not support passing the password using the URL as it's likely leading to accidents.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Url {
    /// The URL scheme.
    pub scheme: Scheme,
    /// The user to impersonate on the remote.
    user: Option<String>,
    /// The password associated with a user.
    password: Option<String>,
    /// The host to which to connect. Localhost is implied if `None`.
    host: Option<String>,
    /// When serializing, use the alternative forms as it was parsed as such.
    serialize_alternative_form: bool,
    /// The port to use when connecting to a host. If `None`, standard ports depending on `scheme` will be used.
    pub port: Option<u16>,
    /// The path portion of the URL, usually the location of the git repository.
    ///
    /// # Security-Warning
    ///
    /// URLs allow paths to start with `-` which makes it possible to mask command-line arguments as path which then leads to
    /// the invocation of programs from an attacker controlled URL. See <https://secure.phabricator.com/T12961> for details.
    ///
    /// If this value is going to be used in a command-line application, call [Self::path_argument_safe()] instead.
    pub path: bstr::BString,
}

/// Instantiation
impl Url {
    /// Create a new instance from the given parts, including a password, which will be validated by parsing them back.
    pub fn from_parts(
        scheme: Scheme,
        user: Option<String>,
        password: Option<String>,
        host: Option<String>,
        port: Option<u16>,
        path: BString,
        serialize_alternative_form: bool,
    ) -> Result<Self, parse::Error> {
        parse(
            Url {
                scheme,
                user,
                password,
                host,
                port,
                path,
                serialize_alternative_form,
            }
            .to_bstring()
            .as_ref(),
        )
    }
}

/// Modification
impl Url {
    /// Set the given `user`, with `None` unsetting it. Returns the previous value.
    pub fn set_user(&mut self, user: Option<String>) -> Option<String> {
        let prev = self.user.take();
        self.user = user;
        prev
    }
}

/// Builder
impl Url {
    /// Enable alternate serialization for this url, e.g. `file:///path` becomes `/path`.
    ///
    /// This is automatically set correctly for parsed URLs, but can be set here for urls
    /// created by constructor.
    pub fn serialize_alternate_form(mut self, use_alternate_form: bool) -> Self {
        self.serialize_alternative_form = use_alternate_form;
        self
    }

    /// Turn a file url like `file://relative` into `file:///root/relative`, hence it assures the url's path component is absolute,
    /// using `current_dir` if needed to achieve that.
    pub fn canonicalize(&mut self, current_dir: &std::path::Path) -> Result<(), gix_path::realpath::Error> {
        if self.scheme == Scheme::File {
            let path = gix_path::from_bstr(Cow::Borrowed(self.path.as_ref()));
            let abs_path = gix_path::realpath_opts(path.as_ref(), current_dir, gix_path::realpath::MAX_SYMLINKS)?;
            self.path = gix_path::into_bstr(abs_path).into_owned();
        }
        Ok(())
    }
}

/// Access
impl Url {
    /// Returns the user mentioned in the url, if present.
    pub fn user(&self) -> Option<&str> {
        self.user.as_deref()
    }
    /// Returns the password mentioned in the url, if present.
    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    /// Returns the host mentioned in the url, if present.
    ///
    /// # Security-Warning
    ///
    /// URLs allow hosts to start with `-` which makes it possible to mask command-line arguments as host which then leads to
    /// the invocation of programs from an attacker controlled URL. See <https://secure.phabricator.com/T12961> for details.
    ///
    /// If this value is going to be used in a command-line application, call [Self::host_argument_safe()] instead.
    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    /// Return the host of this URL if present *and* if it can't be mistaken for a command-line argument.
    ///
    /// Use this method if the host is going to be passed to a command-line application.
    pub fn host_argument_safe(&self) -> Option<&str> {
        self.host().filter(|host| !looks_like_argument(host.as_bytes()))
    }

    /// Return the path of this URL *and* if it can't be mistaken for a command-line argument.
    /// Note that it always begins with a slash, which is ignored for this comparison.
    ///
    /// Use this method if the path is going to be passed to a command-line application.
    pub fn path_argument_safe(&self) -> Option<&BStr> {
        self.path
            .get(1..)
            .and_then(|truncated| (!looks_like_argument(truncated)).then_some(self.path.as_ref()))
    }

    /// Returns true if the path portion of the url is `/`.
    pub fn path_is_root(&self) -> bool {
        self.path == "/"
    }
    /// Returns the actual or default port for use according to the url scheme.
    /// Note that there may be no default port either.
    pub fn port_or_default(&self) -> Option<u16> {
        self.port.or_else(|| {
            use Scheme::*;
            Some(match self.scheme {
                Http => 80,
                Https => 443,
                Ssh => 22,
                Git => 9418,
                File | Ext(_) => return None,
            })
        })
    }
}

fn looks_like_argument(b: &[u8]) -> bool {
    b.first() == Some(&b'-')
}

/// Transformation
impl Url {
    /// Turn a file url like `file://relative` into `file:///root/relative`, hence it assures the url's path component is absolute, using
    /// `current_dir` if necessary.
    pub fn canonicalized(&self, current_dir: &std::path::Path) -> Result<Self, gix_path::realpath::Error> {
        let mut res = self.clone();
        res.canonicalize(current_dir)?;
        Ok(res)
    }
}

/// Serialization
impl Url {
    /// Write this URL losslessly to `out`, ready to be parsed again.
    pub fn write_to(&self, mut out: &mut dyn std::io::Write) -> std::io::Result<()> {
        if !(self.serialize_alternative_form && (self.scheme == Scheme::File || self.scheme == Scheme::Ssh)) {
            out.write_all(self.scheme.as_str().as_bytes())?;
            out.write_all(b"://")?;
        }
        match (&self.user, &self.host) {
            (Some(user), Some(host)) => {
                out.write_all(user.as_bytes())?;
                if let Some(password) = &self.password {
                    out.write_all(&[b':'])?;
                    out.write_all(password.as_bytes())?;
                }
                out.write_all(&[b'@'])?;
                out.write_all(host.as_bytes())?;
            }
            (None, Some(host)) => {
                out.write_all(host.as_bytes())?;
            }
            (None, None) => {}
            (Some(_user), None) => unreachable!("BUG: should not be possible to have a user but no host"),
        };
        if let Some(port) = &self.port {
            write!(&mut out, ":{port}")?;
        }
        if self.serialize_alternative_form && self.scheme == Scheme::Ssh {
            out.write_all(b":")?;
        }
        out.write_all(&self.path)?;
        Ok(())
    }

    /// Transform ourselves into a binary string, losslessly, or fail if the URL is malformed due to host or user parts being incorrect.
    pub fn to_bstring(&self) -> bstr::BString {
        let mut buf = Vec::with_capacity(
            (5 + 3)
                + self.user.as_ref().map(String::len).unwrap_or_default()
                + 1
                + self.host.as_ref().map(String::len).unwrap_or_default()
                + self.port.map(|_| 5).unwrap_or_default()
                + self.path.len(),
        );
        self.write_to(&mut buf).expect("io cannot fail in memory");
        buf.into()
    }
}

/// Deserialization
impl Url {
    /// Parse a URL from `bytes`
    pub fn from_bytes(bytes: &BStr) -> Result<Self, parse::Error> {
        parse(bytes)
    }
}

mod impls;
