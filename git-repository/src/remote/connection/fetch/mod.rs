use std::sync::atomic::AtomicBool;

use git_odb::FindExt;
use git_protocol::transport::client::Transport;

use crate::{
    remote,
    remote::{
        fetch,
        fetch::{DryRun, RefMap},
        ref_map, Connection,
    },
    Progress,
};

mod error {
    /// The error returned by [`receive()`](super::Prepare::receive()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{message}{}", desired.map(|n| format!(" (got {})", n)).unwrap_or_default())]
        Configuration {
            message: &'static str,
            desired: Option<i64>,
            source: Option<git_config::value::Error>,
        },
        #[error("Could not decode server reply")]
        FetchResponse(#[from] git_protocol::fetch::response::Error),
        #[error(transparent)]
        Negotiate(#[from] super::negotiate::Error),
        #[error(transparent)]
        Client(#[from] git_protocol::transport::client::Error),
        #[error(transparent)]
        WritePack(#[from] git_pack::bundle::write::Error),
        #[error(transparent)]
        UpdateRefs(#[from] super::refs::update::Error),
    }
}
pub use error::Error;

/// The status of the repository after the fetch operation
#[derive(Debug, Clone)]
pub enum Status {
    /// Nothing changed as the remote didn't have anything new compared to our tracking branches.
    NoChange,
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
    /// # Blocking Only
    ///
    /// Note that this implementation is currently limited to blocking mode as it relies on Drop semantics to close the connection
    /// should the fetch not be performed. Furthermore, there the code doing the fetch is inherently blocking so there is no benefit.
    /// It's best to unblock it by placing it into its own thread or offload it should usage in an async context be required.
    pub fn prepare_fetch(mut self, options: ref_map::Options) -> Result<Prepare<'remote, 'repo, T, P>, prepare::Error> {
        if self.remote.refspecs(remote::Direction::Fetch).is_empty() {
            return Err(prepare::Error::MissingRefSpecs);
        }
        let ref_map = self.ref_map_inner(options)?;
        Ok(Prepare {
            con: Some(self),
            ref_map,
            dry_run: DryRun::No,
        })
    }
}

impl<'remote, 'repo, T, P> Prepare<'remote, 'repo, T, P>
where
    T: Transport,
    P: Progress,
{
    /// Receive the pack and perform the operation as configured by git via `git-config` or overridden by various builder methods.
    /// Return `Ok(None)` if there was nothing to do because all remote refs are at the same state as they are locally, or `Ok(Some(outcome))`
    /// to inform about all the changes that were made.
    ///
    /// ### Negotiation
    ///
    /// "fetch.negotiationAlgorithm" describes algorithms `git` uses currently, with the default being `consecutive` and `skipping` being
    /// experimented with. We currently implement something we could call 'naive' which works for now.
    ///
    ///
    /// ### Deviation
    ///
    /// When **updating refs**, the `git-fetch` docs state that the following:
    ///
    /// > Unlike when pushing with git-push, any updates outside of refs/{tags,heads}/* will be accepted without + in the refspec (or --force), whether that’s swapping e.g. a tree object for a blob, or a commit for another commit that’s doesn’t have the previous commit as an ancestor etc.
    ///
    /// We explicitly don't special case those refs and expect the user to take control. Note that by its nature,
    /// force only applies to refs pointing to commits and if they don't, they will be updated either way in our
    /// implementation as well.
    pub fn receive(mut self, should_interrupt: &AtomicBool) -> Result<Outcome, Error> {
        let mut con = self.con.take().expect("receive() can only be called once");

        let handshake = &self.ref_map.handshake;
        let protocol_version = handshake.server_protocol_version;

        let fetch = git_protocol::fetch::Command::Fetch;
        let fetch_features = fetch.default_features(protocol_version, &handshake.capabilities);

        git_protocol::fetch::Response::check_required_features(protocol_version, &fetch_features)?;
        let sideband_all = fetch_features.iter().any(|(n, _)| *n == "sideband-all");
        let mut arguments = git_protocol::fetch::Arguments::new(protocol_version, fetch_features);
        let mut previous_response = None::<git_protocol::fetch::Response>;
        let mut round = 1;
        let progress = &mut con.progress;
        let repo = con.remote.repo;

        let reader = 'negotiation: loop {
            progress.step();
            progress.set_name(format!("negotiate (round {})", round));

            let is_done = match negotiate::one_round(
                negotiate::Algorithm::Naive,
                round,
                repo,
                &self.ref_map,
                &mut arguments,
                previous_response.as_ref(),
            ) {
                Ok(_) if arguments.is_empty() => {
                    git_protocol::fetch::indicate_end_of_interaction(&mut con.transport).ok();
                    return Ok(Outcome {
                        ref_map: std::mem::take(&mut self.ref_map),
                        status: Status::NoChange,
                    });
                }
                Ok(is_done) => is_done,
                Err(err) => {
                    git_protocol::fetch::indicate_end_of_interaction(&mut con.transport).ok();
                    return Err(err.into());
                }
            };
            round += 1;
            let mut reader = arguments.send(&mut con.transport, is_done)?;
            if sideband_all {
                setup_remote_progress(progress, &mut reader);
            }
            let response = git_protocol::fetch::Response::from_line_reader(protocol_version, &mut reader)?;
            if response.has_pack() {
                progress.step();
                progress.set_name("receiving pack");
                if !sideband_all {
                    setup_remote_progress(progress, &mut reader);
                }
                break 'negotiation reader;
            } else {
                previous_response = Some(response);
            }
        };

        let options = git_pack::bundle::write::Options {
            thread_limit: config::index_threads(repo)?,
            index_version: config::pack_index_version(repo)?,
            iteration_mode: git_pack::data::input::Mode::Verify,
            object_hash: con.remote.repo.object_hash(),
        };

        let write_pack_bundle = if matches!(self.dry_run, fetch::DryRun::No) {
            Some(git_pack::Bundle::write_to_directory(
                reader,
                Some(repo.objects.store_ref().path().join("pack")),
                con.progress,
                should_interrupt,
                Some(Box::new({
                    let repo = repo.clone();
                    move |oid, buf| repo.objects.find(oid, buf).ok()
                })),
                options,
            )?)
        } else {
            drop(reader);
            None
        };

        if matches!(protocol_version, git_protocol::transport::Protocol::V2) {
            git_protocol::fetch::indicate_end_of_interaction(&mut con.transport).ok();
        }

        let update_refs = refs::update(
            repo,
            "fetch",
            &self.ref_map.mappings,
            con.remote.refspecs(remote::Direction::Fetch),
            self.dry_run,
        )?;

        Ok(Outcome {
            ref_map: std::mem::take(&mut self.ref_map),
            status: match write_pack_bundle {
                Some(write_pack_bundle) => Status::Change {
                    write_pack_bundle,
                    update_refs,
                },
                None => Status::DryRun { update_refs },
            },
        })
    }
}

fn setup_remote_progress(
    progress: &mut impl Progress,
    reader: &mut Box<dyn git_protocol::transport::client::ExtendedBufRead + Unpin + '_>,
) {
    use git_protocol::transport::client::ExtendedBufRead;
    reader.set_progress_handler(Some(Box::new({
        let mut remote_progress = progress.add_child("remote");
        move |is_err: bool, data: &[u8]| {
            git_protocol::RemoteProgress::translate_to_progress(is_err, data, &mut remote_progress)
        }
    }) as git_protocol::transport::client::HandleProgress));
}

mod config;
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
}

impl<'remote, 'repo, T, P> Drop for Prepare<'remote, 'repo, T, P>
where
    T: Transport,
{
    fn drop(&mut self) {
        if let Some(mut con) = self.con.take() {
            git_protocol::fetch::indicate_end_of_interaction(&mut con.transport).ok();
        }
    }
}
