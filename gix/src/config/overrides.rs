use std::convert::TryFrom;

use crate::bstr::{BStr, BString, ByteSlice};

/// The error returned by [`SnapshotMut::apply_cli_overrides()`][crate::config::SnapshotMut::append_config()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("{input:?} is not a valid configuration key. Examples are 'core.abbrev' or 'remote.origin.url'")]
    InvalidKey { input: BString },
    #[error("Key {key:?} could not be parsed")]
    SectionKey {
        key: BString,
        source: gix_config::parse::section::key::Error,
    },
    #[error(transparent)]
    SectionHeader(#[from] gix_config::parse::section::header::Error),
}

pub(crate) fn append(
    config: &mut gix_config::File<'static>,
    values: impl IntoIterator<Item = impl AsRef<BStr>>,
    source: gix_config::Source,
    mut make_comment: impl FnMut(&BStr) -> Option<BString>,
) -> Result<(), Error> {
    let mut file = gix_config::File::new(gix_config::file::Metadata::from(source));
    for key_value in values {
        let key_value = key_value.as_ref();
        let mut tokens = key_value.splitn(2, |b| *b == b'=').map(ByteSlice::trim);
        let key = tokens.next().expect("always one value").as_bstr();
        let value = tokens.next();
        let key = gix_config::parse::key(
            key.to_str()
                .map_err(|_| Error::InvalidKey { input: key.into() })?
                .into(),
        )
        .ok_or_else(|| Error::InvalidKey { input: key.into() })?;
        let mut section = file.section_mut_or_create_new(key.section_name, key.subsection_name)?;
        let key =
            gix_config::parse::section::Key::try_from(key.value_name.to_owned()).map_err(|err| Error::SectionKey {
                source: err,
                key: key.value_name.into(),
            })?;
        let comment = make_comment(key_value);
        let value = value.map(ByteSlice::as_bstr);
        match comment {
            Some(comment) => section.push_with_comment(key, value, &**comment),
            None => section.push(key, value),
        };
    }
    config.append(file);
    Ok(())
}
