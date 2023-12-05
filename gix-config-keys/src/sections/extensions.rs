use crate::{keys, Extensions, Key, Section, Tree};

impl Extensions {
    /// The `extensions.worktreeConfig` key.
    pub const WORKTREE_CONFIG: keys::Boolean = keys::Boolean::new_boolean("worktreeConfig", &Tree::EXTENSIONS);
    /// The `extensions.objectFormat` key.
    pub const OBJECT_FORMAT: ObjectFormat =
        ObjectFormat::new_with_validate("objectFormat", &Tree::EXTENSIONS, validate::ObjectFormat).with_note(
            "Support for SHA256 is prepared but not fully implemented yet. For now we abort when encountered",
        );
}

/// The `core.checkStat` key.
pub type ObjectFormat = keys::Any<validate::ObjectFormat>;

mod object_format {
    use std::borrow::Cow;
    use bstr::BStr;
    use crate::sections::extensions::ObjectFormat;

    impl ObjectFormat {
        pub fn try_into_object_format(
            &'static self,
            value: Cow<'_, BStr>,
        ) -> Result<gix_hash::Kind, crate::key::GenericErrorWithValue> {
            if value.as_ref().eq_ignore_ascii_case(b"sha1") {
                Ok(gix_hash::Kind::Sha1)
            } else {
                Err(crate::key::GenericErrorWithValue::from_value(self, value.into_owned()))
            }
        }
    }
}

impl Section for Extensions {
    fn name(&self) -> &str {
        "extensions"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::OBJECT_FORMAT, &Self::WORKTREE_CONFIG]
    }
}

mod validate {
    use bstr::BStr;
    use crate::keys;

    pub struct ObjectFormat;

    impl keys::Validate for ObjectFormat {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            super::Extensions::OBJECT_FORMAT.try_into_object_format(value.into())?;
            Ok(())
        }
    }
}
