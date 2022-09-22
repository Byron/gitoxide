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
    /// Note that at this point, the `transport` should be configured using the [`transport_mut()`][Self::transport_mut()]
    /// method, as it will be consumed here.
    #[allow(missing_docs)]
    pub fn prepare_fetch(self, _options: ref_map::Options) -> Result<Prepare<'remote, 'repo, T, P>, ref_map::Error> {
        todo!()
    }
}

/// A structure to hold the result of the handshake with the remote and configure the upcoming fetch operation.
#[allow(dead_code)]
pub struct Prepare<'remote, 'repo, T, P> {
    con: Connection<'remote, 'repo, T, P>,
    ref_map: RefMap<'remote>,
}
