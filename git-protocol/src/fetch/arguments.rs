use crate::fetch::command::Feature;
use bstr::BString;
use git_transport::Protocol;

pub struct Arguments {
    args: Vec<BString>,
    filter: bool,
    shallow: bool,
    deepen_since: bool,
    deepen_not: bool,
    deepen_relative: bool,
}

impl Arguments {
    pub fn can_use_filter(&self) -> bool {
        self.filter
    }
    pub fn can_use_shallow(&self) -> bool {
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
    pub(crate) fn new(initial_arguments: Vec<BString>, version: Protocol, features: &[Feature]) -> Self {
        let has = |name: &str| features.iter().any(|f| f.0 == name);
        let filter = has("filter");
        let shallow = has("shallow");
        let mut deepen_since = shallow;
        let mut deepen_not = shallow;
        let mut deepen_relative = shallow;
        match version {
            Protocol::V1 => {
                deepen_since = has("deepen-since");
                deepen_not = has("deepen-not");
                deepen_relative = has("deepen-relative");
            }
            Protocol::V2 => {}
        };

        Arguments {
            args: initial_arguments,
            filter,
            shallow,
            deepen_not,
            deepen_relative,
            deepen_since,
        }
    }
}
