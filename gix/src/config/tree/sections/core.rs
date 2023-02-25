use crate::{
    config,
    config::tree::{keys, Core, Key, Section},
};

impl Core {
    /// The `core.abbrev` key.
    pub const ABBREV: Abbrev = Abbrev::new_with_validate("abbrev", &config::Tree::CORE, validate::Abbrev);
    /// The `core.bare` key.
    pub const BARE: keys::Boolean = keys::Boolean::new_boolean("bare", &config::Tree::CORE);
    /// The `core.checkStat` key.
    pub const CHECK_STAT: CheckStat =
        CheckStat::new_with_validate("checkStat", &config::Tree::CORE, validate::CheckStat);
    /// The `core.deltaBaseCacheLimit` key.
    pub const DELTA_BASE_CACHE_LIMIT: keys::UnsignedInteger =
        keys::UnsignedInteger::new_unsigned_integer("deltaBaseCacheLimit", &config::Tree::CORE)
            .with_environment_override("GITOXIDE_PACK_CACHE_MEMORY")
            .with_note("if unset, we default to a small 64 slot fixed-size cache that holds at most 64 full delta base objects of any size. Set to 0 to deactivate it entirely");
    /// The `core.disambiguate` key.
    pub const DISAMBIGUATE: Disambiguate =
        Disambiguate::new_with_validate("disambiguate", &config::Tree::CORE, validate::Disambiguate);
    /// The `core.fileMode` key.
    pub const FILE_MODE: keys::Boolean = keys::Boolean::new_boolean("fileMode", &config::Tree::CORE);
    /// The `core.ignoreCase` key.
    pub const IGNORE_CASE: keys::Boolean = keys::Boolean::new_boolean("ignoreCase", &config::Tree::CORE);
    /// The `core.filesRefLockTimeout` key.
    pub const FILES_REF_LOCK_TIMEOUT: keys::LockTimeout =
        keys::LockTimeout::new_lock_timeout("filesRefLockTimeout", &config::Tree::CORE);
    /// The `core.packedRefsTimeout` key.
    pub const PACKED_REFS_TIMEOUT: keys::LockTimeout =
        keys::LockTimeout::new_lock_timeout("packedRefsTimeout", &config::Tree::CORE);
    /// The `core.multiPackIndex` key.
    pub const MULTIPACK_INDEX: keys::Boolean = keys::Boolean::new_boolean("multiPackIndex", &config::Tree::CORE);
    /// The `core.logAllRefUpdates` key.
    pub const LOG_ALL_REF_UPDATES: LogAllRefUpdates =
        LogAllRefUpdates::new_with_validate("logAllRefUpdates", &config::Tree::CORE, validate::LogAllRefUpdates);
    /// The `core.precomposeUnicode` key.
    ///
    /// Needs application to use [env::args_os][crate::env::args_os()] to conform all input paths before they are used.
    pub const PRECOMPOSE_UNICODE: keys::Boolean = keys::Boolean::new_boolean("precomposeUnicode", &config::Tree::CORE)
        .with_note("application needs to conform all program input by using gix::env::args_os()");
    /// The `core.repositoryFormatVersion` key.
    pub const REPOSITORY_FORMAT_VERSION: keys::UnsignedInteger =
        keys::UnsignedInteger::new_unsigned_integer("repositoryFormatVersion", &config::Tree::CORE);
    /// The `core.symlinks` key.
    pub const SYMLINKS: keys::Boolean = keys::Boolean::new_boolean("symlinks", &config::Tree::CORE);
    /// The `core.trustCTime` key.
    pub const TRUST_C_TIME: keys::Boolean = keys::Boolean::new_boolean("trustCTime", &config::Tree::CORE);
    /// The `core.worktree` key.
    pub const WORKTREE: keys::Any = keys::Any::new("worktree", &config::Tree::CORE)
        .with_environment_override("GIT_WORK_TREE")
        .with_deviation("Overriding the worktree with environment variables is supported using `ThreadSafeRepository::open_with_environment_overrides()");
    /// The `core.askPass` key.
    pub const ASKPASS: keys::Executable = keys::Executable::new_executable("askPass", &config::Tree::CORE)
        .with_environment_override("GIT_ASKPASS")
        .with_note("fallback is 'SSH_ASKPASS'");
    /// The `core.excludesFile` key.
    pub const EXCLUDES_FILE: keys::Executable = keys::Executable::new_executable("excludesFile", &config::Tree::CORE);
    /// The `core.attributesFile` key.
    pub const ATTRIBUTES_FILE: keys::Executable =
        keys::Executable::new_executable("attributesFile", &config::Tree::CORE)
            .with_deviation("for checkout - it's already queried but needs building of attributes group, and of course support during checkout");
    /// The `core.sshCommand` key.
    pub const SSH_COMMAND: keys::Executable = keys::Executable::new_executable("sshCommand", &config::Tree::CORE)
        .with_environment_override("GIT_SSH_COMMAND");
}

impl Section for Core {
    fn name(&self) -> &str {
        "core"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[
            &Self::ABBREV,
            &Self::BARE,
            &Self::CHECK_STAT,
            &Self::DELTA_BASE_CACHE_LIMIT,
            &Self::DISAMBIGUATE,
            &Self::FILE_MODE,
            &Self::IGNORE_CASE,
            &Self::FILES_REF_LOCK_TIMEOUT,
            &Self::PACKED_REFS_TIMEOUT,
            &Self::MULTIPACK_INDEX,
            &Self::LOG_ALL_REF_UPDATES,
            &Self::PRECOMPOSE_UNICODE,
            &Self::REPOSITORY_FORMAT_VERSION,
            &Self::SYMLINKS,
            &Self::TRUST_C_TIME,
            &Self::WORKTREE,
            &Self::ASKPASS,
            &Self::EXCLUDES_FILE,
            &Self::ATTRIBUTES_FILE,
            &Self::SSH_COMMAND,
        ]
    }
}

/// The `core.checkStat` key.
pub type CheckStat = keys::Any<validate::CheckStat>;

/// The `core.abbrev` key.
pub type Abbrev = keys::Any<validate::Abbrev>;

/// The `core.logAllRefUpdates` key.
pub type LogAllRefUpdates = keys::Any<validate::LogAllRefUpdates>;

/// The `core.disambiguate` key.
pub type Disambiguate = keys::Any<validate::Disambiguate>;

mod disambiguate {
    use std::borrow::Cow;

    use crate::{
        bstr::{BStr, ByteSlice},
        config,
        config::tree::core::Disambiguate,
        revision::spec::parse::ObjectKindHint,
    };

    impl Disambiguate {
        /// Convert a disambiguation marker into the respective enum.
        pub fn try_into_object_kind_hint(
            &'static self,
            value: Cow<'_, BStr>,
        ) -> Result<Option<ObjectKindHint>, config::key::GenericErrorWithValue> {
            let hint = match value.as_ref().as_bytes() {
                b"none" => return Ok(None),
                b"commit" => ObjectKindHint::Commit,
                b"committish" => ObjectKindHint::Committish,
                b"tree" => ObjectKindHint::Tree,
                b"treeish" => ObjectKindHint::Treeish,
                b"blob" => ObjectKindHint::Blob,
                _ => return Err(config::key::GenericErrorWithValue::from_value(self, value.into_owned())),
            };
            Ok(Some(hint))
        }
    }
}

mod log_all_ref_updates {
    use std::borrow::Cow;

    use crate::{bstr::BStr, config, config::tree::core::LogAllRefUpdates};

    impl LogAllRefUpdates {
        /// Returns the mode for ref-updates as parsed from `value`. If `value` is not a boolean, `string_on_failure` will be called
        /// to obtain the key `core.logAllRefUpdates` as string instead. For correctness, this two step process is necessary as
        /// the interpretation of booleans in special in `gix-config`, i.e. we can't just treat it as string.
        pub fn try_into_ref_updates<'a>(
            &'static self,
            value: Option<Result<bool, gix_config::value::Error>>,
            string_on_failure: impl FnOnce() -> Option<Cow<'a, BStr>>,
        ) -> Result<Option<gix_ref::store::WriteReflog>, config::key::GenericErrorWithValue> {
            match value.transpose().ok().flatten() {
                Some(bool) => Ok(Some(if bool {
                    gix_ref::store::WriteReflog::Normal
                } else {
                    gix_ref::store::WriteReflog::Disable
                })),
                None => match string_on_failure() {
                    Some(val) if val.eq_ignore_ascii_case(b"always") => Ok(Some(gix_ref::store::WriteReflog::Always)),
                    Some(val) => Err(config::key::GenericErrorWithValue::from_value(self, val.into_owned())),
                    None => Ok(None),
                },
            }
        }
    }
}

mod check_stat {
    use std::borrow::Cow;

    use crate::{
        bstr::{BStr, ByteSlice},
        config,
        config::tree::core::CheckStat,
    };

    impl CheckStat {
        /// Returns true if the full set of stat entries should be checked, and it's just as lenient as git.
        pub fn try_into_checkstat(
            &'static self,
            value: Cow<'_, BStr>,
        ) -> Result<bool, config::key::GenericErrorWithValue> {
            Ok(match value.as_ref().as_bytes() {
                b"minimal" => false,
                b"default" => true,
                _ => {
                    return Err(config::key::GenericErrorWithValue::from_value(self, value.into_owned()));
                }
            })
        }
    }
}

mod abbrev {
    use std::borrow::Cow;

    use config::abbrev::Error;

    use crate::{
        bstr::{BStr, ByteSlice},
        config,
        config::tree::core::Abbrev,
    };

    impl Abbrev {
        /// Convert the given `hex_len_str` into the amount of characters that a short hash should have.
        /// If `None` is returned, the correct value can be determined based on the amount of objects in the repo.
        pub fn try_into_abbreviation(
            &'static self,
            hex_len_str: Cow<'_, BStr>,
            object_hash: gix_hash::Kind,
        ) -> Result<Option<usize>, Error> {
            let max = object_hash.len_in_hex() as u8;
            if hex_len_str.trim().is_empty() {
                return Err(Error {
                    value: hex_len_str.into_owned(),
                    max,
                });
            }
            if hex_len_str.trim().eq_ignore_ascii_case(b"auto") {
                Ok(None)
            } else {
                let value_bytes = hex_len_str.as_ref();
                if let Ok(false) = gix_config::Boolean::try_from(value_bytes).map(Into::into) {
                    Ok(object_hash.len_in_hex().into())
                } else {
                    let value = gix_config::Integer::try_from(value_bytes)
                        .map_err(|_| Error {
                            value: hex_len_str.clone().into_owned(),
                            max,
                        })?
                        .to_decimal()
                        .ok_or_else(|| Error {
                            value: hex_len_str.clone().into_owned(),
                            max,
                        })?;
                    if value < 4 || value as usize > object_hash.len_in_hex() {
                        return Err(Error {
                            value: hex_len_str.clone().into_owned(),
                            max,
                        });
                    }
                    Ok(Some(value as usize))
                }
            }
        }
    }
}

mod validate {
    use crate::{bstr::BStr, config::tree::keys};

    pub struct LockTimeout;
    impl keys::Validate for LockTimeout {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            let value = gix_config::Integer::try_from(value)?
                .to_decimal()
                .ok_or_else(|| format!("integer {value} cannot be represented as integer"));
            super::Core::FILES_REF_LOCK_TIMEOUT.try_into_lock_timeout(Ok(value?))?;
            Ok(())
        }
    }

    pub struct Disambiguate;
    impl keys::Validate for Disambiguate {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            super::Core::DISAMBIGUATE.try_into_object_kind_hint(value.into())?;
            Ok(())
        }
    }

    pub struct LogAllRefUpdates;
    impl keys::Validate for LogAllRefUpdates {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            super::Core::LOG_ALL_REF_UPDATES
                .try_into_ref_updates(Some(gix_config::Boolean::try_from(value).map(|b| b.0)), || {
                    Some(value.into())
                })?;
            Ok(())
        }
    }

    pub struct CheckStat;
    impl keys::Validate for CheckStat {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            super::Core::CHECK_STAT.try_into_checkstat(value.into())?;
            Ok(())
        }
    }

    pub struct Abbrev;
    impl keys::Validate for Abbrev {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            // TODO: when there is options, validate against all hashes and assure all fail to trigger a validation failure.
            super::Core::ABBREV.try_into_abbreviation(value.into(), gix_hash::Kind::Sha1)?;
            Ok(())
        }
    }
}
