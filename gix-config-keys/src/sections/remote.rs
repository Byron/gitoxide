use crate::{sections::http, keys, Key, Remote, Section, SubSectionRequirement, Tree};

const NAME_PARAMETER: Option<SubSectionRequirement> = Some(SubSectionRequirement::Parameter("name"));

impl Remote {
    /// The `remote.pushDefault` key
    pub const PUSH_DEFAULT: keys::RemoteName = keys::RemoteName::new_remote_name("pushDefault", &Tree::REMOTE);
    /// The `remote.<name>.tagOpt` key
    pub const TAG_OPT: TagOpt = TagOpt::new_with_validate("tagOpt", &Tree::REMOTE, validate::TagOpt)
        .with_subsection_requirement(Some(SubSectionRequirement::Parameter("name")));
    /// The `remote.<name>.url` key
    pub const URL: keys::Url =
        keys::Url::new_url("url", &Tree::REMOTE).with_subsection_requirement(NAME_PARAMETER);
    /// The `remote.<name>.pushUrl` key
    pub const PUSH_URL: keys::Url =
        keys::Url::new_url("pushUrl", &Tree::REMOTE).with_subsection_requirement(NAME_PARAMETER);
    /// The `remote.<name>.fetch` key
    pub const FETCH: keys::FetchRefSpec = keys::FetchRefSpec::new_fetch_refspec("fetch", &Tree::REMOTE)
        .with_subsection_requirement(NAME_PARAMETER);
    /// The `remote.<name>.push` key
    pub const PUSH: keys::PushRefSpec =
        keys::PushRefSpec::new_push_refspec("push", &Tree::REMOTE).with_subsection_requirement(NAME_PARAMETER);
    /// The `remote.<name>.proxy` key
    pub const PROXY: keys::String =
        keys::String::new_string("proxy", &Tree::REMOTE).with_subsection_requirement(NAME_PARAMETER);
    /// The `remote.<name>.proxyAuthMethod` key.
    pub const PROXY_AUTH_METHOD: http::ProxyAuthMethod =
        http::ProxyAuthMethod::new_proxy_auth_method("proxyAuthMethod", &Tree::REMOTE)
            .with_subsection_requirement(NAME_PARAMETER)
            .with_deviation("implemented like git, but never actually tried");
}

impl Section for Remote {
    fn name(&self) -> &str {
        "remote"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[
            &Self::PUSH_DEFAULT,
            &Self::TAG_OPT,
            &Self::URL,
            &Self::PUSH_URL,
            &Self::FETCH,
            &Self::PUSH,
            &Self::PROXY,
            &Self::PROXY_AUTH_METHOD,
        ]
    }
}

/// The `remote.<name>.tagOpt` key type.
pub type TagOpt = keys::Any<validate::TagOpt>;

pub mod validate {
    use std::error::Error;
    use bstr::BStr;
    use crate::keys::Validate;

    pub struct TagOpt;
    impl Validate for TagOpt {
        fn validate(&self, _value: &BStr) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
            // TODO Adapt
            // match value.as_bytes() {
            //     b"--tags" | b"--no-tags" => Ok(()),
            //     _ => Err(Box::new(crate::key::GenericErrorWithValue::from_value(self, value.into()))),
            // }
            Ok(())
        }
    }
}
