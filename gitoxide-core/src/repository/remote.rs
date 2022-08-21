#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod net {
    use crate::OutputFormat;
    use git_repository as git;

    #[git::protocol::maybe_async::maybe_async]
    pub async fn refs(
        _repo: git::Repository,
        _name: &str,
        _format: OutputFormat,
        _progress: impl git::Progress,
        _out: impl std::io::Write,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub use net::refs;
