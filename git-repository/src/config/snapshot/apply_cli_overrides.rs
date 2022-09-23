use std::convert::TryFrom;

use crate::{
    bstr::{BStr, BString, ByteSlice},
    config::SnapshotMut,
};

/// The error returned by [SnapshotMut::apply_cli_overrides()][crate::config::SnapshotMut::apply_cli_overrides()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("{input:?} is not a valid configuration key. Examples are 'core.abbrev' or 'remote.origin.url'")]
    InvalidKey { input: BString },
    #[error("Key {key:?} could not be parsed")]
    SectionKey {
        key: BString,
        source: git_config::parse::section::key::Error,
    },
    #[error(transparent)]
    SectionHeader(#[from] git_config::parse::section::header::Error),
}

impl SnapshotMut<'_> {
    /// Apply configuration values of the form `core.abbrev=5` or `remote.origin.url = foo` or `core.bool-implicit-true`
    /// to the repository configuration, marked with [source CLI][git_config::Source::Cli].
    pub fn apply_cli_overrides(
        &mut self,
        values: impl IntoIterator<Item = impl AsRef<BStr>>,
    ) -> Result<&mut Self, Error> {
        let mut file = git_config::File::new(git_config::file::Metadata::from(git_config::Source::Cli));
        for key_value in values {
            let key_value = key_value.as_ref();
            let mut tokens = key_value.splitn(2, |b| *b == b'=').map(|v| v.trim());
            let key = tokens.next().expect("always one value").as_bstr();
            let value = tokens.next();
            let key = git_config::parse::key(key.to_str().map_err(|_| Error::InvalidKey { input: key.into() })?)
                .ok_or_else(|| Error::InvalidKey { input: key.into() })?;
            let mut section = file.section_mut_or_create_new(key.section_name, key.subsection_name)?;
            section.push(
                git_config::parse::section::Key::try_from(key.value_name.to_owned()).map_err(|err| {
                    Error::SectionKey {
                        source: err,
                        key: key.value_name.into(),
                    }
                })?,
                value.map(|v| v.as_bstr()),
            );
        }
        self.config.append(file);
        Ok(self)
    }
}
