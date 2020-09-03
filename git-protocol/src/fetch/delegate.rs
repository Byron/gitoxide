use crate::fetch::{Capabilities, Ref};
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
    Fetch,
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::LsRefs => "ls-refs",
            Command::Fetch => "fetch",
        }
    }

    fn all_argument_prefixes(&self) -> &'static [&'static str] {
        let name = self.as_str();
        BUILTIN_V2_COMMAND_ARGUMENT_NAMES
            .iter()
            .find_map(|(n, v)| if *n == name { Some(v) } else { None })
            .expect("command to be found")
    }
    fn all_features(&self) -> impl Iterator<Item = &str> {
        self.all_argument_prefixes()
            .iter()
            .filter(|s| !(s.ends_with(' ') || s.ends_with('=')))
            .map(|s| *s)
    }

    pub(crate) fn collect_initial_features(
        &self,
        version: git_transport::Protocol,
        capabtilies: &Capabilities,
    ) -> Vec<(&str, Option<&str>)> {
        let all_features = self.all_features();

        all_features
            .filter(|f| match self {
                Command::LsRefs => true,
                Command::Fetch => !["no-progress"].contains(f),
            })
            .map(|s| (s, None))
            .chain(Some(("agent", Some(concat!("git/oxide-", env!("CARGO_PKG_VERSION"))))))
            .collect()
    }
    /// Panics if the given arguments and features don't match what's statically known. It's considered a bug in the delegate.
    pub(crate) fn validate_argument_prefixes_or_panic(
        &self,
        version: git_transport::Protocol,
        server: &Capabilities,
        arguments: &[BString],
        features: &[(&str, Option<&str>)],
    ) {
        let allowed = self.all_argument_prefixes();
        for arg in arguments {
            if allowed.iter().any(|allowed| arg.starts_with(allowed.as_bytes())) {
                continue;
            }
            panic!("{}: argument {} is not known or allowed", self.as_str(), arg);
        }
        match version {
            git_transport::Protocol::V1 => {
                for (feature, _) in features {
                    if server
                        .available
                        .iter()
                        .any(|(allowed, _)| feature.starts_with(allowed.to_str_lossy().as_ref()))
                    {
                        continue;
                    }
                    panic!("{}: capability {} is not supported", self.as_str(), feature);
                }
            }
            git_transport::Protocol::V2 => {
                if let Some(allowed) = server
                    .values_of(self.as_str())
                    .map(|v| v.map(|f| f.to_str_lossy()).collect::<Vec<_>>())
                {
                    for (feature, _) in features {
                        if allowed.iter().any(|allowed| feature.starts_with(allowed.as_ref())) {
                            continue;
                        }
                        panic!("{}: V2 feature/capability {} is not supported", self.as_str(), feature);
                    }
                }
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
    /// Note that this is called only if we are using protocol version 2.
    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) {
    }

    /// Called before invoking the 'fetch' interaction, with `arguments` and `features` pre-filled for typical use.
    /// `refs` is a list of known references on the remote, based on the handshake or a prior call to ls_refs.
    /// As there will be another call allowing to post arguments conveniently in the correct format, i.e. `want hex-oid`,
    /// there is no way to set arguments at this time.
    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        _refs: &[Ref],
    ) -> Action {
        Action::Continue
    }
}
