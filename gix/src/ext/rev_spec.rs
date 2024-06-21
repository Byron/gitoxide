/// Extensions for [revision specifications][gix_revision::Spec].
pub trait RevSpecExt {
    /// Attach [`Repository`][crate::Repository] to the given rev-spec.
    fn attach(self, repo: &crate::Repository) -> crate::revision::Spec<'_>;
}

impl RevSpecExt for gix_revision::Spec {
    fn attach(self, repo: &crate::Repository) -> crate::revision::Spec<'_> {
        crate::revision::Spec {
            inner: self,
            path: None,
            first_ref: None,
            second_ref: None,
            repo,
        }
    }
}
