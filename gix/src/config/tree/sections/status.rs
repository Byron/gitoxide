use crate::config;
use crate::config::tree::sections::Status;
use crate::config::tree::{keys, Key, Section};

impl Status {
    /// The `status.showUntrackedFiles` key
    pub const SHOW_UNTRACKED_FILES: ShowUntrackedFiles = ShowUntrackedFiles::new_with_validate(
        "showUntrackedFiles",
        &config::Tree::STATUS,
        validate::ShowUntrackedFiles,
    );
}

/// The `status.showUntrackedFiles` key.
pub type ShowUntrackedFiles = keys::Any<validate::ShowUntrackedFiles>;

mod show_untracked_files {
    use std::borrow::Cow;

    use crate::{bstr::BStr, config, config::tree::status::ShowUntrackedFiles, status};

    impl ShowUntrackedFiles {
        pub fn try_into_show_untracked_files(
            &'static self,
            value: Cow<'_, BStr>,
        ) -> Result<status::UntrackedFiles, config::key::GenericErrorWithValue> {
            use crate::bstr::ByteSlice;
            Ok(match value.as_ref().as_bytes() {
                b"no" => status::UntrackedFiles::None,
                b"normal" => status::UntrackedFiles::Collapsed,
                b"all" => status::UntrackedFiles::Files,
                _ => return Err(config::key::GenericErrorWithValue::from_value(self, value.into_owned())),
            })
        }
    }
}

impl Section for Status {
    fn name(&self) -> &str {
        "status"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::SHOW_UNTRACKED_FILES]
    }
}

mod validate {
    use crate::{bstr::BStr, config::tree::keys};

    pub struct ShowUntrackedFiles;
    impl keys::Validate for ShowUntrackedFiles {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            super::Status::SHOW_UNTRACKED_FILES.try_into_show_untracked_files(value.into())?;
            Ok(())
        }
    }
}
