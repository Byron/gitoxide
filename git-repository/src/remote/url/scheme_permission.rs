#![allow(dead_code, unused_variables)]
use crate::permission;
use std::collections::BTreeMap;

///
pub mod init {
    use crate::bstr::BString;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("{value:?} must be allow|deny|user in configuration key protocol{0}.allow", scheme.as_ref().map(|s| format!(".{}", s)).unwrap_or_default())]
        InvalidConfiguration { scheme: Option<BString>, value: BString },
    }
}

#[derive(Debug, Clone, Copy)]
enum Allow {
    Always,
    Never,
    User,
}

impl Allow {
    pub fn to_bool(&self, user_allowed: Option<bool>) -> bool {
        match self {
            Allow::Always => true,
            Allow::Never => false,
            Allow::User => user_allowed.unwrap_or(true),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SchemePermission {
    /// `None`, env-var is unset or wasn't queried, otherwise true if `GIT_PROTOCOL_FROM_USER` is `1`.
    user_allowed: Option<bool>,
    /// The general allow value from `protocol.allow`.
    allow: Option<Allow>,
    /// Per scheme allow information
    allow_per_scheme: BTreeMap<git_url::Scheme, Allow>,
}

/// Init
impl SchemePermission {
    pub fn from_config(
        config: &git_config::File<'static>,
        git_prefix: &permission::env_var::Resource,
    ) -> Result<Self, init::Error> {
        todo!()
    }
}

/// Access
impl SchemePermission {
    pub fn allow(&self, scheme: git_url::Scheme) -> bool {
        self.allow_per_scheme.get(&scheme).or(self.allow.as_ref()).map_or_else(
            || {
                use git_url::Scheme::*;
                match scheme {
                    File | Git | Ssh | Http | Https => true,
                    Ext(_) => false,
                    // TODO: figure out what 'ext' really entails, and what 'other' protocols are which aren't representable for us yet
                }
            },
            |allow| allow.to_bool(self.user_allowed),
        )
    }
}
