use crate::remote::Connection;
use crate::Progress;
use git_protocol::transport::client::Transport;

#[allow(missing_docs)]
pub struct Options {}

impl<'remote, 'repo, T, P> Connection<'remote, 'repo, T, P>
where
    T: Transport,
    P: Progress,
{
    #[allow(missing_docs)]
    pub fn prepare_fetch(self) -> ! {
        todo!()
    }
}
