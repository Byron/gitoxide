use crate::IsActivePlatform;
use bstr::BStr;

/// The error returned by [File::names_and_active_state](crate::File::names_and_active_state()).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    NormalizePattern(#[from] gix_pathspec::normalize::Error),
    #[error(transparent)]
    ParsePattern(#[from] gix_pathspec::parse::Error),
}

///
pub mod is_active {
    /// The error returned by the iterator of [File::names_and_active_state](crate::File::names_and_active_state()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The value of the 'active' field of a submodule could not be decoded")]
        ActiveField(#[from] gix_config::value::Error),
        #[error(transparent)]
        Url(#[from] crate::config::url::Error),
    }
}

impl IsActivePlatform {
    /// Returns `true` if the submodule named `name` is active or `false` otherwise.
    /// `modules` is the instance that [is_active_platform()](crate::File::is_active_platform()) was called on, and
    /// `config` is the configuration that was passed there as well.
    /// `attributes(relative_path, case, is_dir, outcome)` provides a way to resolve the attributes mentioned in `submodule.active` pathspecs
    /// that are evaluated in the platforms git configuration.
    ///
    /// A submodule's active state is determined in the following order
    ///
    /// * it's `submodule.<name>.active` configuration is set
    /// * it matches a `submodule.active` pathspec either positively or negatively via `:!<spec>`
    /// * it's active if it has a `url`
    pub fn is_active(
        &mut self,
        modules: &crate::File,
        config: &gix_config::File<'static>,
        name: &BStr,
        attributes: impl FnMut(
            &BStr,
            gix_pathspec::attributes::glob::pattern::Case,
            bool,
            &mut gix_pathspec::attributes::search::Outcome,
        ) -> bool,
    ) -> Result<bool, is_active::Error> {
        if let Some(val) = config.boolean("submodule", Some(name), "active").transpose()? {
            return Ok(val);
        };
        if let Some(val) = self
            .search
            .as_mut()
            .and_then(|search| search.pattern_matching_relative_path(name, Some(true), attributes))
            .map(|m| !m.is_excluded())
        {
            return Ok(val);
        }
        Ok(match modules.url(name) {
            Ok(_) => true,
            Err(crate::config::url::Error::Missing { .. }) => false,
            Err(err) => return Err(err.into()),
        })
    }
}
