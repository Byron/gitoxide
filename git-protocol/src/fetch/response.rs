use crate::fetch::{self, command::Feature};
use bstr::BString;
use git_object::owned;
use git_transport::{client, Protocol};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
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
    pub fn check_required_features(features: &[Feature]) -> Result<(), fetch::Error> {
        let has = |name: &str| features.iter().any(|f| f.0 == name);
        // Let's focus on V2 standards, and simply not support old servers to keep our code simpler
        if !has("multi_ack_detailed") {
            return Err(fetch::Error::MissingServerCapability("multi_ack_detailed"));
        }
        // It's easy to NOT do sideband for us, but then again, everyone supports it.
        if !has("side-band") && !has("side-band-64k") {
            return Err(fetch::Error::MissingServerCapability("side-band OR side-band-64k"));
        }
        Ok(())
    }
    pub fn from_line_reader(
        _version: Protocol,
        _reader: Box<dyn client::ExtendedBufRead + '_>,
    ) -> Result<Response, Error> {
        unimplemented!("from line reader")
    }

    pub fn acknowledgements(&self) -> Option<&[Acknowledgement]> {
        self.acks.as_deref()
    }
}
