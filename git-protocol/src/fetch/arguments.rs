use crate::fetch::{command::Feature, Command};
use bstr::{BStr, BString};
use git_object::borrowed;
use git_transport::client::TransportV2Ext;
use git_transport::{client, Protocol};
use std::fmt;
use std::io::Write;

pub struct Arguments {
    base_args: Vec<BString>,

    args: Vec<BString>,
    haves: Vec<BString>,

    filter: bool,
    shallow: bool,
    deepen_since: bool,
    deepen_not: bool,
    deepen_relative: bool,

    features_for_first_want: Option<Vec<String>>,
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
            Some(features) => self.prefixed("want ", format!("{}\0{}", id, features.join(" "))),
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
    pub(crate) fn new(version: Protocol, features: &[Feature]) -> Self {
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
            base_args: initial_arguments.clone(),
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
        version: Protocol,
        transport: &'a mut T,
        features: &[Feature],
        is_done: bool,
    ) -> Result<Box<dyn client::ExtendedBufRead + 'a>, client::Error> {
        let has = |name: &str| features.iter().any(|(n, _)| *n == name);
        let mut add_done_argument = is_done;
        if has("no-done") && has("multi_ack_detailed") {
            add_done_argument = false;
        }
        match version {
            git_transport::Protocol::V1 => {
                let mut on_drop = if add_done_argument {
                    vec![client::MessageKind::Text(&b"done"[..])]
                } else {
                    vec![client::MessageKind::Flush]
                };
                let is_stateful = transport.is_stateful();
                let retained_state = if is_stateful { None } else { Some(self.args.clone()) };
                let mut line_writer = transport.request(client::WriteMode::OneLFTerminatedLinePerWriteCall, on_drop)?;

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
                Ok(line_writer.into_read())
            }
            git_transport::Protocol::V2 => {
                let mut arguments = std::mem::replace(&mut self.args, self.base_args.clone());
                if add_done_argument {
                    arguments.push("done".into());
                }
                transport.invoke(Command::Fetch.as_str(), features.iter().cloned(), Some(arguments))
            }
        }
    }
}
