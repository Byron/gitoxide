use git_features::progress::Progress;
use git_protocol::transport::client::Transport;

use crate::remote::{connection::HandshakeWithRefs, Connection, Direction};

/// The error returned by [`Connection::list_refs()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
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
        let res = self.fetch_refs().await;
        git_protocol::fetch::indicate_end_of_interaction(&mut self.transport).await?;
        Ok(res?.refs)
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
}

///
pub mod to_map {
    use crate::remote::{fetch, Connection};
    use git_features::progress::Progress;
    use git_protocol::transport::client::Transport;

    /// The error returned by [`Connection::list_refs_to_map()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        ListRefs(#[from] crate::remote::list_refs::Error),
        #[error(transparent)]
        MappingValidation(#[from] git_refspec::match_group::validate::Error),
    }

    impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P>
    where
        T: Transport,
        P: Progress,
    {
        /// List all references on the remote that have been filtered through our remote's [`refspecs`][crate::Remote::refspecs()]
        /// for _fetching_.
        ///
        /// This comes in the form of all matching tips on the remote and the object they point to, along with
        /// with the local tracking branch of these tips (if available).
        ///
        /// Note that this doesn't fetch the objects mentioned in the tips nor does it make any change to underlying repository.
        #[git_protocol::maybe_async::maybe_async]
        pub async fn list_refs_to_map(mut self) -> Result<fetch::RefMap, Error> {
            let res = self.ref_map().await;
            git_protocol::fetch::indicate_end_of_interaction(&mut self.transport)
                .await
                .map_err(|err| Error::ListRefs(crate::remote::list_refs::Error::Transport(err)))?;
            res
        }

        #[git_protocol::maybe_async::maybe_async]
        async fn ref_map(&mut self) -> Result<fetch::RefMap, Error> {
            let remote = self.fetch_refs().await?;
            let group = git_refspec::MatchGroup::from_fetch_specs(self.remote.fetch_specs.iter().map(|s| s.to_ref()));
            let (res, fixes) = group
                .match_remotes(remote.refs.iter().map(|r| {
                    let (full_ref_name, target, object) = r.unpack();
                    git_refspec::match_group::Item {
                        full_ref_name,
                        target,
                        object,
                    }
                }))
                .validated()?;
            let mappings = res.mappings;
            let mappings = mappings
                .into_iter()
                .map(|m| fetch::Mapping {
                    remote: m
                        .item_index
                        .map(|idx| fetch::Source::Ref(remote.refs[idx].clone()))
                        .unwrap_or_else(|| {
                            fetch::Source::ObjectId(match m.lhs {
                                git_refspec::match_group::SourceRef::ObjectId(id) => id,
                                _ => unreachable!("no item index implies having an object id"),
                            })
                        }),
                    local: m.rhs.map(|c| c.into_owned()),
                })
                .collect();
            Ok(fetch::RefMap { mappings, fixes })
        }
    }
}
