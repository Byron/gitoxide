use crate::fetch::command::Feature;
use bstr::BString;
use git_object::owned;
use git_transport::{client, Protocol};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        MissingServerCapability(feature: &'static str) {
            display("Currently we require feature '{}', which is not supported by the server", feature)
        }
        UnknownPrefix(prefix: BString) {
            display("Encountered an unknown line prefix: {}", prefix)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Acknowledgement {
    Common(owned::Id),
    Ready,
    NAK,
}

impl Acknowledgement {
    pub fn id(&self) -> Option<&owned::Id> {
        match self {
            Acknowledgement::Common(id) => Some(id),
            _ => None,
        }
    }
}

/// A representation of a complete fetch response
pub struct Response {
    acks: Option<Vec<Acknowledgement>>,
}

impl Response {
    pub fn check_required_features(features: &[Feature]) -> Result<(), Error> {
        let has = |name: &str| features.iter().any(|f| f.0 == name);
        // Let's focus on V2 standards, and simply not support old servers to keep our code simpler
        if !has("multi_ack_detailed") {
            return Err(Error::MissingServerCapability("multi_ack_detailed"));
        }
        // It's easy to NOT do sideband for us, but then again, everyone supports it.
        if !has("side-band") && !has("side-band-64k") {
            return Err(Error::MissingServerCapability("side-band OR side-band-64k"));
        }
        Ok(())
    }
    pub fn from_line_reader(
        version: Protocol,
        _reader: Box<dyn client::ExtendedBufRead + '_>,
    ) -> Result<Response, Error> {
        match version {
            Protocol::V1 => unimplemented!("read v1"),
            Protocol::V2 => unimplemented!("read v2"),
        }
    }

    pub fn acknowledgements(&self) -> Option<&[Acknowledgement]> {
        self.acks.as_deref()
    }
}
