/// The error returned by [`Repository::attributes()`][crate::Repository::attributes()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    ConfigureAttributes(#[from] crate::config::attribute_stack::Error),
    #[error(transparent)]
    ConfigureExcludes(#[from] crate::config::exclude_stack::Error),
}
