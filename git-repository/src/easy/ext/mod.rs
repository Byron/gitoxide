#![allow(missing_docs)]
pub(crate) mod object;
pub use object::ObjectAccessExt;

mod reference;
pub use reference::ReferenceAccessExt;

mod config {
    use crate::easy;

    pub trait ConfigAccessExt: easy::Access + Sized {
        // TODO: actual implementation
        fn committer(&self) -> git_actor::Signature {
            // TODO: actually read the committer information from git-config, probably it should be provided here
            git_actor::Signature::empty()
        }
    }
    impl<A> ConfigAccessExt for A where A: easy::Access + Sized {}
}
pub use config::ConfigAccessExt;
