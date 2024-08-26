use gix_protocol::transport::client::Transport;

use crate::{
    bstr::BString,
    remote,
    remote::{
        fetch::{DryRun, RefMap},
        ref_map, Connection,
    },
    Progress,
};

mod error;
pub use error::Error;

use crate::remote::fetch::WritePackedRefs;

/// The way reflog messages should be composed whenever a ref is written with recent objects from a remote.
pub enum RefLogMessage {
    /// Prefix the log with `action` and generate the typical suffix as `git` would.
    Prefixed {
        /// The action to use, like `fetch` or `pull`.
        action: String,
    },
    /// Control the entire message, using `message` verbatim.
    Override {
        /// The complete reflog message.
        message: BString,
    },
}

impl RefLogMessage {
    pub(crate) fn compose(&self, context: &str) -> BString {
        match self {
            RefLogMessage::Prefixed { action } => format!("{action}: {context}").into(),
            RefLogMessage::Override { message } => message.to_owned(),
        }
    }
}

/// The status of the repository after the fetch operation
#[derive(Debug, Clone)]
pub enum Status {
    /// Nothing changed as the remote didn't have anything new compared to our tracking branches, thus no pack was received
    /// and no new object was added.
    ///
    /// As we could determine that nothing changed without remote interaction, there was no negotiation at all.
    NoPackReceived {
        /// If `true`, we didn't receive a pack due to dry-run mode being enabled.
        dry_run: bool,
        /// Information about the pack negotiation phase if negotiation happened at all.
        ///
        /// It's possible that negotiation didn't have to happen as no reference of interest changed on the server.
        negotiate: Option<outcome::Negotiate>,
        /// However, depending on the refspecs, references might have been updated nonetheless to point to objects as
        /// reported by the remote.
        update_refs: refs::update::Outcome,
    },
    /// There was at least one tip with a new object which we received.
    Change {
        /// Information about the pack negotiation phase.
        negotiate: outcome::Negotiate,
        /// Information collected while writing the pack and its index.
        write_pack_bundle: gix_pack::bundle::write::Outcome,
        /// Information collected while updating references.
        update_refs: refs::update::Outcome,
    },
}

/// The outcome of receiving a pack via [`Prepare::receive()`].
#[derive(Debug, Clone)]
pub struct Outcome {
    /// The result of the initial mapping of references, the prerequisite for any fetch.
    pub ref_map: RefMap,
    /// The status of the operation to indicate what happened.
    pub status: Status,
}

/// Additional types related to the outcome of a fetch operation.
pub mod outcome {
    /// Information about the negotiation phase of a fetch.
    ///
    /// Note that negotiation can happen even if no pack is ultimately produced.
    #[derive(Default, Debug, Clone)]
    pub struct Negotiate {
        /// The negotiation graph indicating what kind of information 'the algorithm' collected in the end.
        pub graph: gix_negotiate::IdMap,
        /// Additional information for each round of negotiation.
        pub rounds: Vec<negotiate::Round>,
    }

    ///
    pub mod negotiate {
        /// Key information about each round in the pack-negotiation.
        #[derive(Debug, Clone)]
        pub struct Round {
            /// The amount of `HAVE` lines sent this round.
            ///
            /// Each `HAVE` is an object that we tell the server about which would acknowledge each one it has as well.
            pub haves_sent: usize,
            /// A total counter, over all previous rounds, indicating how many `HAVE`s we sent without seeing a single acknowledgement,
            /// i.e. the indication of a common object.
            ///
            /// This number maybe zero or be lower compared to the previous round if we have received at least one acknowledgement.
            pub in_vain: usize,
            /// The amount of haves we should send in this round.
            ///
            /// If the value is lower than `haves_sent` (the `HAVE` lines actually sent), the negotiation algorithm has run out of options
            /// which typically indicates the end of the negotiation phase.
            pub haves_to_send: usize,
            /// If `true`, the server reported, as response to our previous `HAVE`s, that at least one of them is in common by acknowledging it.
            ///
            /// This may also lead to the server responding with a pack.
            pub previous_response_had_at_least_one_in_common: bool,
        }
    }
}

/// The progress ids used in during various steps of the fetch operation.
///
/// Note that tagged progress isn't very widely available yet, but support can be improved as needed.
///
/// Use this information to selectively extract the progress of interest in case the parent application has custom visualization.
#[derive(Debug, Copy, Clone)]
pub enum ProgressId {
    /// The progress name is defined by the remote and the progress messages it sets, along with their progress values and limits.
    RemoteProgress,
}

impl From<ProgressId> for gix_features::progress::Id {
    fn from(v: ProgressId) -> Self {
        match v {
            ProgressId::RemoteProgress => *b"FERP",
        }
    }
}

pub(crate) mod negotiate;

///
pub mod prepare {
    /// The error returned by [`prepare_fetch()`][super::Connection::prepare_fetch()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Cannot perform a meaningful fetch operation without any configured ref-specs")]
        MissingRefSpecs,
        #[error(transparent)]
        RefMap(#[from] crate::remote::ref_map::Error),
    }

    impl gix_protocol::transport::IsSpuriousError for Error {
        fn is_spurious(&self) -> bool {
            match self {
                Error::RefMap(err) => err.is_spurious(),
                _ => false,
            }
        }
    }
}

impl<'remote, 'repo, T> Connection<'remote, 'repo, T>
where
    T: Transport,
{
    /// Perform a handshake with the remote and obtain a ref-map with `options`, and from there one
    /// Note that at this point, the `transport` should already be configured using the [`transport_mut()`][Self::transport_mut()]
    /// method, as it will be consumed here.
    ///
    /// From there additional properties of the fetch can be adjusted to override the defaults that are configured via git-config.
    ///
    /// # Async Experimental
    ///
    /// Note that this implementation is currently limited correctly in blocking mode only as it relies on Drop semantics to close the connection
    /// should the fetch not be performed. Furthermore, there the code doing the fetch is inherently blocking and it's not offloaded to a thread,
    /// making this call block the executor.
    /// It's best to unblock it by placing it into its own thread or offload it should usage in an async context be truly required.
    #[allow(clippy::result_large_err)]
    #[gix_protocol::maybe_async::maybe_async]
    pub async fn prepare_fetch(
        mut self,
        progress: impl Progress,
        options: ref_map::Options,
    ) -> Result<Prepare<'remote, 'repo, T>, prepare::Error> {
        if self.remote.refspecs(remote::Direction::Fetch).is_empty() {
            return Err(prepare::Error::MissingRefSpecs);
        }
        let ref_map = self.ref_map_inner(progress, options).await?;
        Ok(Prepare {
            con: Some(self),
            ref_map,
            dry_run: DryRun::No,
            reflog_message: None,
            write_packed_refs: WritePackedRefs::Never,
            shallow: Default::default(),
        })
    }
}

impl<'remote, 'repo, T> Prepare<'remote, 'repo, T>
where
    T: Transport,
{
    /// Return the `ref_map` (that includes the server handshake) which was part of listing refs prior to fetching a pack.
    pub fn ref_map(&self) -> &RefMap {
        &self.ref_map
    }
}

mod config;
mod receive_pack;
///
#[path = "update_refs/mod.rs"]
pub mod refs;

/// A structure to hold the result of the handshake with the remote and configure the upcoming fetch operation.
pub struct Prepare<'remote, 'repo, T>
where
    T: Transport,
{
    con: Option<Connection<'remote, 'repo, T>>,
    ref_map: RefMap,
    dry_run: DryRun,
    reflog_message: Option<RefLogMessage>,
    write_packed_refs: WritePackedRefs,
    shallow: remote::fetch::Shallow,
}

/// Builder
impl<'remote, 'repo, T> Prepare<'remote, 'repo, T>
where
    T: Transport,
{
    /// If dry run is enabled, no change to the repository will be made.
    ///
    /// This works by not actually fetching the pack after negotiating it, nor will refs be updated.
    pub fn with_dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = if enabled { DryRun::Yes } else { DryRun::No };
        self
    }

    /// If enabled, don't write ref updates to loose refs, but put them exclusively to packed-refs.
    ///
    /// This improves performance and allows case-sensitive filesystems to deal with ref names that would otherwise
    /// collide.
    pub fn with_write_packed_refs_only(mut self, enabled: bool) -> Self {
        self.write_packed_refs = if enabled {
            WritePackedRefs::Only
        } else {
            WritePackedRefs::Never
        };
        self
    }

    /// Set the reflog message to use when updating refs after fetching a pack.
    pub fn with_reflog_message(mut self, reflog_message: RefLogMessage) -> Self {
        self.reflog_message = reflog_message.into();
        self
    }

    /// Define what to do when the current repository is a shallow clone.
    ///
    /// *Has no effect if the current repository is not as shallow clone.*
    pub fn with_shallow(mut self, shallow: remote::fetch::Shallow) -> Self {
        self.shallow = shallow;
        self
    }
}

impl<'remote, 'repo, T> Drop for Prepare<'remote, 'repo, T>
where
    T: Transport,
{
    fn drop(&mut self) {
        if let Some(mut con) = self.con.take() {
            #[cfg(feature = "async-network-client")]
            {
                // TODO: this should be an async drop once the feature is available.
                //       Right now we block the executor by forcing this communication, but that only
                //       happens if the user didn't actually try to receive a pack, which consumes the
                //       connection in an async context.
                gix_protocol::futures_lite::future::block_on(gix_protocol::indicate_end_of_interaction(
                    &mut con.transport,
                    con.trace,
                ))
                .ok();
            }
            #[cfg(not(feature = "async-network-client"))]
            {
                gix_protocol::indicate_end_of_interaction(&mut con.transport, con.trace).ok();
            }
        }
    }
}
