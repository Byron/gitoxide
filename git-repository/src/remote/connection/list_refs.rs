use git_features::progress::Progress;
use git_protocol::transport::client::Transport;

use crate::remote::{connection::HandshakeWithRefs, fetch, Connection, Direction};

mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Handshake(#[from] git_protocol::fetch::handshake::Error),
        #[error(transparent)]
        ListRefs(#[from] git_protocol::fetch::refs::Error),
        #[error(transparent)]
        Transport(#[from] git_protocol::transport::client::Error),
        #[error(transparent)]
        ConfigureCredentials(#[from] crate::config::credential_helpers::Error),
    }
}
pub use error::Error;

impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P>
where
    T: Transport,
    P: Progress,
{
    /// List all references on the remote.
    ///
    /// Note that this doesn't fetch the objects mentioned in the tips nor does it make any change to underlying repository.
    #[git_protocol::maybe_async::maybe_async]
    pub async fn list_refs(mut self) -> Result<Vec<git_protocol::fetch::Ref>, Error> {
        let res = self.fetch_refs().await?;
        git_protocol::fetch::indicate_end_of_interaction(&mut self.transport).await?;
        Ok(res.refs)
    }

    /// A mapping showing the objects available in refs matching our ref-specs on the remote side, along with their destination
    /// ref locally, if set and if there are no conflicts.
    #[git_protocol::maybe_async::maybe_async]
    pub async fn ref_mapping(self) -> Result<Vec<fetch::Mapping>, Error> {
        todo!()
    }

    #[git_protocol::maybe_async::maybe_async]
    async fn fetch_refs(&mut self) -> Result<HandshakeWithRefs, Error> {
        let mut credentials_storage;
        let authenticate = match self.credentials.as_mut() {
            Some(f) => f,
            None => {
                let url = self
                    .remote
                    .url(Direction::Fetch)
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| {
                        git_url::parse(self.transport.to_url().as_bytes().into())
                            .expect("valid URL to be provided by transport")
                    });
                credentials_storage = self.configured_credentials(url)?;
                &mut credentials_storage
            }
        };
        let mut outcome =
            git_protocol::fetch::handshake(&mut self.transport, authenticate, Vec::new(), &mut self.progress).await?;
        let refs = match outcome.refs.take() {
            Some(refs) => refs,
            None => {
                git_protocol::fetch::refs(
                    &mut self.transport,
                    outcome.server_protocol_version,
                    &outcome.capabilities,
                    |_a, _b, _c| Ok(git_protocol::fetch::delegate::LsRefsAction::Continue),
                    &mut self.progress,
                )
                .await?
            }
        };
        Ok(HandshakeWithRefs { outcome, refs })
    }

    /// List all references on the remote that have been filtered through our remote's [`refspecs`][crate::Remote::refspecs()]
    /// for _fetching_ or _pushing_ depending on `direction`.
    ///
    /// This comes in the form of information of all matching tips on the remote and the object they point to, along with
    /// with the local tracking branch of these tips (if available).
    ///
    /// Note that this doesn't fetch the objects mentioned in the tips nor does it make any change to underlying repository.
    pub fn list_refs_by_refspec(&mut self, _direction: Direction) -> ! {
        todo!()
    }
}
