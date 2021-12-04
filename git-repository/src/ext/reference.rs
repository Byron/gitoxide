use crate::easy;

pub trait Sealed {}

impl Sealed for git_ref::Reference {}

/// Extensions for [references][git_ref::Reference].
pub trait ReferenceExt {
    /// Attach [`easy::Handle`] to the given reference. It can be detached later with [`detach()]`.
    fn attach(self, handle: &easy::Handle) -> easy::Reference<'_>;
}

impl ReferenceExt for git_ref::Reference {
    fn attach(self, handle: &easy::Handle) -> easy::Reference<'_> {
        easy::Reference::from_ref(self, handle)
    }
}
