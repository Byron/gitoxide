use bstr::{BString, ByteSlice};
use git_transport::client::Capabilities;

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
        _version: git_transport::Protocol,
        _capabilities: &Capabilities,
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
                        .iter()
                        .any(|c| feature.starts_with(c.name().to_str_lossy().as_ref()))
                    {
                        continue;
                    }
                    panic!("{}: capability {} is not supported", self.as_str(), feature);
                }
            }
            git_transport::Protocol::V2 => {
                if let Some(allowed) = server.iter().find_map(|c| {
                    if c.name() == self.as_str().as_bytes().as_bstr() {
                        c.values().map(|v| v.map(|f| f.to_string()).collect::<Vec<_>>())
                    } else {
                        None
                    }
                }) {
                    for (feature, _) in features {
                        if allowed.iter().any(|allowed| feature.starts_with(allowed)) {
                            continue;
                        }
                        panic!("{}: V2 feature/capability {} is not supported", self.as_str(), feature);
                    }
                }
            }
        }
    }
}
