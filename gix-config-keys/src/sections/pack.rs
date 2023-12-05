use crate::{keys, Key, Pack, Section, Tree};

impl Pack {
    /// The `pack.threads` key.
    pub const THREADS: keys::UnsignedInteger =
        keys::UnsignedInteger::new_unsigned_integer("threads", &Tree::PACK)
            .with_deviation("Leaving this key unspecified uses all available cores, instead of 1");

    /// The `pack.indexVersion` key.
    pub const INDEX_VERSION: IndexVersion =
        IndexVersion::new_with_validate("indexVersion", &Tree::PACK, validate::IndexVersion);
}

/// The `pack.indexVersion` key.
pub type IndexVersion = keys::Any<validate::IndexVersion>;

mod index_version {
    use crate::sections::pack::IndexVersion;

    impl IndexVersion {
        /// Try to interpret an integer value as index version.
        pub fn try_into_index_version(
            &'static self,
            value: Result<i64, gix_config_value::Error>,
        ) -> Result<gix_pack::index::Version, crate::key::GenericError> {
            let value = value.map_err(|err| crate::key::GenericError::from(self).with_source(err))?;
            Ok(match value {
                1 => gix_pack::index::Version::V1,
                2 => gix_pack::index::Version::V2,
                _ => return Err(crate::key::GenericError::from(self)),
            })
        }
    }
}

impl Section for Pack {
    fn name(&self) -> &str {
        "pack"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::THREADS, &Self::INDEX_VERSION]
    }
}

mod validate {
    use bstr::BStr;
    use crate::keys;

    pub struct IndexVersion;
    impl keys::Validate for IndexVersion {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            super::Pack::INDEX_VERSION.try_into_index_version(gix_config_value::Integer::try_from(value).and_then(
                |int| {
                    int.to_decimal()
                        .ok_or_else(|| gix_config_value::Error::new("integer out of range", value))
                },
            ))?;
            Ok(())
        }
    }
}
