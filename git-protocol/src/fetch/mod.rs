use crate::credentials;
use bstr::{BString, ByteSlice};
use git_transport::{
    client::{self, SetServiceResponse, TransportV2Ext},
    Service,
};
use quick_error::quick_error;
use std::{convert::TryInto, io};

mod refs;
pub use refs::Ref;

#[cfg(test)]
mod tests;

// Note that arguments suffixed by spaces take another value.
const BUILTIN_V2_COMMAND_ARGUMENT_NAMES: &[(&str, &[&str])] = &[
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

/// Define what to do next.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Action {
    /// Continue the typical flow of operations in this flow.
    Continue,
    /// Close the connection without making any further requests.
    Close,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Command {
    LsRefs,
}

impl Command {
    fn as_str(&self) -> &'static str {
        match self {
            Command::LsRefs => "ls-refs",
        }
    }
    fn builtin_argument_prefixes(&self) -> &'static [&'static str] {
        let name = self.as_str();
        BUILTIN_V2_COMMAND_ARGUMENT_NAMES
            .iter()
            .find_map(|(n, v)| if *n == name { Some(v) } else { None })
            .expect("command to be found")
    }

    /// Panics if the given arguments and features don't match what's statically known. It's considered a bug in the delegate.
    fn validate_prefixes_or_panic(&self, server: &Capabilities, arguments: &[BString], features: &[&str]) {
        let allowed = self.builtin_argument_prefixes();
        for arg in arguments {
            if allowed.iter().any(|allowed| arg.starts_with(allowed.as_bytes())) {
                continue;
            }
            panic!("{}: argument {} is not known or allowed", self.as_str(), arg);
        }
        if let Some(allowed) = server
            .values_of(self.as_str())
            .map(|v| v.map(|f| f.to_str_lossy()).collect::<Vec<_>>())
        {
            for feature in features {
                if allowed.iter().any(|allowed| feature.starts_with(allowed.as_ref())) {
                    continue;
                }
                panic!("{}: feature/capability {} is not supported", self.as_str(), feature);
            }
        }
    }
}

pub trait Delegate {
    /// A chance to inspect or adjust the Capabilities returned after handshake with the server.
    /// They will be used in subsequent calls to the server, but the client is free to cache information as they see fit.
    fn adjust_capabilities(&mut self, _version: git_transport::Protocol, _capabilities: &mut Capabilities) {}

    /// Called before invoking a given `command` to allow providing it with additional `arguments` and to enable `features`.
    /// Note that some arguments might be preset based on typical usage.
    /// The `server` capabilities can be used to see which additional capabilities the server supports as per the handshake.
    fn prepare_command(
        &mut self,
        _command: Command,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<&str>,
    ) -> Action {
        Action::Continue
    }
}

mod capabilities;
pub use capabilities::Capabilities;

/// Note that depending on the `delegate`, the actual action peformed can be `ls-refs`, `clone` or `fetch`.
pub fn fetch<F: FnMut(credentials::Action) -> credentials::Result>(
    mut transport: impl client::Transport,
    mut delegate: impl Delegate,
    mut authenticate: F,
) -> Result<(), Error> {
    let (actual_protocol, _refs, capabilities, call_ls_refs) = {
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

    let agent = (
        "agent".into(),
        Some(concat!("git/oxide-", env!("CARGO_PKG_VERSION")).into()),
    );
    if call_ls_refs {
        assert_eq!(
            actual_protocol,
            git_transport::Protocol::V2,
            "Only V2 needs a separate request to get specific refs"
        );

        let mut ls_features = Vec::new();
        let mut ls_args = vec!["peel".into(), "symrefs".into()];
        let ls_refs = Command::LsRefs;
        let _next = delegate.prepare_command(ls_refs, &capabilities, &mut ls_args, &mut ls_features);
        ls_refs.validate_prefixes_or_panic(&capabilities, &ls_args, &ls_features);

        let _refs = transport.invoke(
            ls_refs.as_str(),
            ls_features.iter().map(|f| (*f, None)).chain(Some(agent)),
            if ls_args.is_empty() { None } else { Some(ls_args) },
        )?;
    }

    transport.close()?;
    unimplemented!("rest of fetch")
}
