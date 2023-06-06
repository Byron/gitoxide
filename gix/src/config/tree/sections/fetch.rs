use crate::{
    config,
    config::tree::{keys, Fetch, Key, Section},
};

impl Fetch {
    /// The `fetch.negotiationAlgorithm` key.
    pub const NEGOTIATION_ALGORITHM: NegotiationAlgorithm = NegotiationAlgorithm::new_with_validate(
        "negotiationAlgorithm",
        &config::Tree::FETCH,
        validate::NegotiationAlgorithm,
    );
}

impl Section for Fetch {
    fn name(&self) -> &str {
        "fetch"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::NEGOTIATION_ALGORITHM]
    }
}

/// The `fetch.negotiationAlgorithm` key.
pub type NegotiationAlgorithm = keys::Any<validate::NegotiationAlgorithm>;

mod algorithm {
    use std::borrow::Cow;

    use gix_object::bstr::ByteSlice;

    use crate::{
        bstr::BStr,
        config::{key::GenericErrorWithValue, tree::sections::fetch::NegotiationAlgorithm},
        remote::fetch::negotiate,
    };

    impl NegotiationAlgorithm {
        /// Derive the negotiation algorithm identified by `name`, case-sensitively.
        pub fn try_into_negotiation_algorithm(
            &'static self,
            name: Cow<'_, BStr>,
        ) -> Result<negotiate::Algorithm, GenericErrorWithValue> {
            Ok(match name.as_ref().as_bytes() {
                b"noop" => negotiate::Algorithm::Noop,
                b"consecutive" | b"default" => negotiate::Algorithm::Consecutive,
                b"skipping" => negotiate::Algorithm::Skipping,
                _ => return Err(GenericErrorWithValue::from_value(self, name.into_owned())),
            })
        }
    }
}

mod validate {
    use crate::{
        bstr::BStr,
        config::tree::{keys, Fetch},
    };

    pub struct NegotiationAlgorithm;
    impl keys::Validate for NegotiationAlgorithm {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            Fetch::NEGOTIATION_ALGORITHM.try_into_negotiation_algorithm(value.into())?;
            Ok(())
        }
    }
}
