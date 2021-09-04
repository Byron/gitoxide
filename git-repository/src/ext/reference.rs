use crate::easy;

pub trait Sealed {}

impl Sealed for git_ref::Reference {}

/// Extensions for [references][Reference].
pub trait ReferenceExt {
    /// Attach [`easy::Access`] to the given reference. It can be detached later with [`detach()]`.
    fn attach<A: easy::Access + Sized>(self, access: &A) -> easy::Reference<'_, A>;
}

impl ReferenceExt for git_ref::Reference {
    fn attach<A: easy::Access + Sized>(self, access: &A) -> easy::Reference<'_, A> {
        easy::Reference::from_ref(self, access)
    }
}
