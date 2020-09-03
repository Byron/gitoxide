use crate::credentials;
use bstr::ByteSlice;
use git_transport::{
    client::{self, SetServiceResponse, TransportV2Ext},
    Service,
};
use quick_error::quick_error;
use std::io;

mod refs;
pub use refs::Ref;

mod command;
pub use command::Command;

#[cfg(test)]
mod tests;

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

mod delegate;
pub use delegate::{Action, Delegate};

pub fn agent() -> (&'static str, Option<&'static str>) {
    ("agent", Some(concat!("git/oxide-", env!("CARGO_PKG_VERSION"))))
}

/// Note that depending on the `delegate`, the actual action peformed can be `ls-refs`, `clone` or `fetch`.
pub fn fetch<F: FnMut(credentials::Action) -> credentials::Result>(
    mut transport: impl client::Transport,
    delegate: &mut impl Delegate,
    mut authenticate: F,
) -> Result<(), Error> {
    let (protocol_version, mut parsed_refs, capabilities, call_ls_refs) = {
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

        let mut parsed_refs = Vec::<Ref>::new();
        refs::from_capabilities(&mut parsed_refs, capabilities.iter())?;

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
            protocol_version,
            git_transport::Protocol::V2,
            "Only V2 needs a separate request to get specific refs"
        );

        let mut ls_features = Vec::new();
        let ls_refs = Command::LsRefs;
        let mut ls_args: Vec<_> = ls_refs
            .collect_initial_features(protocol_version, &capabilities)
            .map(|(n, _)| n.as_bytes().as_bstr().to_owned())
            .collect();
        delegate.prepare_ls_refs(&capabilities, &mut ls_args, &mut ls_features);
        ls_refs.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &ls_args, &ls_features);

        let mut refs = transport.invoke(
            ls_refs.as_str(),
            ls_features.iter().cloned().chain(Some(agent())),
            if ls_args.is_empty() { None } else { Some(ls_args) },
        )?;
        refs::from_v2_refs(&mut parsed_refs, &mut refs)?;
    }

    // let mut fetch_features = Command::Fetch.collect_initial_features(protocol_version, &capabilities);
    let mut fetch_features = Vec::new();
    let next = delegate.prepare_fetch(protocol_version, &capabilities, &mut fetch_features, &parsed_refs);
    if next == Action::Close {
        transport.close()?;
        return Ok(());
    }

    transport.close()?;
    unimplemented!("rest of fetch or clone")
}
