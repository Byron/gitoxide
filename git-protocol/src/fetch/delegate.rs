use crate::fetch::Capabilities;
use bstr::{BString, ByteSlice};

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
    pub fn as_str(&self) -> &'static str {
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
    pub(crate) fn validate_prefixes_or_panic(&self, server: &Capabilities, arguments: &[BString], features: &[&str]) {
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

    /// Called before invoking ls-refs to allow providing it with additional `arguments` and to enable `features`.
    /// Note that some arguments are preset based on typical usage.
    /// The `server` capabilities can be used to see which additional capabilities the server supports as per the handshake.
    fn prepare_ls_refs(
        &mut self,
        _command: Command,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<&str>,
    ) {
    }
}
