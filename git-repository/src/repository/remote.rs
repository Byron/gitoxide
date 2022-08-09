use crate::bstr::{BStr, BString, ByteVec};
use crate::remote::find;
use crate::Remote;

impl crate::Repository {
    /// Find the remote with the given `name` or report an error, similar to [`try_find_remote(…)`][Self::try_find_remote()].
    ///
    /// Note that we will include remotes only if we deem them [trustworthy][crate::open::Options::filter_config_section()].
    pub fn find_remote(&self, name: &str) -> Result<Remote<'_>, find::existing::Error> {
        Ok(self
            .try_find_remote(name)
            .ok_or_else(|| find::existing::Error::NotFound { name: name.into() })??)
    }

    /// Find the remote with the given `name` or return `None` if it doesn't exist, for the purpose of fetching or pushing
    /// data to a remote.
    ///
    /// There are various error kinds related to partial information or incorrectly formatted URLs or ref-specs.
    /// Also note that the created `Remote` may have neither fetch nor push ref-specs set at all.
    ///
    /// Note that ref-specs are de-duplicated right away which may change their order. This doesn't affect matching in any way
    /// as negations/excludes are applied after includes.
    ///
    /// We will only include information if we deem it [trustworthy][crate::open::Options::filter_config_section()].
    pub fn try_find_remote(&self, name: &str) -> Option<Result<Remote<'_>, find::Error>> {
        let mut filter = self.filter_config_section();
        let mut config_url = |field: &str, kind: &'static str| {
            self.config
                .resolved
                .string_filter("remote", name.into(), field, &mut filter)
                .map(|url| {
                    git_url::parse::parse(url.as_ref()).map_err(|err| find::Error::UrlInvalid {
                        kind,
                        url: url.into_owned(),
                        source: err,
                    })
                })
        };
        let url = config_url("url", "fetch");
        let push_url = config_url("pushUrl", "push");

        let mut config_spec = |op: git_refspec::parse::Operation| {
            let kind = match op {
                git_refspec::parse::Operation::Fetch => "fetch",
                git_refspec::parse::Operation::Push => "push",
            };
            self.config
                .resolved
                .strings_filter("remote", name.into(), kind, &mut filter)
                .map(|specs| {
                    specs
                        .into_iter()
                        .map(|spec| {
                            git_refspec::parse(spec.as_ref(), op)
                                .map(|spec| spec.to_owned())
                                .map_err(|err| find::Error::RefSpec {
                                    spec: spec.into_owned(),
                                    kind,
                                    source: err,
                                })
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map(|mut specs| {
                            specs.sort();
                            specs.dedup();
                            specs
                        })
                })
        };
        let fetch_specs = config_spec(git_refspec::parse::Operation::Fetch);
        let push_specs = config_spec(git_refspec::parse::Operation::Push);

        match (url, fetch_specs, push_url, push_specs) {
            (None, None, None, None) => None,
            (None, _, None, _) => Some(Err(find::Error::UrlMissing)),
            (url, fetch_specs, push_url, push_specs) => {
                let url = match url {
                    Some(Ok(v)) => Some(v),
                    Some(Err(err)) => return Some(Err(err)),
                    None => None,
                };
                let push_url = match push_url {
                    Some(Ok(v)) => Some(v),
                    Some(Err(err)) => return Some(Err(err)),
                    None => None,
                };
                let fetch_specs = match fetch_specs {
                    Some(Ok(v)) => v,
                    Some(Err(err)) => return Some(Err(err)),
                    None => Vec::new(),
                };
                let push_specs = match push_specs {
                    Some(Ok(v)) => v,
                    Some(Err(err)) => return Some(Err(err)),
                    None => Vec::new(),
                };

                let (url_alias, push_url_alias) =
                    match rewrite_urls(&self.config.resolved, url.as_ref(), push_url.as_ref(), filter) {
                        Ok(t) => t,
                        Err(err) => return Some(Err(err)),
                    };
                Some(Ok(Remote {
                    name: name.to_owned().into(),
                    url,
                    url_alias,
                    push_url,
                    push_url_alias,
                    fetch_specs,
                    push_specs,
                    apply_url_aliases: true,
                    repo: self,
                }))
            }
        }
    }
}

fn rewrite_urls(
    config: &git_config::File<'static>,
    url: Option<&git_url::Url>,
    push_url: Option<&git_url::Url>,
    mut filter: fn(&git_config::file::Metadata) -> bool,
) -> Result<(Option<git_url::Url>, Option<git_url::Url>), find::Error> {
    let mut url_alias = None;
    let mut push_url_alias = None;
    if let Some(sections) = config.sections_by_name_and_filter("url", &mut filter) {
        let mut rewrite_url = None::<(usize, &BStr)>;
        let mut rewrite_push_url = None::<(usize, &BStr)>;
        let url = url.as_ref().map(|url| url.to_bstring().expect("still valid"));
        let push_url = push_url.as_ref().map(|url| url.to_bstring().expect("still valid"));
        for section in sections {
            let rewrite_with = match section.header().subsection_name() {
                Some(base) => base,
                None => continue,
            };
            if let Some(url) = url.as_deref() {
                for instead_of in section.values("insteadOf") {
                    if url.starts_with(instead_of.as_ref()) {
                        let (bytes_matched, prev_rewrite_with) =
                            rewrite_url.get_or_insert_with(|| (instead_of.len(), rewrite_with));
                        if *bytes_matched < instead_of.len() {
                            *bytes_matched = instead_of.len();
                            *prev_rewrite_with = rewrite_with;
                        }
                    }
                }
            }
            if let Some(url) = push_url.as_deref() {
                for instead_of in section.values("pushInsteadOf") {
                    if url.starts_with(instead_of.as_ref()) {
                        let (bytes_matched, prev_rewrite_with) =
                            rewrite_push_url.get_or_insert_with(|| (instead_of.len(), rewrite_with));
                        if *bytes_matched < instead_of.len() {
                            *bytes_matched = instead_of.len();
                            *prev_rewrite_with = rewrite_with;
                        }
                    }
                }
            }
        }

        fn replace_url(
            url: Option<BString>,
            rewrite: Option<(usize, &BStr)>,
            kind: &'static str,
        ) -> Result<Option<git_url::Url>, find::Error> {
            url.zip(rewrite)
                .map(|(mut url, (bytes_at_start, replace_with))| {
                    url.replace_range(..bytes_at_start, replace_with);
                    git_url::parse(&url).map_err(|err| find::Error::RewrittenUrlInvalid {
                        kind,
                        source: err,
                        rewritten_url: url,
                    })
                })
                .transpose()
        }
        url_alias = replace_url(url, rewrite_url, "fetch")?;
        push_url_alias = replace_url(push_url, rewrite_push_url, "push")?;
    }
    Ok((url_alias, push_url_alias))
}
