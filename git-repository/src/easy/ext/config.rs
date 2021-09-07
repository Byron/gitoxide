use crate::easy;

pub trait ConfigAccessExt: easy::Access + Sized {
    // TODO: actual implementation
    fn committer(&self) -> git_actor::Signature {
        // TODO: actually read the committer information from git-config, probably it should be provided here
        git_actor::Signature::empty()
    }

    /// The kind of hash the repository is configured to use
    fn hash_kind(&self) -> Result<git_hash::Kind, easy::borrow::repo::Error> {
        self.repo().map(|r| r.hash_kind)
    }
}

impl<A> ConfigAccessExt for A where A: easy::Access + Sized {}
