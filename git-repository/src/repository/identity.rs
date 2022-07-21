use crate::bstr::BString;
use crate::permission;
use git_actor::SignatureRef;

/// Identity handling.
impl crate::Repository {
    /// Return a crate-specific constant signature with [`Time`][git_actor::Time] set to now,
    /// in a similar vein as the default that git chooses if there is nothing configured.
    ///
    /// This can be useful as fallback for an unset `committer` or `author`.
    pub fn user_default() -> SignatureRef<'static> {
        SignatureRef {
            name: "gitoxide".into(),
            email: "gitoxide@localhost".into(),
            time: Default::default(),
        }
    }

    // TODO: actual implementation
    /// Return the committer as configured by this repository, which is determined by…
    ///
    /// * …the git configuration `committer.name|email`…
    /// * …the `GIT_(COMMITTER)_(NAME|EMAIL|DATE)` and `EMAIL` environment variables…
    /// * …the configuration for `user.name|email` as fallback…
    ///
    /// …and in that order, or `None` if there was nothing configured. In that case, one may use the
    /// [`user_default()`][Self::user_default()] method.
    ///
    /// The values are cached when the repository is instantiated.
    pub fn committer(&self) -> git_actor::Signature {
        git_actor::Signature::empty()
    }

    ///
    pub fn committer2(&self) -> Option<git_actor::SignatureRef<'_>> {
        let p = self.config.personas();

        git_actor::SignatureRef {
            name: p.committer.name.as_ref().or(p.user.name.as_ref()).map(|v| v.as_ref())?,
            email: p
                .committer
                .email
                .as_ref()
                .or(p.user.email.as_ref())
                .map(|v| v.as_ref())?,
            time: p.committer.time.unwrap_or_else(|| todo!("get local time")),
        }
        .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Entity {
    pub name: Option<BString>,
    pub email: Option<BString>,
    /// A time parsed from an environment variable.
    pub time: Option<git_actor::Time>,
}

#[derive(Debug, Clone)]
pub(crate) struct Personas {
    user: Entity,
    committer: Entity,
    // author: Entity,
}

impl Personas {
    pub fn from_config_and_env(_config: &git_config::File<'_>, _git_env: &permission::env_var::Resource) -> Self {
        todo!()
    }
}
