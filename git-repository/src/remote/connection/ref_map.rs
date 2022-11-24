use std::collections::HashSet;

use git_features::progress::Progress;
use git_protocol::transport::client::Transport;

use crate::{
    bstr,
    bstr::{BString, ByteVec},
    remote::{connection::HandshakeWithRefs, fetch, Connection, Direction},
};

/// The error returned by [`Connection::ref_map()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Failed to configure the transport before connecting to {url:?}")]
    GatherTransportConfig {
        url: BString,
        source: crate::config::transport::Error,
    },
    #[error("Failed to configure the transport layer")]
    ConfigureTransport(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    Handshake(#[from] git_protocol::handshake::Error),
    #[error("The object format {format:?} as used by the remote is unsupported")]
    UnknownObjectFormat { format: BString },
    #[error(transparent)]
    ListRefs(#[from] git_protocol::ls_refs::Error),
    #[error(transparent)]
    Transport(#[from] git_protocol::transport::client::Error),
    #[error(transparent)]
    ConfigureCredentials(#[from] crate::config::credential_helpers::Error),
    #[error(transparent)]
    MappingValidation(#[from] git_refspec::match_group::validate::Error),
}

/// For use in [`Connection::ref_map()`].
#[derive(Debug, Clone)]
pub struct Options {
    /// Use a two-component prefix derived from the ref-spec's source, like `refs/heads/`  to let the server pre-filter refs
    /// with great potential for savings in traffic and local CPU time. Defaults to `true`.
    pub prefix_from_spec_as_filter_on_remote: bool,
    /// Parameters in the form of `(name, optional value)` to add to the handshake.
    ///
    /// This is useful in case of custom servers.
    pub handshake_parameters: Vec<(String, Option<String>)>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            prefix_from_spec_as_filter_on_remote: true,
            handshake_parameters: Default::default(),
        }
    }
}

impl<'remote, 'repo, T, P> Connection<'remote, 'repo, T, P>
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
    ///
    /// # Consumption
    ///
    /// Due to management of the transport, it's cleanest to only use it for a single interaction. Thus it's consumed along with
    /// the connection.
    ///
    /// ### Configuration
    ///
    /// - `gitoxide.userAgent` is read to obtain the application user agent for git servers and for HTTP servers as well.
    #[allow(clippy::result_large_err)]
    #[git_protocol::maybe_async::maybe_async]
    pub async fn ref_map(mut self, options: Options) -> Result<fetch::RefMap, Error> {
        let res = self.ref_map_inner(options).await;
        git_protocol::indicate_end_of_interaction(&mut self.transport)
            .await
            .ok();
        res
    }

    #[allow(clippy::result_large_err)]
    #[git_protocol::maybe_async::maybe_async]
    pub(crate) async fn ref_map_inner(
        &mut self,
        Options {
            prefix_from_spec_as_filter_on_remote,
            handshake_parameters,
        }: Options,
    ) -> Result<fetch::RefMap, Error> {
        let null = git_hash::ObjectId::null(git_hash::Kind::Sha1); // OK to hardcode Sha1, it's not supposed to match, ever.
        let remote = self
            .fetch_refs(prefix_from_spec_as_filter_on_remote, handshake_parameters)
            .await?;
        let group = git_refspec::MatchGroup::from_fetch_specs(self.remote.fetch_specs.iter().map(|s| s.to_ref()));
        let (res, fixes) = group
            .match_remotes(remote.refs.iter().map(|r| {
                let (full_ref_name, target, object) = r.unpack();
                git_refspec::match_group::Item {
                    full_ref_name,
                    target: target.unwrap_or(&null),
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
                spec_index: m.spec_index,
            })
            .collect();

        let object_hash = extract_object_format(self.remote.repo, &remote.outcome)?;
        Ok(fetch::RefMap {
            mappings,
            fixes,
            remote_refs: remote.refs,
            handshake: remote.outcome,
            object_hash,
        })
    }

    #[allow(clippy::result_large_err)]
    #[git_protocol::maybe_async::maybe_async]
    async fn fetch_refs(
        &mut self,
        filter_by_prefix: bool,
        extra_parameters: Vec<(String, Option<String>)>,
    ) -> Result<HandshakeWithRefs, Error> {
        let mut credentials_storage;
        let url = self.transport.to_url();
        let authenticate = match self.authenticate.as_mut() {
            Some(f) => f,
            None => {
                let url = self
                    .remote
                    .url(Direction::Fetch)
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| git_url::parse(url.as_ref()).expect("valid URL to be provided by transport"));
                credentials_storage = self.configured_credentials(url)?;
                &mut credentials_storage
            }
        };

        if self.transport_options.is_none() {
            self.transport_options = self
                .remote
                .repo
                .transport_options(url.as_ref(), self.remote.name().map(|n| n.as_bstr()))
                .map_err(|err| Error::GatherTransportConfig {
                    source: err,
                    url: url.into_owned(),
                })?;
        }
        if let Some(config) = self.transport_options.as_ref() {
            self.transport.configure(&**config)?;
        }
        let mut outcome =
            git_protocol::fetch::handshake(&mut self.transport, authenticate, extra_parameters, &mut self.progress)
                .await?;
        let refs = match outcome.refs.take() {
            Some(refs) => refs,
            None => {
                let specs = &self.remote.fetch_specs;
                let agent_feature = self.remote.repo.config.user_agent_tuple();
                git_protocol::ls_refs(
                    &mut self.transport,
                    &outcome.capabilities,
                    move |_capabilities, arguments, features| {
                        features.push(agent_feature);
                        if filter_by_prefix {
                            let mut seen = HashSet::new();
                            for spec in specs {
                                let spec = spec.to_ref();
                                if seen.insert(spec.instruction()) {
                                    let mut prefixes = Vec::with_capacity(1);
                                    spec.expand_prefixes(&mut prefixes);
                                    for mut prefix in prefixes {
                                        prefix.insert_str(0, "ref-prefix ");
                                        arguments.push(prefix);
                                    }
                                }
                            }
                        }
                        Ok(git_protocol::ls_refs::Action::Continue)
                    },
                    &mut self.progress,
                )
                .await?
            }
        };
        Ok(HandshakeWithRefs { outcome, refs })
    }
}

/// Assume sha1 if server says nothing, otherwise configure anything beyond sha1 in the local repo configuration
#[allow(clippy::result_large_err)]
fn extract_object_format(
    _repo: &crate::Repository,
    outcome: &git_protocol::handshake::Outcome,
) -> Result<git_hash::Kind, Error> {
    use bstr::ByteSlice;
    let object_hash =
        if let Some(object_format) = outcome.capabilities.capability("object-format").and_then(|c| c.value()) {
            let object_format = object_format.to_str().map_err(|_| Error::UnknownObjectFormat {
                format: object_format.into(),
            })?;
            match object_format {
                "sha1" => git_hash::Kind::Sha1,
                unknown => return Err(Error::UnknownObjectFormat { format: unknown.into() }),
            }
        } else {
            git_hash::Kind::Sha1
        };
    Ok(object_hash)
}
