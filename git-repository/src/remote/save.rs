use crate::Remote;

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
    pub fn save_to(&self, _config: &mut git_config::File<'static>) -> Result<(), Error> {
        let _name = self.name().ok_or_else(|| Error::NameMissing {
            url: self
                .url
                .as_ref()
                .or(self.push_url.as_ref())
                .expect("one url is always set")
                .to_owned(),
        })?;
        todo!()
    }

    /// Forcefully set our name to `name` and write our state to `config` similar to [`save_to()`][Self::save_to()].
    ///
    /// Note that this sets a name for anonymous remotes, but overwrites the name for those who were named before.
    /// If this name is different from the current one, the git configuration will still contain the previous name,
    /// and the caller should account for that.
    pub fn save_as(
        &mut self,
        name: git_config::parse::section::Name<'_>,
        config: &mut git_config::File<'static>,
    ) -> Result<(), Error> {
        self.name = Some(name.to_string());
        self.save_to(config)
    }
}
