use crate::remote::fetch::RefMap;
use crate::remote::{ref_map, Connection};
use crate::Progress;
use git_protocol::transport::client::Transport;

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
