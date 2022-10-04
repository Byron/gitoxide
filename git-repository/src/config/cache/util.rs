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
    match config
        .boolean(section, None, key)
        .unwrap_or(Ok(default))
        .map_err(|err| Error::DecodeBoolean {
            value: err.input,
            key: key.into(),
        }) {
        Ok(v) => Ok(v),
        Err(_err) if lenient => Ok(default),
        Err(err) => Err(err),
    }
}

pub(crate) fn query_refupdates(config: &git_config::File<'static>) -> Option<git_ref::store::WriteReflog> {
    config.string("core", None, "logallrefupdates").map(|val| {
        (val.eq_ignore_ascii_case(b"always"))
            .then(|| git_ref::store::WriteReflog::Always)
            .or_else(|| {
                git_config::Boolean::try_from(val)
                    .ok()
                    .and_then(|b| b.is_true().then(|| git_ref::store::WriteReflog::Normal))
            })
            .unwrap_or(git_ref::store::WriteReflog::Disable)
    })
}

pub(crate) fn check_lenient<T, E>(v: Result<Option<T>, E>, lenient: bool) -> Result<Option<T>, E> {
    match v {
        Ok(v) => Ok(v),
        Err(_) if lenient => Ok(None),
        Err(err) => Err(err),
    }
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
