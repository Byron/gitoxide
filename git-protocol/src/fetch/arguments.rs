use crate::fetch::{command::Feature, Command};
use bstr::{BStr, BString, ByteSlice};
use git_object::borrowed;
use git_transport::{
    client::{self, TransportV2Ext},
    Protocol,
};
use std::{fmt, io::Write};

/// The arguments passed to a server command.
pub struct Arguments {
    /// The active features/capabilities of the fetch invocation
    features: Vec<Feature>,

    args: Vec<BString>,
    haves: Vec<BString>,

    filter: bool,
    shallow: bool,
    deepen_since: bool,
    deepen_not: bool,
    deepen_relative: bool,

    features_for_first_want: Option<Vec<String>>,
    version: Protocol,
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

    /// Add the given `id` pointing to a commit to the 'want' list.
    ///
    /// As such it should be included in the server response as it's not present on the client.
    pub fn want(&mut self, id: borrowed::Id<'_>) {
        match self.features_for_first_want.take() {
            Some(features) => self.prefixed("want ", format!("{} {}", id, features.join(" "))),
            None => self.prefixed("want ", id),
        }
    }
    /// Add the given `id` pointing to a commit to the 'have' list.
    ///
    /// As such it should _not_ be included in the server response as it's already present on the client.
    pub fn have(&mut self, id: borrowed::Id<'_>) {
        self.haves.push(format!("have {}", id).into());
    }
    /// Add the given `id` pointing to a commit to the 'shallow' list.
    pub fn shallow(&mut self, id: borrowed::Id<'_>) {
        assert!(self.shallow, "'shallow' feature required for 'shallow <id>'");
        self.prefixed("shallow ", id);
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
    pub(crate) fn new(version: Protocol, features: Vec<Feature>) -> Self {
        let has = |name: &str| features.iter().any(|f| f.0 == name);
        let filter = has("filter");
        let shallow = has("shallow");
        let mut deepen_since = shallow;
        let mut deepen_not = shallow;
        let mut deepen_relative = shallow;
        let (initial_arguments, features_for_first_want) = match version {
            Protocol::V1 => {
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
            Protocol::V2 => (Command::Fetch.initial_arguments(&features), None),
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
            deepen_since,
            features_for_first_want,
        }
    }
    pub(crate) fn send<'a, T: client::Transport + 'a>(
        &mut self,
        transport: &'a mut T,
        add_done_argument: bool,
    ) -> Result<Box<dyn client::ExtendedBufRead + 'a>, client::Error> {
        if self.haves.is_empty() {
            assert!(add_done_argument, "If there are no haves, is_done must be true.");
        }
        match self.version {
            git_transport::Protocol::V1 => {
                let on_into_read = if add_done_argument {
                    client::MessageKind::Text(&b"done"[..])
                } else {
                    client::MessageKind::Flush
                };
                let retained_state = if transport.is_stateful() {
                    None
                } else {
                    Some(self.args.clone())
                };
                let mut line_writer =
                    transport.request(client::WriteMode::OneLFTerminatedLinePerWriteCall, on_into_read)?;

                if let Some(first_arg_position) = self.args.iter().position(|l| l.starts_with_str("want ")) {
                    self.args.swap(first_arg_position, 0);
                }
                let had_args = !self.args.is_empty();
                for arg in self.args.drain(..) {
                    line_writer.write_all(&arg)?;
                }
                if had_args {
                    line_writer.write_message(client::MessageKind::Flush)?;
                }
                for line in self.haves.drain(..) {
                    line_writer.write_all(&line)?;
                }
                if let Some(next_args) = retained_state {
                    self.args = next_args;
                }
                Ok(line_writer.into_read()?)
            }
            git_transport::Protocol::V2 => {
                let retained_state = self.args.clone();
                self.args.extend(self.haves.drain(..));
                if add_done_argument {
                    self.args.push("done".into());
                }
                transport.invoke(
                    Command::Fetch.as_str(),
                    self.features.iter().filter(|(_, v)| v.is_some()).cloned(),
                    Some(std::mem::replace(&mut self.args, retained_state)),
                )
            }
        }
    }
}
