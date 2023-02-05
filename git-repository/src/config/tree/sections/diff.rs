use crate::{
    config,
    config::tree::{keys, Diff, Key, Section},
};

impl Diff {
    /// The `diff.algorithm` key.
    pub const ALGORITHM: Algorithm = Algorithm::new_with_validate("algorithm", &config::Tree::DIFF, validate::Algorithm)
                                        .with_deviation("'patience' diff is not implemented and can default to 'histogram' if lenient config is used, and defaults to histogram if unset for fastest and best results");
}

impl Section for Diff {
    fn name(&self) -> &str {
        "diff"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::ALGORITHM]
    }
}

/// The `diff.algorithm` key.
pub type Algorithm = keys::Any<validate::Algorithm>;

mod algorithm {
    use std::borrow::Cow;

    use crate::{
        bstr::BStr,
        config,
        config::{diff::algorithm::Error, tree::sections::diff::Algorithm},
    };

    impl Algorithm {
        /// Derive the diff algorithm identified by `name`, case-insensitively.
        pub fn try_into_algorithm(&self, name: Cow<'_, BStr>) -> Result<git_diff::blob::Algorithm, Error> {
            let algo = if name.eq_ignore_ascii_case(b"myers") || name.eq_ignore_ascii_case(b"default") {
                git_diff::blob::Algorithm::Myers
            } else if name.eq_ignore_ascii_case(b"minimal") {
                git_diff::blob::Algorithm::MyersMinimal
            } else if name.eq_ignore_ascii_case(b"histogram") {
                git_diff::blob::Algorithm::Histogram
            } else if name.eq_ignore_ascii_case(b"patience") {
                return Err(config::diff::algorithm::Error::Unimplemented {
                    name: name.into_owned(),
                });
            } else {
                return Err(Error::Unknown {
                    name: name.into_owned(),
                });
            };
            Ok(algo)
        }
    }
}

mod validate {
    use crate::{
        bstr::BStr,
        config::tree::{keys, Diff},
    };

    pub struct Algorithm;
    impl keys::Validate for Algorithm {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            Diff::ALGORITHM.try_into_algorithm(value.into())?;
            Ok(())
        }
    }
}
