use std::convert::TryFrom;

use super::Error;
use crate::{bstr::ByteSlice, revision::spec::parse::ObjectKindHint};

pub(crate) fn interpolate_context<'a>(
    git_install_dir: Option<&'a std::path::Path>,
    home_dir: Option<&'a std::path::Path>,
) -> git_config::path::interpolate::Context<'a> {
    git_config::path::interpolate::Context {
        git_install_dir,
        home_dir,
        home_for_user: Some(git_config::path::interpolate::home_for_user), // TODO: figure out how to configure this
    }
}

pub(crate) fn base_options(lossy: Option<bool>) -> git_config::file::init::Options<'static> {
    git_config::file::init::Options {
        lossy: lossy.unwrap_or(!cfg!(debug_assertions)),
        ..Default::default()
    }
}

pub(crate) fn config_bool(
    config: &git_config::File<'_>,
    key: &str,
    default: bool,
    lenient: bool,
) -> Result<bool, Error> {
    let (section, key) = key.split_once('.').expect("valid section.key format");
    config
        .boolean(section, None, key)
        .unwrap_or(Ok(default))
        .map_err(|err| Error::DecodeBoolean {
            value: err.input,
            key: key.into(),
        })
        .with_lenient_default(lenient)
}

pub(crate) fn query_refupdates(
    config: &git_config::File<'static>,
    lenient_config: bool,
) -> Result<Option<git_ref::store::WriteReflog>, Error> {
    match config
        .boolean("core", None, "logAllRefUpdates")
        .and_then(|b| b.ok())
        .map(|b| {
            b.then(|| git_ref::store::WriteReflog::Normal)
                .unwrap_or(git_ref::store::WriteReflog::Disable)
        }) {
        Some(val) => Ok(Some(val)),
        None => match config.string("core", None, "logAllRefUpdates") {
            Some(val) if val.eq_ignore_ascii_case(b"always") => Ok(Some(git_ref::store::WriteReflog::Always)),
            Some(_val) if lenient_config => Ok(None),
            Some(val) => Err(Error::LogAllRefUpdates {
                value: val.into_owned(),
            }),
            None => Ok(None),
        },
    }
}

// TODO: Use a specialization here once trait specialization is stabilized. Would be perfect here for `T: Default`.
pub trait ApplyLeniency {
    fn with_leniency(self, is_lenient: bool) -> Self;
}

pub trait ApplyLeniencyDefault {
    fn with_lenient_default(self, is_lenient: bool) -> Self;
}

impl<T, E> ApplyLeniency for Result<Option<T>, E> {
    fn with_leniency(self, is_lenient: bool) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(_) if is_lenient => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl<T, E> ApplyLeniencyDefault for Result<T, E>
where
    T: Default,
{
    fn with_lenient_default(self, is_lenient: bool) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(_) if is_lenient => Ok(T::default()),
            Err(err) => Err(err),
        }
    }
}

pub(crate) fn reflog_or_default(
    config_reflog: Option<git_ref::store::WriteReflog>,
    has_worktree: bool,
) -> git_ref::store::WriteReflog {
    config_reflog.unwrap_or_else(|| {
        has_worktree
            .then(|| git_ref::store::WriteReflog::Normal)
            .unwrap_or(git_ref::store::WriteReflog::Disable)
    })
}

/// Return `(pack_cache_bytes, object_cache_bytes)` as parsed from git-config
pub(crate) fn parse_object_caches(
    config: &git_config::File<'static>,
    lenient: bool,
    mut filter_config_section: fn(&git_config::file::Metadata) -> bool,
) -> Result<(Option<usize>, usize), Error> {
    let key = "core.deltaBaseCacheLimit";
    let pack_cache_bytes = config
        .integer_filter_by_key(key, &mut filter_config_section)
        .transpose()
        .with_leniency(lenient)
        .map_err(|err| Error::Value { source: err, key })?;
    let key = "gitoxide.objects.cacheLimit";
    let object_cache_bytes = config
        .integer_filter_by_key(key, &mut filter_config_section)
        .transpose()
        .with_leniency(lenient)
        .map_err(|err| Error::Value { source: err, key })?
        .unwrap_or_default();
    Ok((
        pack_cache_bytes.and_then(|v| v.try_into().ok()),
        object_cache_bytes.try_into().unwrap_or_default(),
    ))
}

pub(crate) fn parse_core_abbrev(
    config: &git_config::File<'static>,
    object_hash: git_hash::Kind,
) -> Result<Option<usize>, Error> {
    match config.string("core", None, "abbrev") {
        Some(hex_len_str) => {
            if hex_len_str.trim().is_empty() {
                return Err(Error::EmptyValue { key: "core.abbrev" });
            }
            if hex_len_str.trim().eq_ignore_ascii_case(b"auto") {
                Ok(None)
            } else {
                let value_bytes = hex_len_str.as_ref();
                if let Ok(false) = git_config::Boolean::try_from(value_bytes).map(Into::into) {
                    Ok(object_hash.len_in_hex().into())
                } else {
                    let value = git_config::Integer::try_from(value_bytes)
                        .map_err(|_| Error::CoreAbbrev {
                            value: hex_len_str.clone().into_owned(),
                            max: object_hash.len_in_hex() as u8,
                        })?
                        .to_decimal()
                        .ok_or_else(|| Error::CoreAbbrev {
                            value: hex_len_str.clone().into_owned(),
                            max: object_hash.len_in_hex() as u8,
                        })?;
                    if value < 4 || value as usize > object_hash.len_in_hex() {
                        return Err(Error::CoreAbbrev {
                            value: hex_len_str.clone().into_owned(),
                            max: object_hash.len_in_hex() as u8,
                        });
                    }
                    Ok(Some(value as usize))
                }
            }
        }
        None => Ok(None),
    }
}

pub(crate) fn disambiguate_hint(config: &git_config::File<'static>) -> Option<ObjectKindHint> {
    config.string("core", None, "disambiguate").and_then(|value| {
        Some(match value.as_ref().as_ref() {
            b"commit" => ObjectKindHint::Commit,
            b"committish" => ObjectKindHint::Committish,
            b"tree" => ObjectKindHint::Tree,
            b"treeish" => ObjectKindHint::Treeish,
            b"blob" => ObjectKindHint::Blob,
            _ => return None,
        })
    })
}
