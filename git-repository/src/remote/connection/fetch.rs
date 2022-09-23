use crate::remote::fetch::RefMap;
use crate::remote::{ref_map, Connection};
use crate::Progress;
use git_protocol::transport::client::Transport;
use std::sync::atomic::AtomicBool;

mod error {
    /// The error returned by [`receive()`](super::Prepare::receive()).
    #[derive(Debug, thiserror::Error)]
    #[error("TBD")]
    pub enum Error {
        #[error("The configured pack.indexVersion is not valid. It must be 1 or 2, with 2 being the default{}", desired.map(|n| format!(" (but got {})", n)).unwrap_or_default())]
        PackIndexVersion {
            desired: Option<i64>,
            source: Option<git_config::value::Error>,
        },
    }
}
pub use error::Error;

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
        git_protocol::fetch::indicate_end_of_interaction(&mut con.transport).ok();

        let repo = con.remote.repo;
        let _index_version = config::pack_index_version(repo)?;
        // let options = git_pack::bundle::write::Options {
        //     thread_limit: ctx.thread_limit,
        //     index_version: git_pack::index::Version::V2,
        //     iteration_mode: git_pack::data::input::Mode::Verify,
        //     object_hash: ctx.object_hash,
        // };

        todo!()
    }
}

mod config {
    use super::Error;
    use crate::Repository;

    pub fn pack_index_version(repo: &Repository) -> Result<git_pack::index::Version, Error> {
        use git_pack::index::Version;
        let lenient_config = repo.options.lenient_config;
        Ok(
            match repo.config.resolved.integer("pack", None, "indexVersion").transpose() {
                Ok(Some(v)) if v == 1 => Version::V1,
                Ok(Some(v)) if v == 2 => Version::V2,
                Ok(None) => Version::V2,
                Ok(Some(_)) | Err(_) if lenient_config => Version::V2,
                Ok(Some(v)) => {
                    return Err(Error::PackIndexVersion {
                        desired: v.into(),
                        source: None,
                    })
                }
                Err(err) => {
                    return Err(Error::PackIndexVersion {
                        desired: None,
                        source: err.into(),
                    })
                }
            },
        )
    }
}

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
