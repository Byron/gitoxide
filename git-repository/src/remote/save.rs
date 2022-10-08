use crate::Remote;
use std::convert::TryInto;

/// The error returned by [`Remote::save_to()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The remote pointing to {} is anonymous and can't be saved.", url.to_bstring())]
    NameMissing { url: git_url::Url },
}

/// Serialize into git-config.
impl Remote<'_> {
    /// Save ourselves to the given `config` if we are a named remote or fail otherwise.
    ///
    /// Note that all sections named `remote "<name>"` will be cleared of all values we are about to write,
    /// and the last `remote "<name>"` section will be containing all relevant values so that reloading the remote
    /// from `config` would yield the same in-memory state.
    pub fn save_to(&self, config: &mut git_config::File<'static>) -> Result<(), Error> {
        let name = self.name().ok_or_else(|| Error::NameMissing {
            url: self
                .url
                .as_ref()
                .or(self.push_url.as_ref())
                .expect("one url is always set")
                .to_owned(),
        })?;
        let mut section = config
            .section_mut_or_create_new("remote", Some(name))
            .expect("section name is validated and 'remote' is acceptable");
        if let Some(url) = self.url.as_ref() {
            section.push("url".try_into().expect("valid"), Some(url.to_bstring().as_ref()))
        }
        if let Some(url) = self.push_url.as_ref() {
            section.push("pushurl".try_into().expect("valid"), Some(url.to_bstring().as_ref()))
        }
        for (key, spec) in self
            .fetch_specs
            .iter()
            .map(|spec| ("fetch", spec))
            .chain(self.push_specs.iter().map(|spec| ("push", spec)))
        {
            section.push(
                key.try_into().expect("valid"),
                Some(spec.to_ref().to_bstring().as_ref()),
            )
        }
        Ok(())
    }

    /// Forcefully set our name to `name` and write our state to `config` similar to [`save_to()`][Self::save_to()].
    ///
    /// Note that this sets a name for anonymous remotes, but overwrites the name for those who were named before.
    /// If this name is different from the current one, the git configuration will still contain the previous name,
    /// and the caller should account for that.
    pub fn save_as_to(
        &mut self,
        name: git_config::parse::section::Name<'_>,
        config: &mut git_config::File<'static>,
    ) -> Result<(), Error> {
        let prev_name = self.name.take();
        self.name = Some(name.to_string());
        self.save_to(config).map_err(|err| {
            self.name = prev_name;
            err
        })
    }
}
