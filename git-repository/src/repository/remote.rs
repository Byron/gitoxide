use crate::{remote, Remote};

impl crate::Repository {
    /// Find the remote with the given `name` or report an error, similar to [`try_find_remote(…)`][Self::try_find_remote()].
    pub fn find_remote(&self, name: &str) -> Result<Remote<'_>, remote::find::existing::Error> {
        Ok(self
            .try_find_remote(name)
            .ok_or_else(|| remote::find::existing::Error::NotFound { name: name.into() })??)
    }

    /// Find the remote with the given `name` or return `None` if it doesn't exist.
    pub fn try_find_remote(&self, _name: &str) -> Option<Result<Remote<'_>, remote::find::Error>> {
        todo!()
    }
}
