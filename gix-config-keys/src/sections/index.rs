use crate::{keys, Index, Key, Section, Tree};

impl Index {
    /// The `index.threads` key.
    pub const THREADS: IndexThreads =
        IndexThreads::new_with_validate("threads", &Tree::INDEX, validate::IndexThreads);
    /// The `index.skipHash` key.
    pub const SKIP_HASH: keys::Boolean = keys::Boolean::new_boolean("skipHash", &Tree::INDEX)
        .with_deviation("also used to skip the hash when reading, even if a hash exists in the index file");
}

/// The `index.threads` key.
pub type IndexThreads = keys::Any<validate::IndexThreads>;

mod index_threads {
    use std::borrow::Cow;
    use bstr::BStr;
    use crate::{key::GenericErrorWithValue, sections::index::IndexThreads};

    impl IndexThreads {
        /// Parse `value` into the amount of threads to use, with `1` being single-threaded, or `0` indicating
        /// to select the amount of threads, with any other number being the specific amount of threads to use.
        pub fn try_into_index_threads(
            &'static self,
            value: Cow<'_, BStr>,
        ) -> Result<usize, GenericErrorWithValue> {
            gix_config_value::Integer::try_from(value.as_ref())
                .ok()
                .and_then(|i| i.to_decimal().and_then(|i| i.try_into().ok()))
                .or_else(|| {
                    gix_config_value::Boolean::try_from(value.as_ref())
                        .ok()
                        .map(|b| if b.0 { 0 } else { 1 })
                })
                .ok_or_else(|| GenericErrorWithValue::from_value(self, value.into_owned()))
        }
    }
}

impl Section for Index {
    fn name(&self) -> &str {
        "index"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::THREADS, &Self::SKIP_HASH]
    }
}

mod validate {
    use bstr::BStr;
    use crate::keys;

    pub struct IndexThreads;
    impl keys::Validate for IndexThreads {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            super::Index::THREADS.try_into_index_threads(value.into())?;
            Ok(())
        }
    }
}
