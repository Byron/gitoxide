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

    /// Find the remote with the given `name` or return `None` if it doesn't exist.
    /// Note that there are various error kinds related to partial information or incorrectly formatted URLs or ref-specs.
    /// Also note that the created `Remote` may have neither fetch nor push ref-specs set at all.
    ///
    /// Note that we will include remotes only if we deem them [trustworthy][crate::open::Options::filter_config_section()].
    pub fn try_find_remote(&self, name: &str) -> Option<Result<Remote<'_>, find::Error>> {
        let mut filter = self.filter_config_section();
        let mut config_url = |field: &str, kind: &'static str| {
            self.config
                .resolved
                .string_filter("remote", name.into(), field, &mut filter)
                .map(|url| {
                    git_url::parse::parse(url.as_ref()).map_err(|err| find::Error::UrlInvalid { kind, source: err })
                })
        };
        let url = config_url("url", "fetch");
        let push_url = config_url("pushUrl", "push");

        let mut config_spec = |op: git_refspec::parse::Operation| {
            self.config
                .resolved
                .strings_filter(
                    "remote",
                    name.into(),
                    match op {
                        git_refspec::parse::Operation::Fetch => "fetch",
                        git_refspec::parse::Operation::Push => "push",
                    },
                    &mut filter,
                )
                .map(|specs| {
                    specs
                        .into_iter()
                        .map(|spec| git_refspec::parse(spec.as_ref(), op).map(|spec| spec.to_owned()))
                        .collect::<Result<Vec<_>, _>>()
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
                    Some(Err(err)) => return Some(Err(err.into())),
                    None => Vec::new(),
                };
                let push_specs = match push_specs {
                    Some(Ok(v)) => v,
                    Some(Err(err)) => return Some(Err(err.into())),
                    None => Vec::new(),
                };
                Some(Ok(Remote {
                    name: name.to_owned().into(),
                    url,
                    push_url,
                    fetch_specs,
                    push_specs,
                    repo: self,
                }))
            }
        }
    }
}
