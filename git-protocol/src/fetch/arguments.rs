use crate::fetch::{self, command::Feature, Command};
use bstr::{BStr, BString, ByteSlice};
use git_object::borrowed;
use git_transport::{
    client::{self, TransportV2Ext},
    Protocol,
};
use std::{fmt, io::Write};

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
    pub fn can_use_filter(&self) -> bool {
        self.filter
    }
    pub fn can_use_shallow(&self) -> bool {
        self.shallow
    }
    pub fn can_use_deepen(&self) -> bool {
        self.shallow
    }
    pub fn can_use_deepen_since(&self) -> bool {
        self.deepen_since
    }
    pub fn can_use_deepen_not(&self) -> bool {
        self.deepen_not
    }
    pub fn can_use_deepen_relative(&self) -> bool {
        self.deepen_relative
    }

    pub fn want(&mut self, id: borrowed::Id) {
        match self.features_for_first_want.take() {
            Some(features) => self.prefixed("want ", format!("{} {}", id, features.join(" "))),
            None => self.prefixed("want ", id),
        }
    }
    pub fn have(&mut self, id: borrowed::Id) {
        self.haves.push(format!("have {}", id).into());
    }
    pub fn deepen(&mut self, depth: usize) {
        assert!(self.shallow, "'shallow' feature required for deepen");
        self.prefixed("deepen ", depth);
    }
    pub fn deepen_since(&mut self, seconds_since_unix_epoch: usize) {
        assert!(self.deepen_since, "'deepen-since' feature required");
        self.prefixed("deepen-since ", seconds_since_unix_epoch);
    }
    pub fn filter(&mut self, spec: &str) {
        assert!(self.filter, "'filter' feature required");
        self.prefixed("filter ", spec);
    }
    pub fn deepen_not(&mut self, ref_path: &BStr) {
        assert!(self.deepen_not, "'deepen-not' feature required");
        let mut line = BString::from("deepen-not ");
        line.extend_from_slice(&ref_path);
        self.args.push(line);
    }
    fn prefixed(&mut self, prefix: &str, value: impl fmt::Display) {
        self.args.push(format!("{}{}", prefix, value).into());
    }
    pub(crate) fn new(version: Protocol, features: Vec<Feature>) -> Result<Self, fetch::Error> {
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

        Ok(Arguments {
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
        })
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
                for arg in self.args.drain(..) {
                    line_writer.write_all(&arg)?;
                }

                line_writer.write_message(client::MessageKind::Flush)?;
                for line in self.haves.drain(..) {
                    line_writer.write_all(&line)?;
                }
                if let Some(next_args) = retained_state {
                    self.args = next_args;
                }
                Ok(line_writer.into_read()?)
            }
            git_transport::Protocol::V2 => {
                let retained_state = if transport.is_stateful() {
                    None
                } else {
                    Some(self.args.clone())
                };
                self.args.extend(self.haves.drain(..));
                if add_done_argument {
                    self.args.push("done".into());
                }
                transport.invoke(
                    Command::Fetch.as_str(),
                    self.features.iter().filter(|(_, v)| v.is_some()).cloned(),
                    Some(match retained_state {
                        None => std::mem::take(&mut self.args),
                        Some(args) => std::mem::replace(&mut self.args, args),
                    }),
                )
            }
        }
    }
}
