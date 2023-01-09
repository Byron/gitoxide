use std::time::SystemTime;

use crate::bstr::{BString, ByteSlice};

/// Identity handling.
///
/// # Deviation
///
/// There is no notion of a default user like in git, and instead failing to provide a user
/// is fatal. That way, we enforce correctness and force application developers to take care
/// of this issue which can be done in various ways, for instance by setting
/// `gitoxide.committer.nameFallback` and similar.
impl crate::Repository {
    /// Return the committer as configured by this repository, which is determined by…
    ///
    /// * …the git configuration `committer.name|email`…
    /// * …the `GIT_COMMITTER_(NAME|EMAIL|DATE)` environment variables…
    /// * …the configuration for `user.name|email` as fallback…
    ///
    /// …and in that order, or `None` if there was nothing configured.
    ///
    /// # Note
    ///
    /// The values are cached when the repository is instantiated.
    pub fn committer(&self) -> Option<git_actor::SignatureRef<'_>> {
        let p = self.config.personas();

        git_actor::SignatureRef {
            name: p.committer.name.as_ref().or(p.user.name.as_ref()).map(|v| v.as_ref())?,
            email: p
                .committer
                .email
                .as_ref()
                .or(p.user.email.as_ref())
                .map(|v| v.as_ref())?,
            time: p.committer.time.unwrap_or_else(git_date::Time::now_local_or_utc),
        }
        .into()
    }

    /// Return the author as configured by this repository, which is determined by…
    ///
    /// * …the git configuration `author.name|email`…
    /// * …the `GIT_AUTHOR_(NAME|EMAIL|DATE)` environment variables…
    /// * …the configuration for `user.name|email` as fallback…
    ///
    /// …and in that order, or `None` if there was nothing configured.
    ///
    /// # Note
    ///
    /// The values are cached when the repository is instantiated.
    pub fn author(&self) -> Option<git_actor::SignatureRef<'_>> {
        let p = self.config.personas();

        git_actor::SignatureRef {
            name: p.author.name.as_ref().or(p.user.name.as_ref()).map(|v| v.as_ref())?,
            email: p.author.email.as_ref().or(p.user.email.as_ref()).map(|v| v.as_ref())?,
            time: p.author.time.unwrap_or_else(git_date::Time::now_local_or_utc),
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
    pub fn from_config_and_env(config: &git_config::File<'_>) -> Self {
        fn entity_in_section(
            section_name: &str,
            config: &git_config::File<'_>,
            fallback: bool,
        ) -> (Option<BString>, Option<BString>) {
            let fallback = fallback
                .then(|| config.section("gitoxide", Some(section_name.into())).ok())
                .flatten();
            (
                config
                    .string(section_name, None, "name")
                    .or_else(|| fallback.as_ref().and_then(|s| s.value("nameFallback")))
                    .map(|v| v.into_owned()),
                config
                    .string(section_name, None, "email")
                    .or_else(|| fallback.as_ref().and_then(|s| s.value("emailFallback")))
                    .map(|v| v.into_owned()),
            )
        }
        let now = SystemTime::now();
        let parse_date = |key: &str| -> Option<git_date::Time> {
            config.string_by_key(key).and_then(|date| {
                date.to_str()
                    .ok()
                    .and_then(|date| git_date::parse(date, Some(now)).ok())
            })
        };

        let (committer_name, committer_email) = entity_in_section("committer", config, true);
        let (author_name, author_email) = entity_in_section("author", config, true);
        let (user_name, mut user_email) = entity_in_section("user", config, false);

        let committer_date = parse_date("gitoxide.commit.committerDate");
        let author_date = parse_date("gitoxide.commit.authorDate");

        user_email = user_email.or_else(|| config.string_by_key("gitoxide.user.email").map(|v| v.into_owned()));
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
