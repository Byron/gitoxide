use crate::bstr::{BStr, ByteVec};
use git_transport::client::http;
use std::any::Any;
use std::convert::{TryFrom, TryInto};

impl crate::Repository {
    /// Produce configuration suitable for `url`, as differentiated by its protocol/scheme, to be passed to a transport instance via
    /// [configure()][git_transport::client::TransportWithoutIO::configure()].
    /// `None` is returned if there is no known configuration.
    ///
    /// Note that the caller may cast the instance themselves to modify it before passing it on.
    pub fn transport_config<'a>(
        &self,
        url: impl Into<&'a BStr>,
    ) -> Result<Option<Box<dyn Any>>, crate::config::transport::Error> {
        let url = git_url::parse(url.into())?;
        use git_url::Scheme::*;

        match &url.scheme {
            Http | Https => {
                let mut opts = http::Options::default();
                let config = &self.config.resolved;
                let mut trusted_only = self.filter_config_section();
                opts.extra_headers = config
                    .strings_filter("http", None, "extraHeader", &mut trusted_only)
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|v| Vec::from(v.into_owned()).into_string().ok())
                    .collect();

                if let Some(follow_redirects) = config.string_filter("http", None, "followRedirects", &mut trusted_only)
                {
                    opts.follow_redirects = if follow_redirects.as_ref() == "initial" {
                        http::options::FollowRedirects::Initial
                    } else if git_config::Boolean::try_from(follow_redirects)
                        .map_err(|err| crate::config::transport::Error::ConfigValue {
                            source: err,
                            key: "http.followRedirects",
                        })?
                        .0
                    {
                        http::options::FollowRedirects::All
                    } else {
                        http::options::FollowRedirects::None
                    };
                }

                opts.low_speed_time_seconds = integer(config, "http.lowSpeedTime", "u64", trusted_only)?;
                opts.low_speed_limit_bytes_per_second = integer(config, "http.lowSpeedLimit", "u32", trusted_only)?;
                todo!();
            }
            File | Git | Ssh | Ext(_) => Ok(None),
        }
    }
}

fn integer<T>(
    config: &git_config::File<'static>,
    key: &'static str,
    kind: &'static str,
    mut filter: fn(&git_config::file::Metadata) -> bool,
) -> Result<T, crate::config::transport::Error>
where
    T: TryFrom<i64>,
{
    let git_config::parse::Key {
        section_name,
        value_name,
        ..
    } = git_config::parse::key(key).expect("valid key statically known");
    let integer = config
        .integer_filter(section_name, None, value_name, &mut filter)
        .transpose()
        .map_err(|err| crate::config::transport::Error::ConfigValue {
            source: err,
            key: "http.lowSpeedTime",
        })?
        .unwrap_or_default();
    Ok(integer
        .try_into()
        .map_err(|_| crate::config::transport::Error::InvalidInteger {
            actual: integer,
            key,
            kind,
        })?)
}
