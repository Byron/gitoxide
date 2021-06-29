use bstr::{BStr, BString, ByteVec};
use std::fmt;

/// The arguments passed to a server command.
pub struct Arguments {
    /// The active features/capabilities of the fetch invocation
    #[cfg(any(feature = "async-client", feature = "blocking-client"))]
    features: Vec<crate::fetch::command::Feature>,

    args: Vec<BString>,
    haves: Vec<BString>,

    filter: bool,
    shallow: bool,
    deepen_since: bool,
    deepen_not: bool,
    deepen_relative: bool,
    ref_in_want: bool,

    features_for_first_want: Option<Vec<String>>,
    #[cfg(any(feature = "async-client", feature = "blocking-client"))]
    version: git_transport::Protocol,
}

impl Arguments {
    /// Return true if ref filters is supported.
    pub fn can_use_filter(&self) -> bool {
        self.filter
    }
    /// Return true if shallow refs are supported.
    ///
    /// This is relevant for partial clones when using `--depth X`.
    pub fn can_use_shallow(&self) -> bool {
        self.shallow
    }
    /// Return true if the 'deepen' capability is supported.
    ///
    /// This is relevant for partial clones when using `--depth X` and retrieving additional history.
    pub fn can_use_deepen(&self) -> bool {
        self.shallow
    }
    /// Return true if the 'deepen_since' capability is supported.
    ///
    /// This is relevant for partial clones when using `--depth X` and retrieving additional history
    /// based on a date beyond which all history should be present.
    pub fn can_use_deepen_since(&self) -> bool {
        self.deepen_since
    }
    /// Return true if the 'deepen_not' capability is supported.
    ///
    /// This is relevant for partial clones when using `--depth X`.
    pub fn can_use_deepen_not(&self) -> bool {
        self.deepen_not
    }
    /// Return true if the 'deepen_relative' capability is supported.
    ///
    /// This is relevant for partial clones when using `--depth X`.
    pub fn can_use_deepen_relative(&self) -> bool {
        self.deepen_relative
    }
    /// Return true if the 'ref-in-want' capability is supported.
    ///
    /// This can be used to bypass 'ls-refs' entirely in protocol v2.
    pub fn can_use_ref_in_want(&self) -> bool {
        self.ref_in_want
    }

    /// Add the given `id` pointing to a commit to the 'want' list.
    ///
    /// As such it should be included in the server response as it's not present on the client.
    pub fn want(&mut self, id: impl AsRef<git_hash::oid>) {
        match self.features_for_first_want.take() {
            Some(features) => self.prefixed("want ", format!("{} {}", id.as_ref(), features.join(" "))),
            None => self.prefixed("want ", id.as_ref()),
        }
    }
    /// Add the given ref to the 'want-ref' list.
    ///
    /// The server should respond with a corresponding 'wanted-refs' section if it will include the
    /// wanted ref in the packfile response.
    pub fn want_ref(&mut self, ref_path: &BStr) {
        let mut arg = BString::from("want-ref ");
        arg.push_str(ref_path);
        self.args.push(arg);
    }
    /// Add the given `id` pointing to a commit to the 'have' list.
    ///
    /// As such it should _not_ be included in the server response as it's already present on the client.
    pub fn have(&mut self, id: impl AsRef<git_hash::oid>) {
        self.haves.push(format!("have {}", id.as_ref()).into());
    }
    /// Add the given `id` pointing to a commit to the 'shallow' list.
    pub fn shallow(&mut self, id: impl AsRef<git_hash::oid>) {
        assert!(self.shallow, "'shallow' feature required for 'shallow <id>'");
        self.prefixed("shallow ", id.as_ref());
    }
    /// Deepen the commit history by `depth` amount of commits.
    pub fn deepen(&mut self, depth: usize) {
        assert!(self.shallow, "'shallow' feature required for deepen");
        self.prefixed("deepen ", depth);
    }
    /// Deepen the commit history to include all commits from now to `seconds_since_unix_epoch`.
    pub fn deepen_since(&mut self, seconds_since_unix_epoch: usize) {
        assert!(self.deepen_since, "'deepen-since' feature required");
        self.prefixed("deepen-since ", seconds_since_unix_epoch);
    }
    /// Deepen the commit history in a relative instead of absolute fashion.
    pub fn deepen_relative(&mut self) {
        assert!(self.deepen_relative, "'deepen-relative' feature required");
        self.args.push("deepen-relative".into());
    }
    /// Do not include commits reachable by the given `ref_path` when deepening the history.
    pub fn deepen_not(&mut self, ref_path: &BStr) {
        assert!(self.deepen_not, "'deepen-not' feature required");
        let mut line = BString::from("deepen-not ");
        line.extend_from_slice(&ref_path);
        self.args.push(line);
    }
    /// Set the given filter `spec` when listing references.
    pub fn filter(&mut self, spec: &str) {
        assert!(self.filter, "'filter' feature required");
        self.prefixed("filter ", spec);
    }
    fn prefixed(&mut self, prefix: &str, value: impl fmt::Display) {
        self.args.push(format!("{}{}", prefix, value).into());
    }
    #[cfg(any(feature = "async-client", feature = "blocking-client"))]
    pub(crate) fn new(version: git_transport::Protocol, features: Vec<crate::fetch::command::Feature>) -> Self {
        use crate::fetch::Command;
        let has = |name: &str| features.iter().any(|f| f.0 == name);
        let filter = has("filter");
        let shallow = has("shallow");
        let ref_in_want = has("ref-in-want");
        let mut deepen_since = shallow;
        let mut deepen_not = shallow;
        let mut deepen_relative = shallow;
        let (initial_arguments, features_for_first_want) = match version {
            git_transport::Protocol::V1 => {
                deepen_since = has("deepen-since");
                deepen_not = has("deepen-not");
                deepen_relative = has("deepen-relative");
                let baked_features = features
                    .iter()
                    .map(|(n, v)| match v {
                        Some(v) => format!("{}={}", n, v),
                        None => n.to_string(),
                    })
                    .collect::<Vec<_>>();
                (Vec::new(), Some(baked_features))
            }
            git_transport::Protocol::V2 => (Command::Fetch.initial_arguments(&features), None),
        };

        Arguments {
            features,
            version,
            args: initial_arguments,
            haves: Vec::new(),
            filter,
            shallow,
            deepen_not,
            deepen_relative,
            ref_in_want,
            deepen_since,
            features_for_first_want,
        }
    }
}

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod shared {
    use crate::fetch::Arguments;
    use bstr::{BString, ByteSlice};
    use git_transport::{client, client::MessageKind};

    impl Arguments {
        pub(in crate::fetch::arguments) fn prepare_v1(
            &mut self,
            transport_is_stateful: bool,
            add_done_argument: bool,
        ) -> Result<(MessageKind, Option<Vec<BString>>), client::Error> {
            if self.haves.is_empty() {
                assert!(add_done_argument, "If there are no haves, is_done must be true.");
            }
            let on_into_read = if add_done_argument {
                client::MessageKind::Text(&b"done"[..])
            } else {
                client::MessageKind::Flush
            };
            let retained_state = if transport_is_stateful {
                None
            } else {
                Some(self.args.clone())
            };

            if let Some(first_arg_position) = self.args.iter().position(|l| l.starts_with_str("want ")) {
                self.args.swap(first_arg_position, 0);
            }
            Ok((on_into_read, retained_state))
        }
    }
}

#[cfg(feature = "async-client")]
mod async_io;

#[cfg(feature = "blocking-client")]
mod blocking_io;
