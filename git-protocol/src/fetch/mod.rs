use crate::credentials;
use git_features::{progress, progress::Progress};
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

mod arguments;
pub use arguments::Arguments;

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
    mut progress: impl Progress,
) -> Result<(), Error> {
    let (protocol_version, mut parsed_refs, capabilities, call_ls_refs) = {
        progress.init(None, progress::steps());
        progress.set_name("handshake");
        progress.step();
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
                progress.set_name("authentication");
                let credentials::Outcome { identity, next } =
                    authenticate(credentials::Action::Fill(&url))?.expect("FILL provides an identity");
                transport.set_identity(identity)?;
                progress.step();
                progress.set_name("handshake (authenticated)");
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

    if transport.desired_protocol_version() != protocol_version {
        progress.info(format!(
            "server did not support protocol {} and downgraded to {}",
            transport.desired_protocol_version() as usize,
            protocol_version as usize,
        ));
    }

    if call_ls_refs {
        assert_eq!(
            protocol_version,
            git_transport::Protocol::V2,
            "Only V2 needs a separate request to get specific refs"
        );

        let ls_refs = Command::LsRefs;
        let mut ls_features = ls_refs.default_features(protocol_version, &capabilities);
        let mut ls_args = ls_refs.initial_arguments(&ls_features);
        delegate.prepare_ls_refs(&capabilities, &mut ls_args, &mut ls_features);
        ls_refs.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &ls_args, &ls_features);

        progress.step();
        progress.set_name("list refs");
        let mut remote_refs = transport.invoke(
            ls_refs.as_str(),
            ls_features,
            if ls_args.is_empty() { None } else { Some(ls_args) },
        )?;
        refs::from_v2_refs(&mut parsed_refs, &mut remote_refs)?;
    }

    let fetch = Command::Fetch;
    let mut fetch_features = fetch.default_features(protocol_version, &capabilities);
    let next = delegate.prepare_fetch(protocol_version, &capabilities, &mut fetch_features, &parsed_refs);
    fetch.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &[], &fetch_features);

    if next == Action::Close {
        transport.close()?;
        return Ok(());
    }
    let mut arguments = Arguments::new(protocol_version, &fetch_features);
    let previous_response = None::<Response>;
    // 16? Git does it that way, limiting the amount of lines sent at a time
    for round in 1..=16 {
        progress.step();
        progress.set_name(format!("negotiate (round {})", round));
        let action = delegate.negotiate(&parsed_refs, &mut arguments, previous_response.as_ref());
        arguments.send(
            protocol_version,
            &mut transport,
            &fetch_features,
            action == Action::Close,
        )?;
        // TODO: read result in a protocol independent way
        // match action {
        //     Action::Close {
        //
        //     }
        // }
    }

    // TODO: negotiation rounds till pack file is received or someone aborts.
    transport.close()?;
    Ok(())
}

mod response {
    pub struct Response;
}
pub use response::Response;
