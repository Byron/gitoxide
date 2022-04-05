use crate::bstr::BString;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not open repository conifguration file")]
    Open(#[from] git_config::parser::ParserOrIoError<'static>),
    #[error("Cannot handle objects formatted as {:?}", .name)]
    UnsupportedObjectFormat { name: crate::bstr::BString },
    #[error("The value for '{}' cannot be empty", .key)]
    EmptyValue { key: &'static str },
    #[error("Invalid value for 'core.abbrev' = '{}'. It must be between 4 and {}", .value, .max)]
    CoreAbbrev { value: BString, max: u8 },
}

/// Utility type to keep pre-obtained configuration values.
#[derive(Debug, Clone)]
pub(crate) struct Cache {
    pub resolved: crate::Config,
    /// The hex-length to assume when shortening object ids. If `None`, it should be computed based on the approximate object count.
    pub hex_len: Option<usize>,
    /// true if the repository is designated as 'bare', without work tree
    pub is_bare: bool,
    /// The type of hash to use
    pub object_hash: git_hash::Kind,
    /// If true, multi-pack indices, whether present or not, may be used by the object database.
    pub use_multi_pack_index: bool,
}

mod cache {
    use super::{Cache, Error};
    use crate::bstr::ByteSlice;
    use git_config::file::GitConfig;
    use git_config::values;
    use git_config::values::{Boolean, Integer};
    use std::borrow::Cow;
    use std::convert::TryFrom;

    impl Cache {
        pub fn new(git_dir: &std::path::Path) -> Result<Self, Error> {
            let config = GitConfig::open(git_dir.join("config"))?;
            let is_bare = config_bool(&config, "core.bare", false);
            let use_multi_pack_index = config_bool(&config, "core.multiPackIndex", true);
            let repo_format_version = config
                .value::<Integer>("core", None, "repositoryFormatVersion")
                .map_or(0, |v| v.value);
            let object_hash = if repo_format_version == 1 {
                if let Ok(format) = config.value::<Cow<'_, [u8]>>("extensions", None, "objectFormat") {
                    match format.as_ref() {
                        b"sha1" => git_hash::Kind::Sha1,
                        _ => {
                            return Err(Error::UnsupportedObjectFormat {
                                name: format.to_vec().into(),
                            })
                        }
                    }
                } else {
                    git_hash::Kind::Sha1
                }
            } else {
                git_hash::Kind::Sha1
            };

            let mut hex_len = None;
            if let Ok(hex_len_str) = config.value::<values::String<'_>>("core", None, "abbrev") {
                if hex_len_str.value.trim().len() == 0 {
                    return Err(Error::EmptyValue { key: "core.abbrev" });
                }
                if hex_len_str.value.as_ref() != "auto" {
                    let value_bytes = hex_len_str.value.as_ref().as_ref();
                    if let Ok(Boolean::False(_)) = Boolean::try_from(value_bytes) {
                        hex_len = object_hash.len_in_hex().into();
                    } else {
                        // TODO: let it resolve the suffix
                        let value = Integer::try_from(value_bytes)
                            .map_err(|_| Error::CoreAbbrev {
                                value: hex_len_str.value.clone().into_owned(),
                                max: object_hash.len_in_hex() as u8,
                            })?
                            .value;
                        if value < 4 || value as usize > object_hash.len_in_hex() {
                            return Err(Error::CoreAbbrev {
                                value: hex_len_str.value.clone().into_owned(),
                                max: object_hash.len_in_hex() as u8,
                            });
                        }
                        hex_len = Some(value as usize);
                    }
                }
            }

            Ok(Cache {
                resolved: config.into(),
                use_multi_pack_index,
                object_hash,
                is_bare,
                hex_len,
            })
        }
    }

    fn config_bool(config: &GitConfig<'_>, key: &str, default: bool) -> bool {
        let (section, key) = key.split_once('.').expect("valid section.key format");
        config
            .value::<Boolean<'_>>(section, None, key)
            .map_or(default, |b| b.to_bool())
    }
}
