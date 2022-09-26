use crate::remote::fetch::RefMap;
use crate::remote::{fetch, ref_map, Connection};
use crate::Progress;
use git_odb::FindExt;
use git_protocol::transport::client::Transport;
use std::sync::atomic::AtomicBool;

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
        #[error(transparent)]
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
    /// Nothing changed as the remote didn't have anything new.
    NoChange,
    /// There was at least one tip with a new object which we received.
    Change {
        /// Information collected while writing the pack and its index.
        write_pack_bundle: git_pack::bundle::write::Outcome,
        /// Information collected while updating references.
        update_refs: refs::update::Outcome,
    },
}

/// The outcome of receiving a pack via [`Prepare::receive()`].
#[derive(Debug, Clone)]
pub struct Outcome<'spec> {
    /// The result of the initial mapping of references, the prerequisite for any fetch.
    pub ref_map: RefMap<'spec>,
    /// The status of the operation to indicate what happened.
    pub status: Status,
}

///
pub mod negotiate;

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
    pub fn prepare_fetch(mut self, options: ref_map::Options) -> Result<Prepare<'remote, 'repo, T, P>, ref_map::Error> {
        let ref_map = self.ref_map_inner(options)?;
        Ok(Prepare {
            con: Some(self),
            ref_map,
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
    pub fn receive(mut self, should_interrupt: &AtomicBool) -> Result<Outcome<'remote>, Error> {
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
        let write_pack_bundle = git_pack::Bundle::write_to_directory(
            reader,
            Some(repo.objects.store_ref().path().join("pack")),
            con.progress,
            should_interrupt,
            Some(Box::new({
                let repo = repo.clone();
                move |oid, buf| repo.objects.find(oid, buf).ok()
            })),
            options,
        )?;

        if matches!(protocol_version, git_protocol::transport::Protocol::V2) {
            git_protocol::fetch::indicate_end_of_interaction(&mut con.transport).ok();
        }

        let update_refs = refs::update(
            repo,
            &self.ref_map.mappings,
            con.remote.refspecs(crate::remote::Direction::Fetch),
            fetch::DryRun::No,
        )?;

        Ok(Outcome {
            ref_map: std::mem::take(&mut self.ref_map),
            status: Status::Change {
                write_pack_bundle,
                update_refs,
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
#[path = "update_refs.rs"]
pub mod refs;

/// A structure to hold the result of the handshake with the remote and configure the upcoming fetch operation.
#[allow(dead_code)]
pub struct Prepare<'remote, 'repo, T, P>
where
    T: Transport,
{
    con: Option<Connection<'remote, 'repo, T, P>>,
    ref_map: RefMap<'remote>,
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
