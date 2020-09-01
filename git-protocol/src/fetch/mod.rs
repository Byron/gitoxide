use crate::credentials;
use bstr::{BStr, BString, ByteSlice};
use git_transport::{
    client::{self, SetServiceResponse},
    Service,
};
use quick_error::quick_error;
use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    io,
};

mod refs;
mod workaround;
pub use refs::Ref;

#[cfg(test)]
mod tests;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred while reading a response from or writing to the remote")
            from()
            source(err)
        }
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
        SymrefWithoutValue {
            display("A symref 'capability' is expected to have a value")
        }
        MalformedSymref(symref: BString) {
            display("'{}' could not be parsed. A symref is expected to look like <NAME>:<target>.", symref)
        }
        MalformedV1RefLine(line: String) {
            display("'{}' could not be parsed. A V1 ref line should be '<hex-hash> <path>'.", line)
        }
        InvariantViolation(message: &'static str) {
            display("{}", message)
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
    pub symrefs: Vec<BString>,
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

impl TryFrom<client::Capabilities> for Capabilities {
    type Error = Error;

    fn try_from(c: client::Capabilities) -> Result<Self, Self::Error> {
        let (available, symrefs) = {
            let mut map = BTreeMap::new();
            let mut symrefs = Vec::new();
            for c in c.iter() {
                if c.name() == b"symref".as_bstr() {
                    symrefs.push(c.value().ok_or(Error::SymrefWithoutValue)?.to_owned());
                } else {
                    map.insert(c.name().to_owned(), c.value().map(|v| v.to_owned()));
                }
            }
            (map, symrefs)
        };
        Ok(Capabilities { available, symrefs })
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

pub fn fetch<F: FnMut(credentials::Action) -> credentials::Result>(
    mut transport: impl client::Transport,
    mut delegate: impl Delegate,
    mut authenticate: F,
) -> Result<(), Error> {
    let SetServiceResponse {
        actual_protocol,
        capabilities,
        refs,
    } = match transport
        .handshake(Service::UploadPack)
        .map(workaround::LeakedSetServiceResponse::from)
    {
        Ok(v) => Ok(v),
        Err(client::Error::Io { err }) if err.kind() == io::ErrorKind::PermissionDenied => {
            let url = transport.to_url();
            let credentials::Outcome { identity, next } = authenticate(credentials::Action::Fill(&url))?;
            transport.set_identity(identity)?;
            match transport
                .handshake(Service::UploadPack)
                .map(workaround::LeakedSetServiceResponse::from)
            {
                Ok(v) => {
                    authenticate(next.approve())?;
                    Ok(v)
                }
                // Still no permission? Reject the credentials.
                Err(client::Error::Io { err }) if err.kind() == io::ErrorKind::PermissionDenied => {
                    authenticate(next.reject())?;
                    Err(client::Error::Io { err })
                }
                // Otherwise, do nothing, as we don't know if it actually got to try the credentials.
                // If they were previously stored, they remain. In the worst case, the user has to enter them again
                // next time they try.
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }?
    .into();

    let mut capabilities: Capabilities = capabilities.try_into()?;
    delegate.adjust_capabilities(actual_protocol, &mut capabilities);
    capabilities.set_agent_version();

    let mut parsed_refs = Vec::<Ref>::new();
    refs::from_capabilities(&mut parsed_refs, std::mem::take(&mut capabilities.symrefs))?;

    match refs {
        Some(mut refs) => {
            assert_eq!(
                actual_protocol,
                git_transport::Protocol::V1,
                "Only V1 auto-responds with refs"
            );
            refs::from_v1_refs_received_as_part_of_handshake(&mut parsed_refs, &mut refs)?;
        }
        None => {
            assert_eq!(
                actual_protocol,
                git_transport::Protocol::V2,
                "Only V2 needs a separate request to get specific refs"
            );
        }
    };

    unimplemented!("rest of fetch")
}
