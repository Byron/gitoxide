use crate::credentials;
use bstr::{BStr, BString, ByteSlice};
use git_transport::{client, Service};
use quick_error::quick_error;
use std::collections::BTreeMap;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Credentials(err: credentials::Error) {
            display("Failed to obtain, approve or reject credentials")
            from()
            source(err)
        }
        Transport(err: client::Error) {
            display("An error occurred on the transport layer while fetching data")
            from()
            source(err)
        }
    }
}

pub trait Delegate {
    /// A chance to inspect or adjust the Capabilities returned after handshake with the server.
    /// They will be used in subsequent calls to the server, but the client is free to cache information as they see fit.
    fn adjust_capabilities(&mut self, _version: git_transport::Protocol, _capabilities: &mut Capabilities) {}
}

pub struct Capabilities {
    pub available: BTreeMap<BString, Option<BString>>,
}

impl Capabilities {
    /// Returns values of capability of the given name, if present.
    /// Useful when handling capabilities of V2 commands.
    pub fn values_of(&self, name: &str) -> Option<impl Iterator<Item = &BStr>> {
        self.available
            .get(name.as_bytes().as_bstr())
            .and_then(|v| v.as_ref().map(|v| v.split(|b| *b == b' ').map(|v| v.as_bstr())))
    }

    pub(crate) fn set_agent_version(&mut self) {
        self.available.insert(
            "agent".into(),
            Some(concat!("git/oxide-", env!("CARGO_PKG_VERSION")).into()),
        );
    }
}

impl From<client::Capabilities> for Capabilities {
    fn from(c: client::Capabilities) -> Self {
        Capabilities {
            available: {
                let mut map = BTreeMap::new();
                map.extend(c.iter().map(|c| (c.name().to_owned(), c.value().map(|v| v.to_owned()))));
                map
            },
        }
    }
}

// ("multi_ack", None),
// ("thin-pack", None),
// ("side-band", None),
// ("side-band-64k", None),
// ("ofs-delta", None),
// ("shallow", None),
// ("deepen-since", None),
// ("deepen-not", None),
// ("deepen-relative", None),
// ("no-progress", None),
// ("include-tag", None),
// ("multi_ack_detailed", None),
// ("allow-tip-sha1-in-want", None),
// ("allow-reachable-sha1-in-want", None),
// ("no-done", None),
// ("symref", Some("HEAD:refs/heads/main")),
// ("filter", None),
// ("agent", Some("git/github-gdf51a71f0236"))
//

// V1
// 0098want 808e50d724f604f69ab93c6da2919c014667bedb multi_ack_detailed no-done side-band-64k thin-pack ofs-delta deepen-since deepen-not agent=git/2.28.0

/// This monstrosity is only needed because for some reason, a match statement takes the drop scope of the enclosing scope, and not of
/// the match arm. This makes it think that a borrowed Ok(value) is still in scope, even though we are in the Err(err) branch.
/// The idea here is that we can workaround this by setting the scope to the level of the function, by splitting everything up accordingly.
/// Tracking issue: https://github.com/rust-lang/rust/issues/76149
fn fetch_inner<F: FnMut(credentials::Action) -> credentials::Result>(
    transport: &mut impl client::Transport,
    delegate: &mut impl Delegate,
    mut authenticate: Option<&mut F>,
    out: &mut Option<credentials::NextAction>,
) -> Result<(), Error> {
    if let Some(authenticate) = authenticate.as_mut() {
        let url = transport.to_url();
        let credentials::Outcome { identity, next } = authenticate(credentials::Action::Fill(&url))?;
        transport.set_identity(identity)?;
        // Remember the output of the authentication function to provide in case we still error with
        // a permission issue.
        *out = Some(next);
    };

    let client::SetServiceResponse {
        actual_protocol,
        capabilities,
        refs: _,
    } = transport.handshake(Service::UploadPack)?;

    if let Some(authenticate) = authenticate {
        authenticate(
            out.take()
                .expect("we put next action in before if an authenticator is present")
                .approve(),
        )?;
    }

    let mut capabilities = Capabilities::from(capabilities);
    delegate.adjust_capabilities(actual_protocol, &mut capabilities);
    capabilities.set_agent_version();
    unimplemented!("fetch")
}

pub fn fetch<F: FnMut(credentials::Action) -> credentials::Result>(
    mut transport: impl client::Transport,
    mut delegate: impl Delegate,
    mut authenticate: F,
) -> Result<(), Error> {
    let mut next = None;
    match fetch_inner(&mut transport, &mut delegate, None::<&mut F>, &mut next) {
        Ok(()) => Ok(()),
        Err(Error::Transport(client::Error::Io { err })) if err.kind() == io::ErrorKind::PermissionDenied => {
            fetch_inner(&mut transport, &mut delegate, Some(&mut authenticate), &mut next).map_err(|err| {
                if let Some(next) = next {
                    match &err {
                        // Still no permission? Reject the credentials
                        Error::Transport(client::Error::Io { err })
                            if err.kind() == io::ErrorKind::PermissionDenied =>
                        {
                            authenticate(next.reject())
                        }
                        // Otherwise it's some other error, still OK to approve the credentials
                        _ => authenticate(next.approve()),
                    }
                    .ok();
                };
                err
            })
        }
        Err(err) => Err(err),
    }
}
