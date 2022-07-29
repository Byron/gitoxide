use crate::ext::ReferenceExt;
use crate::{Id, Reference, RevSpec};
use git_revision::Spec;

///
pub mod parse;

mod impls {
    use crate::RevSpec;
    use std::ops::{Deref, DerefMut};

    impl<'repo> Deref for RevSpec<'repo> {
        type Target = git_revision::Spec;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl<'repo> DerefMut for RevSpec<'repo> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

    impl<'repo> PartialEq for RevSpec<'repo> {
        fn eq(&self, other: &Self) -> bool {
            self.inner == other.inner
        }
    }

    impl<'repo> Eq for RevSpec<'repo> {}
}

/// Initialization
impl<'repo> RevSpec<'repo> {
    /// Create a single specification which points to `id`.
    pub fn from_id(id: Id<'repo>) -> Self {
        RevSpec {
            inner: git_revision::Spec::Include(id.inner),
            repo: id.repo,
            first_ref: None,
            second_ref: None,
        }
    }
}

/// Access
impl<'repo> RevSpec<'repo> {
    /// Detach the `Repository` from this instance, leaving only plain data that can be moved freely and serialized.
    pub fn detach(self) -> git_revision::Spec {
        self.inner
    }

    /// Some revision specifications leave information about reference names which are returned as `(from-ref, to-ref)` here, e.g.
    /// `HEAD@{-1}..main` might be (`refs/heads/previous-branch`, `refs/heads/main`).
    // TODO: tests
    pub fn into_names(self) -> (Option<Reference<'repo>>, Option<Reference<'repo>>) {
        let repo = self.repo;
        (
            self.first_ref.map(|r| r.attach(repo)),
            self.second_ref.map(|r| r.attach(repo)),
        )
    }

    /// Return the single included object represented by this instance, or `None` if it is a range of any kind.
    pub fn single(&self) -> Option<Id<'repo>> {
        match self.inner {
            git_revision::Spec::Include(id) | git_revision::Spec::ExcludeFromParents { from: id } => {
                Id::from_id(id, self.repo).into()
            }
            Spec::Exclude(_) | Spec::Range { .. } | Spec::Merge { .. } | Spec::IncludeOnlyParents { .. } => None,
        }
    }
}
