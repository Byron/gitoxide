use crate::{
    config,
    config::tree::{keys, Key, Protocol, Section},
};

impl Protocol {
    /// The `protocol.allow` key.
    pub const ALLOW: Allow = Allow::new_with_validate("allow", &config::Tree::PROTOCOL, validate::Allow);

    /// The `protocol.<name>` subsection
    pub const NAME_PARAMETER: NameParameter = NameParameter;
}

/// The `protocol.allow` key type.
pub type Allow = keys::Any<validate::Allow>;

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
mod allow {
    use std::borrow::Cow;

    use crate::{bstr::BStr, config, config::tree::protocol::Allow, remote::url::scheme_permission};

    impl Allow {
        /// Convert `value` into its respective `Allow` variant, possibly informing about the `scheme` we are looking at in the error.
        pub fn try_into_allow(
            &'static self,
            value: Cow<'_, BStr>,
            scheme: Option<&str>,
        ) -> Result<scheme_permission::Allow, config::protocol::allow::Error> {
            scheme_permission::Allow::try_from(value).map_err(|value| config::protocol::allow::Error {
                value,
                scheme: scheme.map(ToOwned::to_owned),
            })
        }
    }
}

/// The `protocol.<name>` parameter section.
pub struct NameParameter;

impl NameParameter {
    /// The `credential.<url>.helper` key.
    pub const ALLOW: Allow = Allow::new_with_validate("allow", &Protocol::NAME_PARAMETER, validate::Allow);
}

impl Section for NameParameter {
    fn name(&self) -> &str {
        "<name>"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::ALLOW]
    }

    fn parent(&self) -> Option<&dyn Section> {
        Some(&config::Tree::PROTOCOL)
    }
}

impl Section for Protocol {
    fn name(&self) -> &str {
        "protocol"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::ALLOW]
    }

    fn sub_sections(&self) -> &[&dyn Section] {
        &[&Self::NAME_PARAMETER]
    }
}

mod validate {
    use crate::{bstr::BStr, config::tree::keys};

    pub struct Allow;
    impl keys::Validate for Allow {
        fn validate(&self, _value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
            super::Protocol::ALLOW.try_into_allow(std::borrow::Cow::Borrowed(_value), None)?;
            Ok(())
        }
    }
}
