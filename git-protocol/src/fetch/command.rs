use crate::fetch::agent;
use bstr::{BString, ByteSlice};
use git_transport::client::Capabilities;

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

    /// Only V2
    fn all_argument_prefixes(&self) -> &'static [&'static str] {
        match self {
            Command::LsRefs => &["symrefs", "peel", "ref-prefix "],
            Command::Fetch => &[
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
        }
    }

    fn all_features(&self, version: git_transport::Protocol) -> &'static [&'static str] {
        match self {
            Command::LsRefs => &[],
            Command::Fetch => match version {
                git_transport::Protocol::V1 => &[
                    "multi_ack",
                    "thin-pack",
                    "side-band",
                    "side-band-64k",
                    "ofs-delta",
                    "shallow",
                    "deepen-since",
                    "deepen-not",
                    "deepen-relative",
                    "no-progress",
                    "include-tag",
                    "multi_ack_detailed",
                    "allow-tip-sha1-in-want",
                    "allow-reachable-sha1-in-want",
                    "no-done",
                    "filter",
                ],
                git_transport::Protocol::V2 => &["shallow", "filter", "ref-in-want", "sideband-all", "packfile-uris"],
            },
        }
    }

    /// Compute initial arguments based on the given `features`. They are typically provided by the `default_features(…)` method.
    pub(crate) fn initial_arguments<'a>(&'a self, features: &[(&str, Option<&str>)]) -> Vec<BString> {
        match self {
            Command::Fetch => ["thin-pack", "include-tag", "ofs-delta"]
                .iter()
                .map(|s| s.as_bytes().as_bstr().to_owned())
                .chain(
                    ["sideband-all", "ref-in-want", "packfile-uris"]
                        .iter()
                        .filter(|f| features.iter().any(|(sf, _)| sf == *f))
                        .map(|f| f.as_bytes().as_bstr().to_owned()),
                )
                .collect(),
            Command::LsRefs => vec![b"symrefs".as_bstr().to_owned(), b"peel".as_bstr().to_owned()],
        }
    }

    pub(crate) fn default_features<'a>(
        &'a self,
        version: git_transport::Protocol,
        server_capabilities: &'a Capabilities,
    ) -> Vec<(&str, Option<&str>)> {
        match self {
            Command::Fetch => match version {
                git_transport::Protocol::V1 => {
                    let has_multi_ack_detailed = server_capabilities.contains("multi_ack_detailed");
                    let has_sideband_64k = server_capabilities.contains("side-band-64k");
                    self.all_features(version)
                        .iter()
                        .copied()
                        .filter(|feature| match *feature {
                            "side-band" if has_sideband_64k => false,
                            "multi_ack" if has_multi_ack_detailed => false,
                            "include-tag" | "no-progress" => false,
                            feature => server_capabilities.contains(feature),
                        })
                        .map(|s| (s, None))
                        .chain(Some(agent()))
                        .collect()
                }
                git_transport::Protocol::V2 => {
                    let supported_features = server_capabilities
                        .iter()
                        .find_map(|c| {
                            if c.name() == Command::Fetch.as_str().as_bytes().as_bstr() {
                                c.values().map(|v| v.map(|f| f.to_owned()).collect())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(Vec::new);
                    self.all_features(version)
                        .iter()
                        .copied()
                        .filter(|feature| supported_features.iter().any(|supported| supported == feature))
                        .map(|s| (s, None))
                        .chain(Some(agent()))
                        .collect()
                }
            },
            Command::LsRefs => {
                debug_assert!(
                    !server_capabilities.contains("ls-refs"),
                    "we don't currently know about any features for ls-refs. Time to have a look"
                );
                vec![agent()]
            }
        }
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
                if *self == Command::LsRefs {
                    if arguments.iter().any(|a| a.starts_with_str("ref-prefix ")) {
                        panic!("ref-prefix is not supported in V1 ls-refs");
                    }
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
