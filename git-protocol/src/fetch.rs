use crate::credentials;
use bstr::{BStr, BString, ByteSlice};
use git_transport::client::SetServiceResponse;
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

/// This types sole purpose is to 'disable' the destructor on the Box provided in the `SetServiceResponse` type
/// by leaking the box. We provide a method to restore the box and drop it right away to not actually leak.
/// However, we do leak in error cases because we don't call the manual destructor then.
struct LeakedSetServiceResponse<'a> {
    /// The protocol the service can provide. May be different from the requested one
    pub actual_protocol: git_transport::Protocol,
    pub capabilities: Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    pub refs: Option<&'a mut dyn io::BufRead>,
}

impl<'a> From<client::SetServiceResponse<'a>> for LeakedSetServiceResponse<'a> {
    fn from(v: SetServiceResponse<'a>) -> Self {
        LeakedSetServiceResponse {
            actual_protocol: v.actual_protocol,
            capabilities: v.capabilities.into(),
            refs: v.refs.map(Box::leak),
        }
    }
}

impl<'a> LeakedSetServiceResponse<'a> {
    fn drop_explicitly(&mut self) {
        if let Some(b) = self.refs.take() {
            // SAFETY: We are bound to lifetime 'a, which is the lifetime of the thing pointed to by the trait object in the box.
            // Thus we can only drop the box while that thing is indeed valid, due to Rusts standard lifetime rules.
            // The box itself was leaked by us.
            // Note that this is only required because Drop scopes are the outer ones in the match, not the match arms, making them
            // too broad to be usable intuitively. I consider this a technical shortcoming and hope there is a way to resolve it.
            #[allow(unsafe_code)]
            unsafe {
                drop(Box::from_raw(b as *mut _))
            }
        }
    }
}

pub fn fetch<F: FnMut(credentials::Action) -> credentials::Result>(
    mut transport: impl client::Transport,
    mut delegate: impl Delegate,
    mut authenticate: F,
) -> Result<(), Error> {
    let mut res: LeakedSetServiceResponse = match transport.handshake(Service::UploadPack).map(Into::into) {
        Ok(v) => Ok(v),
        Err(client::Error::Io { err }) if err.kind() == io::ErrorKind::PermissionDenied => {
            let url = transport.to_url();
            let credentials::Outcome { identity, next } = authenticate(credentials::Action::Fill(&url))?;
            transport.set_identity(identity)?;
            match transport.handshake(Service::UploadPack).map(Into::into) {
                Ok(v) => {
                    authenticate(next.approve())?;
                    Ok(v)
                }
                // Still no permission? Reject the credentials.
                Err(client::Error::Io { err }) if err.kind() == io::ErrorKind::PermissionDenied => {
                    authenticate(next.reject())?;
                    Err(client::Error::Io { err })
                }
                // Otherwise it's some other error, still OK to approve the credentials. We also do this to not accidentally
                // discard credentials that have been previously stored.
                Err(err) => {
                    authenticate(next.approve())?;
                    Err(err)
                }
            }
        }
        Err(err) => Err(err),
    }?;

    delegate.adjust_capabilities(res.actual_protocol, &mut res.capabilities);
    res.capabilities.set_agent_version();

    res.drop_explicitly();
    unimplemented!("rest of fetch")
}
