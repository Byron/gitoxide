use crate::remote::fetch::RefMap;
use crate::remote::{ref_map, Connection};
use crate::Progress;
use git_protocol::transport::client::Transport;
use std::sync::atomic::AtomicBool;

mod error {
    /// The error returned by [`receive()`](super::Prepare::receive()).
    #[derive(Debug, thiserror::Error)]
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
    }
}
pub use error::Error;

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
    ///
    /// ### Negotiation
    ///
    /// "fetch.negotiationAlgorithm" describes algorithms `git` uses currently, with the default being `consecutive` and `skipping` being
    /// experimented with. We currently implement something we could call 'naive' which works for now.
    pub fn receive(mut self, _should_interrupt: &AtomicBool) -> Result<(), Error> {
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

        let _reader = 'negotiation: loop {
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

        let _options = git_pack::bundle::write::Options {
            thread_limit: config::index_threads(repo)?,
            index_version: config::pack_index_version(repo)?,
            iteration_mode: git_pack::data::input::Mode::Verify,
            object_hash: con.remote.repo.object_hash(),
        };
        // git_pack::Bundle::write_to_directory();
        todo!("read pack");

        if matches!(protocol_version, git_protocol::transport::Protocol::V2) {
            git_protocol::fetch::indicate_end_of_interaction(&mut con.transport).ok();
        }
        todo!("apply refs")
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
