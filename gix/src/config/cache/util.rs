#![allow(clippy::result_large_err)]
use super::Error;
use crate::{
    config,
    config::tree::{gitoxide, Core},
};

pub(crate) fn interpolate_context<'a>(
    git_install_dir: Option<&'a std::path::Path>,
    home_dir: Option<&'a std::path::Path>,
) -> gix_config::path::interpolate::Context<'a> {
    gix_config::path::interpolate::Context {
        git_install_dir,
        home_dir,
        home_for_user: Some(gix_config::path::interpolate::home_for_user), // TODO: figure out how to configure this
    }
}

pub(crate) fn base_options(lossy: Option<bool>, lenient: bool) -> gix_config::file::init::Options<'static> {
    gix_config::file::init::Options {
        lossy: lossy.unwrap_or(!cfg!(debug_assertions)),
        ignore_io_errors: lenient,
        ..Default::default()
    }
}

pub(crate) fn config_bool(
    config: &gix_config::File<'_>,
    key: &'static config::tree::keys::Boolean,
    key_str: &str,
    default: bool,
    lenient: bool,
) -> Result<bool, Error> {
    use config::tree::Key;
    debug_assert_eq!(
        key_str,
        key.logical_name(),
        "BUG: key name and hardcoded name must match"
    );
    config
        .boolean_by_key(key_str)
        .map_or(Ok(default), |res| key.enrich_error(res))
        .map_err(Error::from)
        .with_lenient_default(lenient)
}

pub(crate) fn query_refupdates(
    config: &gix_config::File<'static>,
    lenient_config: bool,
) -> Result<Option<gix_ref::store::WriteReflog>, Error> {
    let key = "core.logAllRefUpdates";
    Core::LOG_ALL_REF_UPDATES
        .try_into_ref_updates(config.boolean_by_key(key))
        .with_leniency(lenient_config)
        .map_err(Into::into)
}

pub(crate) fn reflog_or_default(
    config_reflog: Option<gix_ref::store::WriteReflog>,
    has_worktree: bool,
) -> gix_ref::store::WriteReflog {
    config_reflog.unwrap_or(if has_worktree {
        gix_ref::store::WriteReflog::Normal
    } else {
        gix_ref::store::WriteReflog::Disable
    })
}

/// Return `(pack_cache_bytes, object_cache_bytes)` as parsed from gix-config
pub(crate) fn parse_object_caches(
    config: &gix_config::File<'static>,
    lenient: bool,
    mut filter_config_section: fn(&gix_config::file::Metadata) -> bool,
) -> Result<(Option<usize>, Option<usize>, usize), Error> {
    let static_pack_cache_limit = config
        .integer_filter_by_key("gitoxide.core.deltaBaseCacheLimit", &mut filter_config_section)
        .map(|res| gitoxide::Core::DEFAULT_PACK_CACHE_MEMORY_LIMIT.try_into_usize(res))
        .transpose()
        .with_leniency(lenient)?;
    let pack_cache_bytes = config
        .integer_filter_by_key("core.deltaBaseCacheLimit", &mut filter_config_section)
        .map(|res| Core::DELTA_BASE_CACHE_LIMIT.try_into_usize(res))
        .transpose()
        .with_leniency(lenient)?;
    let object_cache_bytes = config
        .integer_filter_by_key("gitoxide.objects.cacheLimit", &mut filter_config_section)
        .map(|res| gitoxide::Objects::CACHE_LIMIT.try_into_usize(res))
        .transpose()
        .with_leniency(lenient)?
        .unwrap_or_default();
    Ok((static_pack_cache_limit, pack_cache_bytes, object_cache_bytes))
}

pub(crate) fn parse_core_abbrev(
    config: &gix_config::File<'static>,
    object_hash: gix_hash::Kind,
) -> Result<Option<usize>, Error> {
    Ok(config
        .string_by_key("core.abbrev")
        .map(|abbrev| Core::ABBREV.try_into_abbreviation(abbrev, object_hash))
        .transpose()?
        .flatten())
}

#[cfg(feature = "revision")]
pub(crate) fn disambiguate_hint(
    config: &gix_config::File<'static>,
    lenient_config: bool,
) -> Result<Option<crate::revision::spec::parse::ObjectKindHint>, config::key::GenericErrorWithValue> {
    match config.string_by_key("core.disambiguate") {
        None => Ok(None),
        Some(value) => Core::DISAMBIGUATE
            .try_into_object_kind_hint(value)
            .with_leniency(lenient_config),
    }
}

// TODO: Use a specialization here once trait specialization is stabilized. Would be perfect here for `T: Default`.
pub trait ApplyLeniency {
    fn with_leniency(self, is_lenient: bool) -> Self;
}

pub trait IgnoreEmptyPath {
    fn ignore_empty(self) -> Self;
}

pub trait ApplyLeniencyDefault {
    fn with_lenient_default(self, is_lenient: bool) -> Self;
}

pub trait ApplyLeniencyDefaultValue<T> {
    fn with_lenient_default_value(self, is_lenient: bool, default: T) -> Self;
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

impl IgnoreEmptyPath for Result<Option<std::borrow::Cow<'_, std::path::Path>>, gix_config::path::interpolate::Error> {
    fn ignore_empty(self) -> Self {
        match self {
            Ok(maybe_path) => Ok(maybe_path),
            Err(gix_config::path::interpolate::Error::Missing { .. }) => Ok(None),
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

impl<T, E> ApplyLeniencyDefaultValue<T> for Result<T, E> {
    fn with_lenient_default_value(self, is_lenient: bool, default: T) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(_) if is_lenient => Ok(default),
            Err(err) => Err(err),
        }
    }
}
