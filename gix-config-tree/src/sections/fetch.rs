use crate::{keys, Fetch, Key, Section, Tree};

impl Fetch {
    /// The `fetch.negotiationAlgorithm` key.
    pub const NEGOTIATION_ALGORITHM: NegotiationAlgorithm = NegotiationAlgorithm::new_with_validate(
        "negotiationAlgorithm",
        &Tree::FETCH,
        validate::NegotiationAlgorithm,
    );
    /// The `fetch.recurseSubmodules` key.
    #[cfg(feature = "attributes")]
    pub const RECURSE_SUBMODULES: RecurseSubmodules =
        RecurseSubmodules::new_with_validate("recurseSubmodules", &Tree::FETCH, validate::RecurseSubmodules);
}

impl Section for Fetch {
    fn name(&self) -> &str {
        "fetch"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[
            &Self::NEGOTIATION_ALGORITHM,
            #[cfg(feature = "attributes")]
            &Self::RECURSE_SUBMODULES,
        ]
    }
}

/// The `fetch.negotiationAlgorithm` key.
pub type NegotiationAlgorithm = keys::Any<validate::NegotiationAlgorithm>;

/// The `fetch.recurseSubmodules` key.
#[cfg(feature = "attributes")]
pub type RecurseSubmodules = keys::Any<validate::RecurseSubmodules>;

mod algorithm {
    #[cfg(feature = "credentials")]
    impl crate::config::sections::fetch::NegotiationAlgorithm {
        /// Derive the negotiation algorithm identified by `name`, case-sensitively.
        pub fn try_into_negotiation_algorithm(
            &'static self,
            name: std::borrow::Cow<'_, crate::bstr::BStr>,
        ) -> Result<crate::remote::fetch::negotiate::Algorithm, crate::config::key::GenericErrorWithValue> {
            use crate::bstr::ByteSlice;
            use crate::remote::fetch::negotiate::Algorithm;

            Ok(match name.as_ref().as_bytes() {
                b"noop" => Algorithm::Noop,
                b"consecutive" | b"default" => Algorithm::Consecutive,
                b"skipping" => Algorithm::Skipping,
                _ => {
                    return Err(crate::config::key::GenericErrorWithValue::from_value(
                        self,
                        name.into_owned(),
                    ))
                }
            })
        }
    }

    #[cfg(feature = "attributes")]
    impl crate::sections::fetch::RecurseSubmodules {
        /// Obtain the way submodules should be updated.
        pub fn try_into_recurse_submodules(
            &'static self,
            value: Result<bool, gix_config::value::Error>,
        ) -> Result<gix_submodule::config::FetchRecurse, crate::key::GenericErrorWithValue> {
            gix_submodule::config::FetchRecurse::new(value)
                .map_err(|err| crate::key::GenericErrorWithValue::from_value(self, err))
        }
    }
}

mod validate {
    use bstr::BStr;
    use crate::keys;

    pub struct NegotiationAlgorithm;
    impl keys::Validate for NegotiationAlgorithm {
        #[cfg_attr(not(feature = "credentials"), allow(unused_variables))]
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            #[cfg(feature = "credentials")]
            crate::config::Fetch::NEGOTIATION_ALGORITHM.try_into_negotiation_algorithm(value.into())?;
            Ok(())
        }
    }

    pub struct RecurseSubmodules;
    impl keys::Validate for RecurseSubmodules {
        #[cfg_attr(not(feature = "attributes"), allow(unused_variables))]
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            #[cfg(feature = "attributes")]
            {
                let boolean = gix_config::Boolean::try_from(value).map(|b| b.0);
                crate::Fetch::RECURSE_SUBMODULES.try_into_recurse_submodules(boolean)?;
            }
            Ok(())
        }
    }
}
