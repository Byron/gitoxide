use crate::fetch::command::Feature;
use crate::fetch::Command;
use bstr::{BStr, BString};
use git_object::borrowed;
use git_transport::Protocol;
use std::fmt;

pub struct Arguments {
    _base_args: Vec<BString>,
    args: Vec<BString>,

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
        self.prefixed("have ", id);
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
                    .collect();
                (Vec::new(), Some(baked_features))
            }
            Protocol::V2 => (Command::Fetch.initial_arguments(&features), None),
        };

        Arguments {
            _base_args: initial_arguments.clone(),
            args: initial_arguments,
            filter,
            shallow,
            deepen_not,
            deepen_relative,
            deepen_since,
            features_for_first_want,
        }
    }
}
