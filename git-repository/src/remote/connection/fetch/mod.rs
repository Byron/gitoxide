use git_protocol::transport::client::Transport;

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
            RefLogMessage::Prefixed { action } => format!("{}: {}", action, context).into(),
            RefLogMessage::Override { message } => message.to_owned(),
        }
    }
}

/// The status of the repository after the fetch operation
#[derive(Debug, Clone)]
pub enum Status {
    /// Nothing changed as the remote didn't have anything new compared to our tracking branches, thus no pack was received
    /// and no new object was added.
    NoPackReceived {
        /// However, depending on the refspecs, references might have been updated nonetheless to point to objects as
        /// reported by the remote.
        update_refs: refs::update::Outcome,
    },
    /// There was at least one tip with a new object which we received.
    Change {
        /// Information collected while writing the pack and its index.
        write_pack_bundle: git_pack::bundle::write::Outcome,
        /// Information collected while updating references.
        update_refs: refs::update::Outcome,
    },
    /// A dry run was performed which leaves the local repository without any change
    /// nor will a pack have been received.
    DryRun {
        /// Information about what updates to refs would have been done.
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

///
pub mod negotiate;

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

    impl git_protocol::transport::IsSpuriousError for Error {
        fn is_spurious(&self) -> bool {
            match self {
                Error::RefMap(err) => err.is_spurious(),
                _ => false,
            }
        }
    }
}

impl<'remote, 'repo, T, P> Connection<'remote, 'repo, T, P>
where
    T: Transport,
    P: Progress,
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
    #[git_protocol::maybe_async::maybe_async]
    pub async fn prepare_fetch(
        mut self,
        options: ref_map::Options,
    ) -> Result<Prepare<'remote, 'repo, T, P>, prepare::Error> {
        if self.remote.refspecs(remote::Direction::Fetch).is_empty() {
            return Err(prepare::Error::MissingRefSpecs);
        }
        let ref_map = self.ref_map_inner(options).await?;
        Ok(Prepare {
            con: Some(self),
            ref_map,
            dry_run: DryRun::No,
            reflog_message: None,
            write_packed_refs: WritePackedRefs::Never,
        })
    }
}

impl<'remote, 'repo, T, P> Prepare<'remote, 'repo, T, P>
where
    T: Transport,
{
    /// Return the ref_map (that includes the server handshake) which was part of listing refs prior to fetching a pack.
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
pub struct Prepare<'remote, 'repo, T, P>
where
    T: Transport,
{
    con: Option<Connection<'remote, 'repo, T, P>>,
    ref_map: RefMap,
    dry_run: DryRun,
    reflog_message: Option<RefLogMessage>,
    write_packed_refs: WritePackedRefs,
}

/// Builder
impl<'remote, 'repo, T, P> Prepare<'remote, 'repo, T, P>
where
    T: Transport,
{
    /// If dry run is enabled, no change to the repository will be made.
    ///
    /// This works by not actually fetching the pack after negotiating it, nor will refs be updated.
    pub fn with_dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = enabled.then(|| DryRun::Yes).unwrap_or(DryRun::No);
        self
    }

    /// If enabled, don't write ref updates to loose refs, but put them exclusively to packed-refs.
    ///
    /// This improves performance and allows case-sensitive filesystems to deal with ref names that would otherwise
    /// collide.
    pub fn with_write_packed_refs_only(mut self, enabled: bool) -> Self {
        self.write_packed_refs = enabled.then(|| WritePackedRefs::Only).unwrap_or(WritePackedRefs::Never);
        self
    }

    /// Set the reflog message to use when updating refs after fetching a pack.
    pub fn with_reflog_message(mut self, reflog_message: RefLogMessage) -> Self {
        self.reflog_message = reflog_message.into();
        self
    }
}

impl<'remote, 'repo, T, P> Drop for Prepare<'remote, 'repo, T, P>
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
                git_protocol::futures_lite::future::block_on(git_protocol::indicate_end_of_interaction(
                    &mut con.transport,
                ))
                .ok();
            }
            #[cfg(not(feature = "async-network-client"))]
            {
                git_protocol::indicate_end_of_interaction(&mut con.transport).ok();
            }
        }
    }
}
