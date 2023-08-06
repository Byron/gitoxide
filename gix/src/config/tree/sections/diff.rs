use crate::{
    config,
    config::tree::{keys, Diff, Key, Section},
};

impl Diff {
    /// The `diff.algorithm` key.
    pub const ALGORITHM: Algorithm = Algorithm::new_with_validate("algorithm", &config::Tree::DIFF, validate::Algorithm)
                                        .with_deviation("'patience' diff is not implemented and can default to 'histogram' if lenient config is used, and defaults to histogram if unset for fastest and best results");
    /// The `diff.renameLimit` key.
    pub const RENAME_LIMIT: keys::UnsignedInteger = keys::UnsignedInteger::new_unsigned_integer(
        "renameLimit",
        &config::Tree::DIFF,
    )
    .with_note(
        "The limit is actually squared, so 1000 stands for up to 1 million diffs if fuzzy rename tracking is enabled",
    );
    /// The `diff.renames` key.
    pub const RENAMES: Renames = Renames::new_renames("renames", &config::Tree::DIFF);
}

impl Section for Diff {
    fn name(&self) -> &str {
        "diff"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::ALGORITHM, &Self::RENAME_LIMIT, &Self::RENAMES]
    }
}

/// The `diff.algorithm` key.
pub type Algorithm = keys::Any<validate::Algorithm>;

/// The `diff.renames` key.
pub type Renames = keys::Any<validate::Renames>;

mod algorithm {
    use std::borrow::Cow;

    use crate::{
        bstr::BStr,
        config,
        config::{diff::algorithm::Error, tree::sections::diff::Algorithm},
    };

    impl Algorithm {
        /// Derive the diff algorithm identified by `name`, case-insensitively.
        pub fn try_into_algorithm(&self, name: Cow<'_, BStr>) -> Result<gix_diff::blob::Algorithm, Error> {
            let algo = if name.eq_ignore_ascii_case(b"myers") || name.eq_ignore_ascii_case(b"default") {
                gix_diff::blob::Algorithm::Myers
            } else if name.eq_ignore_ascii_case(b"minimal") {
                gix_diff::blob::Algorithm::MyersMinimal
            } else if name.eq_ignore_ascii_case(b"histogram") {
                gix_diff::blob::Algorithm::Histogram
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

mod renames {
    use crate::{
        bstr::ByteSlice,
        config::{
            key::GenericError,
            tree::{keys, sections::diff::Renames, Section},
        },
        diff::rename::Tracking,
    };

    impl Renames {
        /// Create a new instance.
        pub const fn new_renames(name: &'static str, section: &'static dyn Section) -> Self {
            keys::Any::new_with_validate(name, section, super::validate::Renames)
        }
        /// Try to convert the configuration into a valid rename tracking variant. Use `value` and if it's an error, interpret
        /// the boolean as string
        pub fn try_into_renames(
            &'static self,
            value: Result<bool, gix_config::value::Error>,
        ) -> Result<Tracking, GenericError> {
            Ok(match value {
                Ok(true) => Tracking::Renames,
                Ok(false) => Tracking::Disabled,
                Err(err) => {
                    let value = &err.input;
                    match value.as_bytes() {
                        b"copy" | b"copies" => Tracking::RenamesAndCopies,
                        _ => return Err(GenericError::from_value(self, value.clone()).with_source(err)),
                    }
                }
            })
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

    pub struct Renames;
    impl keys::Validate for Renames {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            let boolean = gix_config::Boolean::try_from(value).map(|b| b.0);
            Diff::RENAMES.try_into_renames(boolean)?;
            Ok(())
        }
    }
}
