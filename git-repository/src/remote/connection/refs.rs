use crate::remote::Connection;
use git_protocol::transport::client::Transport;

impl<'a, 'repo, T> Connection<'a, 'repo, T>
where
    T: Transport,
{
    /// List all references on the remote that have been filtered through our remote's [`refspecs`][crate::Remote::refspecs()]
    /// for _fetching_.
    ///
    /// This comes in the form of information of all matching tips on the remote and the object they point to, along with
    /// with the local tracking branch of these tips (if available).
    ///
    /// Note that this doesn't fetch the objects mentioned in the tips nor does it make any change to underlying repository.
    pub fn refs(&mut self) -> ! {
        todo!()
    }
}
