use super::Error;
use crate::Repository;
use std::convert::TryInto;

pub fn index_threads(repo: &Repository) -> Result<Option<usize>, Error> {
    let lenient_config = repo.options.lenient_config;
    let message = "The configured pack.threads is invalid. It must be 0 or greater, with 0 turning it off";
    Ok(
        match repo
            .config
            .resolved
            .integer_filter("pack", None, "threads", &mut repo.filter_config_section())
            .transpose()
        {
            Ok(Some(0)) => Some(1),
            Ok(Some(n)) => match n.try_into() {
                Ok(n) => Some(n),
                Err(_) if lenient_config => None,
                Err(_) => {
                    return Err(Error::Configuration {
                        message: "The value for pack.threads is out of range",
                        desired: n.into(),
                        source: None,
                    })
                }
            },
            Ok(None) => None,
            Err(_) if lenient_config => None,
            Err(err) => {
                return Err(Error::Configuration {
                    message,
                    desired: None,
                    source: err.into(),
                })
            }
        },
    )
}

pub fn pack_index_version(repo: &Repository) -> Result<git_pack::index::Version, Error> {
    use git_pack::index::Version;
    let lenient_config = repo.options.lenient_config;
    let message = "The configured pack.indexVersion is invalid. It must be 1 or 2, with 2 being the default";
    Ok(
        match repo.config.resolved.integer("pack", None, "indexVersion").transpose() {
            Ok(Some(v)) if v == 1 => Version::V1,
            Ok(Some(v)) if v == 2 => Version::V2,
            Ok(None) => Version::V2,
            Ok(Some(_)) | Err(_) if lenient_config => Version::V2,
            Ok(Some(v)) => {
                return Err(Error::Configuration {
                    message,
                    desired: v.into(),
                    source: None,
                })
            }
            Err(err) => {
                return Err(Error::Configuration {
                    message,
                    desired: None,
                    source: err.into(),
                })
            }
        },
    )
}
