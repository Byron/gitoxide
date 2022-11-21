use crate::bstr::BStr;
use std::any::Any;

impl crate::Repository {
    /// Produce configuration suitable for `url`, as differentiated by its protocol/scheme, to be passed to a transport instance via
    /// [configure()][git_transport::client::TransportWithoutIO::configure()] (via `&**config` to pass the contained `Any` and not the `Box`).
    /// `None` is returned if there is no known configuration.
    ///
    /// Note that the caller may cast the instance themselves to modify it before passing it on.
    pub fn transport_options<'a>(
        &self,
        url: impl Into<&'a BStr>,
    ) -> Result<Option<Box<dyn Any>>, crate::config::transport::Error> {
        let url = git_url::parse(url.into())?;
        use git_url::Scheme::*;

        match &url.scheme {
            Http | Https => {
                #[cfg(not(any(
                    feature = "blocking-http-transport-reqwest",
                    feature = "blocking-http-transport-curl"
                )))]
                {
                    Ok(None)
                }
                #[cfg(any(
                    feature = "blocking-http-transport-reqwest",
                    feature = "blocking-http-transport-curl"
                ))]
                {
                    use crate::bstr::ByteVec;
                    use crate::config::cache::util::{ApplyLeniency, ApplyLeniencyDefault};
                    use git_transport::client::http;
                    use std::borrow::Cow;

                    fn try_cow_to_string(
                        v: Cow<'_, BStr>,
                        lenient: bool,
                        key: &'static str,
                    ) -> Result<Option<String>, crate::config::transport::Error> {
                        Vec::from(v.into_owned())
                            .into_string()
                            .map(Some)
                            .map_err(|err| crate::config::transport::Error::IllformedUtf8 { source: err, key })
                            .with_leniency(lenient)
                    }

                    fn integer<T>(
                        config: &git_config::File<'static>,
                        lenient: bool,
                        key: &'static str,
                        kind: &'static str,
                        filter: fn(&git_config::file::Metadata) -> bool,
                        default: T,
                    ) -> Result<T, crate::config::transport::Error>
                    where
                        T: TryFrom<i64>,
                    {
                        Ok(integer_opt(config, lenient, key, kind, filter)?.unwrap_or(default))
                    }
                    fn integer_opt<T>(
                        config: &git_config::File<'static>,
                        lenient: bool,
                        key: &'static str,
                        kind: &'static str,
                        mut filter: fn(&git_config::file::Metadata) -> bool,
                    ) -> Result<Option<T>, crate::config::transport::Error>
                    where
                        T: TryFrom<i64>,
                    {
                        let git_config::parse::Key {
                            section_name,
                            subsection_name,
                            value_name,
                        } = git_config::parse::key(key).expect("valid key statically known");
                        config
                            .integer_filter(section_name, subsection_name, value_name, &mut filter)
                            .transpose()
                            .map_err(|err| crate::config::transport::Error::ConfigValue { source: err, key })
                            .with_leniency(lenient)?
                            .map(|integer| {
                                integer
                                    .try_into()
                                    .map_err(|_| crate::config::transport::Error::InvalidInteger {
                                        actual: integer,
                                        key,
                                        kind,
                                    })
                            })
                            .transpose()
                            .with_leniency(lenient)
                    }
                    let mut opts = http::Options::default();
                    let config = &self.config.resolved;
                    let mut trusted_only = self.filter_config_section();
                    let lenient = self.config.lenient_config;
                    opts.extra_headers = {
                        let mut headers = Vec::new();
                        for header in config
                            .strings_filter("http", None, "extraHeader", &mut trusted_only)
                            .unwrap_or_default()
                            .into_iter()
                            .map(|v| try_cow_to_string(v, lenient, "http.extraHeader"))
                        {
                            let header = header?;
                            if let Some(header) = header {
                                headers.push(header);
                            }
                        }
                        if let Some(empty_pos) = headers.iter().rev().position(|h| h.is_empty()) {
                            headers.drain(..headers.len() - empty_pos);
                        }
                        headers
                    };

                    if let Some(follow_redirects) =
                        config.string_filter("http", None, "followRedirects", &mut trusted_only)
                    {
                        opts.follow_redirects = if follow_redirects.as_ref() == "initial" {
                            http::options::FollowRedirects::Initial
                        } else if git_config::Boolean::try_from(follow_redirects)
                            .map_err(|err| crate::config::transport::Error::ConfigValue {
                                source: err,
                                key: "http.followRedirects",
                            })
                            .with_lenient_default(lenient)?
                            .0
                        {
                            http::options::FollowRedirects::All
                        } else {
                            http::options::FollowRedirects::None
                        };
                    }

                    opts.low_speed_time_seconds =
                        integer(config, lenient, "http.lowSpeedTime", "u64", trusted_only, 0)?;
                    opts.low_speed_limit_bytes_per_second =
                        integer(config, lenient, "http.lowSpeedLimit", "u32", trusted_only, 0)?;
                    opts.proxy = config
                        .string_filter("http", None, "proxy", &mut trusted_only)
                        .and_then(|v| try_cow_to_string(v, lenient, "http.proxy").transpose())
                        .transpose()?
                        .map(|mut proxy| {
                            if !proxy.trim().is_empty() && !proxy.contains("://") {
                                proxy.insert_str(0, "http://");
                                proxy
                            } else {
                                proxy
                            }
                        });
                    opts.connect_timeout =
                        integer_opt(config, lenient, "gitoxide.http.connectTimeout", "u64", trusted_only)?
                            .map(std::time::Duration::from_millis);
                    opts.user_agent = config
                        .string_filter("http", None, "userAgent", &mut trusted_only)
                        .and_then(|v| try_cow_to_string(v, lenient, "http.userAgent").transpose())
                        .transpose()?
                        .or_else(|| Some(crate::env::agent().into()));

                    Ok(Some(Box::new(opts)))
                }
            }
            File | Git | Ssh | Ext(_) => Ok(None),
        }
    }
}
