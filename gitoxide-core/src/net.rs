use std::str::FromStr;

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub enum Protocol {
    V1,
    #[default]
    V2,
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Protocol::V1,
            "2" => Protocol::V2,
            _ => return Err(format!("Unsupported protocol version '{s}', choose '1' or '2'")),
        })
    }
}

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod impls {
    use gix::protocol::transport;

    use super::Protocol;

    impl From<Protocol> for transport::Protocol {
        fn from(v: Protocol) -> Self {
            match v {
                Protocol::V1 => transport::Protocol::V1,
                Protocol::V2 => transport::Protocol::V2,
            }
        }
    }
}

#[cfg(any(feature = "async-client", feature = "blocking-client"))]
pub use gix::protocol::transport::connect;
