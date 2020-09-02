use crate::credentials;
use bstr::{BStr, BString, ByteSlice};
use git_transport::{
    client::{self, SetServiceResponse},
    Service,
};
use quick_error::quick_error;
use std::{
    convert::{TryFrom, TryInto},
    io,
};

mod refs;
pub use refs::Ref;

#[cfg(test)]
mod tests;

// Note that arguments suffixed by spaces take another value.
const _BUILTIN_V2_COMMAND_ARGUMENT_NAMES: &[(&str, &[&str])] = &[
    ("ls-refs", &["symrefs", "peel", "ref-prefix "]),
    (
        "fetch",
        &[
            "want ", // hex oid
            "have ", // hex oid
            "done",
            "thin-pack",
            "no-progress",
            "include-tag",
            "ofs-delta",
            // Shallow feature/capability
            "shallow ", // hex oid
            "deepen ",  // commit depth
            "deepen-relative",
            "deepen-since ", // time-stamp
            "deepen-not ",   // rev
            // filter feature/capability
            "filter ", // filter-spec
            // ref-in-want feature
            "want-ref ", // ref path
            "sideband-all",
            // packfile-uris feature
            "packfile-uris ", // protocols
        ],
    ),
];

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
        SymrefWithoutValue {
            display("A symref 'capability' is expected to have a value")
        }
        Ref(err: refs::Error) {
            display("A reference could not be parsed or invariants were not met")
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

mod capabilities;
pub use capabilities::Capabilities;

pub fn fetch<F: FnMut(credentials::Action) -> credentials::Result>(
    mut transport: impl client::Transport,
    mut delegate: impl Delegate,
    mut authenticate: F,
) -> Result<(), Error> {
    let (actual_protocol, _refs, _capabilities, call_ls_refs) = {
        let result = transport.handshake(Service::UploadPack);
        let SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        } = match result {
            Ok(v) => Ok(v),
            Err(client::Error::Io { ref err }) if err.kind() == io::ErrorKind::PermissionDenied => {
                drop(result); // needed to workaround this: https://github.com/rust-lang/rust/issues/76149
                let url = transport.to_url();
                let credentials::Outcome { identity, next } = authenticate(credentials::Action::Fill(&url))?;
                transport.set_identity(identity)?;
                match transport.handshake(Service::UploadPack) {
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
        }?;

        let mut capabilities: Capabilities = capabilities.try_into()?;
        delegate.adjust_capabilities(actual_protocol, &mut capabilities);
        capabilities.set_agent_version();

        let mut parsed_refs = Vec::<Ref>::new();
        refs::from_capabilities(&mut parsed_refs, std::mem::take(&mut capabilities.symrefs))?;

        let call_ls_refs = match refs {
            Some(mut refs) => {
                assert_eq!(
                    actual_protocol,
                    git_transport::Protocol::V1,
                    "Only V1 auto-responds with refs"
                );
                refs::from_v1_refs_received_as_part_of_handshake(&mut parsed_refs, &mut refs)?;
                false
            }
            None => true,
        };
        (actual_protocol, parsed_refs, capabilities, call_ls_refs)
    }; // this scope is needed, see https://github.com/rust-lang/rust/issues/76149

    if call_ls_refs {
        assert_eq!(
            actual_protocol,
            git_transport::Protocol::V2,
            "Only V2 needs a separate request to get specific refs"
        );

        // capabilities: impl IntoIterator<Item = (&'a str, Option<&'a str>)>,
        // arguments: Option<impl IntoIterator<Item = BString>>,
        // let next = delegate.prepare_command("ls-refs", &capabilities);
        // transport.request(client::WriteMode::Binary, Vec::new())?;
    }

    transport.close()?;
    unimplemented!("rest of fetch")
}
