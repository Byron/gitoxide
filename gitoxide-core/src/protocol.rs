use git_protocol::git_transport;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Protocol {
    V1,
    V2,
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Protocol::V1,
            "2" => Protocol::V2,
            _ => return Err(format!("Unsupported protocol version '{}', choose '1' or '2'", s)),
        })
    }
}

impl From<Protocol> for git_transport::Protocol {
    fn from(v: Protocol) -> Self {
        match v {
            Protocol::V1 => git_transport::Protocol::V1,
            Protocol::V2 => git_transport::Protocol::V2,
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::V2
    }
}
