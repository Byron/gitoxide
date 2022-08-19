use crate::remote::{Connection, Direction};
use git_protocol::transport::client::Transport;

mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Handshake(#[from] git_protocol::fetch::handshake::Error),
        #[error(transparent)]
        ListRefs(#[from] git_protocol::fetch::refs::Error),
    }
}
use crate::remote::connection::State;
pub use error::Error;
use git_features::progress::Progress;

impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P>
where
    T: Transport,
    P: Progress,
{
    /// List all references on the remote.
    ///
    /// Note that this doesn't fetch the objects mentioned in the tips nor does it make any change to underlying repository.
    ///
    /// This method is idempotent and only runs once.
    #[git_protocol::maybe_async::maybe_async]
    pub async fn list_refs(&mut self) -> Result<&[git_protocol::fetch::Ref], Error> {
        match self.state {
            State::Connected => {
                let mut outcome = git_protocol::fetch::handshake(
                    &mut self.transport,
                    git_protocol::credentials::helper,
                    Vec::new(),
                    &mut self.progress,
                )
                .await?;
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
                self.state = State::HandshakeWithRefs { outcome, refs };
                match &self.state {
                    State::HandshakeWithRefs { refs, .. } => Ok(refs),
                    _ => unreachable!(),
                }
            }
            State::HandshakeWithRefs { ref refs, .. } => Ok(refs),
        }
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
