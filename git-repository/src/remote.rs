mod errors {
    ///
    pub mod find {
        use crate::bstr::BString;

        /// The error returned by [`Repository::find_remote(…)`][crate::Repository::find_remote()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("{spec:?} {kind} ref-spec failed to parse")]
            RefSpec {
                spec: BString,
                kind: &'static str,
                source: git_refspec::parse::Error,
            },
            #[error("Neither 'url` nor 'pushUrl' fields were set in the remote's configuration.")]
            UrlMissing,
            #[error("The {kind} url couldn't be parsed")]
            UrlInvalid {
                kind: &'static str,
                url: BString,
                source: git_url::parse::Error,
            },
            #[error("The rewritten {kind} url {rewritten_url:?} failed to parse")]
            RewrittenUrlInvalid {
                kind: &'static str,
                rewritten_url: BString,
                source: git_url::parse::Error,
            },
        }

        ///
        pub mod existing {
            /// The error returned by [`Repository::find_remote(…)`][crate::Repository::find_remote()].
            #[derive(Debug, thiserror::Error)]
            #[allow(missing_docs)]
            pub enum Error {
                #[error(transparent)]
                Find(#[from] super::Error),
                #[error("The remote named {name:?} did not exist")]
                NotFound { name: String },
            }
        }
    }
}
pub use errors::find;

mod access {
    use crate::{remote, Remote};
    use git_refspec::RefSpec;

    /// Builder methods
    impl Remote<'_> {
        /// By default, `url.<base>.insteadOf|pushInsteadOf` configuration variables will be applied to rewrite fetch and push
        /// urls unless `toggle` is `false`.
        pub fn apply_url_aliases(mut self, toggle: bool) -> Self {
            self.apply_url_aliases = toggle;
            self
        }
    }

    impl Remote<'_> {
        /// Return the name of this remote or `None` if it wasn't persisted to disk yet.
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }

        /// Return the set of ref-specs used for `direction`, which may be empty, in order of occurrence in the configuration.
        pub fn refspecs(&self, direction: remote::Direction) -> &[RefSpec] {
            match direction {
                remote::Direction::Fetch => &self.fetch_specs,
                remote::Direction::Push => &self.push_specs,
            }
        }

        /// Return the url used for the given `direction` with rewrites from `url.<base>.insteadOf|pushInsteadOf` applied unless
        /// [`apply_url_aliases(false)`][Self::apply_url_aliases()] was called before.
        /// For pushing, this is the `remote.<name>.pushUrl` or the `remote.<name>.url` used for fetching, and for fetching it's
        /// the `remote.<name>.url`.
        pub fn url(&self, direction: remote::Direction) -> Option<&git_url::Url> {
            match direction {
                remote::Direction::Fetch => self
                    .apply_url_aliases
                    .then(|| self.url_alias.as_ref())
                    .flatten()
                    .or(self.url.as_ref()),
                remote::Direction::Push => self
                    .apply_url_aliases
                    .then(|| self.push_url_alias.as_ref())
                    .flatten()
                    .or(self.push_url.as_ref())
                    .or_else(|| self.url(remote::Direction::Fetch)),
            }
        }
    }
}

pub(crate) mod url {
    use crate::bstr::{BStr, BString, ByteVec};
    use crate::remote::Direction;
    use git_features::threading::OwnShared;

    #[derive(Debug, Clone)]
    pub(crate) struct Replace {
        find: BString,
        with: OwnShared<BString>,
    }

    #[derive(Default, Debug, Clone)]
    pub(crate) struct Rewrite {
        url_rewrite: Vec<Replace>,
        push_url_rewrite: Vec<Replace>,
    }

    /// Init
    impl Rewrite {
        pub fn from_config(
            config: &git_config::File<'static>,
            mut filter: fn(&git_config::file::Metadata) -> bool,
        ) -> Rewrite {
            config
                .sections_by_name_and_filter("url", &mut filter)
                .map(|sections| {
                    let mut url_rewrite = Vec::new();
                    let mut push_url_rewrite = Vec::new();
                    for section in sections {
                        let replace = match section.header().subsection_name() {
                            Some(base) => OwnShared::new(base.to_owned()),
                            None => continue,
                        };

                        for instead_of in section.values("insteadOf") {
                            url_rewrite.push(Replace {
                                with: OwnShared::clone(&replace),
                                find: instead_of.into_owned(),
                            });
                        }
                        for instead_of in section.values("pushInsteadOf") {
                            push_url_rewrite.push(Replace {
                                with: OwnShared::clone(&replace),
                                find: instead_of.into_owned(),
                            });
                        }
                    }
                    Rewrite {
                        url_rewrite,
                        push_url_rewrite,
                    }
                })
                .unwrap_or_default()
        }
    }

    /// Access
    impl Rewrite {
        fn replacements_for(&self, direction: Direction) -> &[Replace] {
            match direction {
                Direction::Fetch => &self.url_rewrite,
                Direction::Push => &self.push_url_rewrite,
            }
        }

        pub fn rewrite_url(&self, url: &git_url::Url, direction: Direction) -> Option<BString> {
            if self.replacements_for(direction).is_empty() {
                None
            } else {
                let mut url = url.to_bstring().ok()?;
                self.rewrite_url_in_place(&mut url, direction).then(|| url)
            }
        }

        /// Rewrite the given `url` of `direction` and return `true` if a replacement happened.
        ///
        /// Note that the result must still be checked for validity, it might not be a valid URL as we do a syntax-unaware replacement.
        pub fn rewrite_url_in_place(&self, url: &mut BString, direction: Direction) -> bool {
            self.replacements_for(direction)
                .iter()
                .fold(None::<(usize, &BStr)>, |mut acc, replace| {
                    if url.starts_with(replace.find.as_ref()) {
                        let (bytes_matched, prev_rewrite_with) =
                            acc.get_or_insert((replace.find.len(), replace.with.as_slice().into()));
                        if *bytes_matched < replace.find.len() {
                            *bytes_matched = replace.find.len();
                            *prev_rewrite_with = replace.with.as_slice().into();
                        }
                    };
                    acc
                })
                .map(|(bytes_matched, replace_with)| {
                    url.replace_range(..bytes_matched, replace_with);
                })
                .is_some()
        }
    }
}

/// The direction of an operation carried out (or to be carried out) through a remote.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Direction {
    /// Push local changes to the remote.
    Push,
    /// Fetch changes from the remote to the local repository.
    Fetch,
}
