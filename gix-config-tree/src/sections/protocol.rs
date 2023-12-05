use crate::{keys, Key, Protocol, Section, Tree};

impl Protocol {
    /// The `protocol.allow` key.
    pub const ALLOW: Allow = Allow::new_with_validate("allow", &Tree::PROTOCOL, validate::Allow);
    /// The `protocol.version` key.
    pub const VERSION: Version = Version::new_with_validate("version", &Tree::PROTOCOL, validate::Version);

    /// The `protocol.<name>` subsection
    pub const NAME_PARAMETER: NameParameter = NameParameter;
}

// TODO Copied from gix/src/remote/url/scheme_permission.rs
pub mod scheme_permission {
    use std::borrow::Cow;

    use bstr::{BStr, BString, ByteSlice};

    // All allowed values of the `protocol.allow` key.
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum Allow {
        /// Allow use this protocol.
        Always,
        /// Forbid using this protocol
        Never,
        /// Only supported if the `GIT_PROTOCOL_FROM_USER` is unset or is set to `1`.
        User,
    }

    impl Allow {
        /// Return true if we represent something like 'allow == true'.
        pub fn to_bool(self, user_allowed: Option<bool>) -> bool {
            match self {
                Allow::Always => true,
                Allow::Never => false,
                Allow::User => user_allowed.unwrap_or(true),
            }
        }
    }

    impl<'a> TryFrom<Cow<'a, BStr>> for Allow {
        type Error = BString;

        fn try_from(v: Cow<'a, BStr>) -> Result<Self, Self::Error> {
            Ok(match v.as_ref().as_bytes() {
                b"never" => Allow::Never,
                b"always" => Allow::Always,
                b"user" => Allow::User,
                unknown => return Err(unknown.into()),
            })
        }
    }
}

/// The `protocol.allow` key type.
pub type Allow = keys::Any<validate::Allow>;

/// The `protocol.version` key.
pub type Version = keys::Any<validate::Version>;

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
mod allow {
    use std::borrow::Cow;
    use bstr::BStr;
    use crate::sections::protocol::{Allow, scheme_permission};

    impl Allow {
        /// Convert `value` into its respective `Allow` variant, possibly informing about the `scheme` we are looking at in the error.
        pub fn try_into_allow(
            &'static self,
            value: Cow<'_, BStr>,
            scheme: Option<&str>,
        ) -> Result<scheme_permission::Allow, crate::protocol::allow::Error> {
            scheme_permission::Allow::try_from(value).map_err(|value| crate::protocol::allow::Error {
                value,
                scheme: scheme.map(ToOwned::to_owned),
            })
        }
    }
}

/// The `protocol.<name>` parameter section.
pub struct NameParameter;

impl NameParameter {
    /// The `protocol.<name>.allow` key.
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
        Some(&Tree::PROTOCOL)
    }
}

impl Section for Protocol {
    fn name(&self) -> &str {
        "protocol"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::ALLOW, &Self::VERSION]
    }

    fn sub_sections(&self) -> &[&dyn Section] {
        &[&Self::NAME_PARAMETER]
    }
}

mod key_impls {
    impl super::Version {
        /// Convert `value` into the corresponding protocol version, possibly applying the correct default.
        #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
        pub fn try_into_protocol_version(
            &'static self,
            value: Option<Result<i64, gix_config_value::Error>>,
        ) -> Result<gix_transport::Protocol, crate::key::GenericErrorWithValue> {
            let value = match value {
                None => return Ok(gix_transport::Protocol::V2),
                Some(v) => v,
            };
            Ok(match value {
                Ok(0) => gix_transport::Protocol::V0,
                Ok(1) => gix_transport::Protocol::V1,
                Ok(2) => gix_transport::Protocol::V2,
                Ok(other) => {
                    return Err(crate::key::GenericErrorWithValue::from_value(
                        self,
                        other.to_string().into(),
                    ))
                }
                Err(err) => {
                    return Err(
                        crate::key::GenericErrorWithValue::from_value(self, "unknown".into()).with_source(err),
                    )
                }
            })
        }
    }
}

mod validate {
    use bstr::BStr;
    use crate::keys;

    pub struct Allow;
    impl keys::Validate for Allow {
        fn validate(&self, _value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
            super::Protocol::ALLOW.try_into_allow(std::borrow::Cow::Borrowed(_value), None)?;
            Ok(())
        }
    }

    pub struct Version;
    impl keys::Validate for Version {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            let value = gix_config_value::Integer::try_from(value)?
                .to_decimal()
                .ok_or_else(|| format!("integer {value} cannot be represented as integer"))?;
            match value {
                0..=2 => Ok(()),
                _ => Err(format!("protocol version {value} is unknown").into()),
            }
        }
    }
}
