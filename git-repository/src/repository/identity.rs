use crate::bstr::BString;
use crate::permission;
use git_actor::SignatureRef;
use git_config::File;
use std::borrow::Cow;

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
            time: p.committer.time.unwrap_or_else(|| git_date::Time::now_local_or_utc()),
        }
        .into()
    }

    ///
    pub fn author(&self) -> Option<git_actor::SignatureRef<'_>> {
        let p = self.config.personas();

        git_actor::SignatureRef {
            name: p.author.name.as_ref().or(p.user.name.as_ref()).map(|v| v.as_ref())?,
            email: p.author.email.as_ref().or(p.user.email.as_ref()).map(|v| v.as_ref())?,
            time: p.author.time.unwrap_or_else(|| git_date::Time::now_local_or_utc()),
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
    author: Entity,
}

impl Personas {
    pub fn from_config_and_env(config: &git_config::File<'_>, git_env: &permission::env_var::Resource) -> Self {
        fn env_var(name: &str) -> Option<BString> {
            std::env::var_os(name).map(|value| git_path::into_bstr(Cow::Owned(value.into())).into_owned())
        }
        fn entity_in_section(name: &str, config: &File<'_>) -> (Option<BString>, Option<BString>) {
            config
                .section(name, None)
                .map(|section| {
                    (
                        section.value("name").map(|v| v.into_owned()),
                        section.value("email").map(|v| v.into_owned()),
                    )
                })
                .unwrap_or_default()
        }

        let (mut committer_name, mut committer_email) = entity_in_section("committer", config);
        let mut committer_date = None;
        let ((mut author_name, mut author_email), mut author_date) = (entity_in_section("author", config), None);
        let (user_name, mut user_email) = entity_in_section("user", config);

        if git_env.eq(&git_sec::Permission::Allow) {
            committer_name = committer_name.or_else(|| env_var("GIT_COMMITTER_NAME"));
            committer_email = committer_email.or_else(|| env_var("GIT_COMMITTER_EMAIL"));
            committer_date = env_var("GIT_COMMITTER_DATE").and_then(|date| git_date::parse(date.as_ref()));

            author_name = author_name.or_else(|| env_var("GIT_AUTHOR_NAME"));
            author_email = author_email.or_else(|| env_var("GIT_AUTHOR_EMAIL"));
            author_date = env_var("GIT_AUTHOR_DATE").and_then(|date| git_date::parse(date.as_ref()));

            user_email = user_email.or_else(|| env_var("EMAIL")); // NOTE: we don't have permission for this specific one…
        }
        Personas {
            user: Entity {
                name: user_name,
                email: user_email,
                time: None,
            },
            committer: Entity {
                name: committer_name,
                email: committer_email,
                time: committer_date,
            },
            author: Entity {
                name: author_name,
                email: author_email,
                time: author_date,
            },
        }
    }
}
