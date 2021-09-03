use git_hash::{oid, ObjectId};

use crate::{
    easy,
    easy::{ext::ObjectAccessExt, object::find, Object, ObjectRef, Oid},
};

impl<'repo, A, B> PartialEq<Oid<'repo, A>> for Oid<'repo, B> {
    fn eq(&self, other: &Oid<'repo, A>) -> bool {
        self.inner == other.inner
    }
}

impl<'repo, A> PartialEq<ObjectId> for Oid<'repo, A> {
    fn eq(&self, other: &ObjectId) -> bool {
        &self.inner == other
    }
}

impl<'repo, A> PartialEq<oid> for Oid<'repo, A> {
    fn eq(&self, other: &oid) -> bool {
        self.inner == other
    }
}

impl<'repo, A, B> PartialEq<ObjectRef<'repo, A>> for Oid<'repo, B> {
    fn eq(&self, other: &ObjectRef<'repo, A>) -> bool {
        self.inner == other.id
    }
}

impl<'repo, A> PartialEq<Object> for Oid<'repo, A> {
    fn eq(&self, other: &Object) -> bool {
        self.inner == other.id
    }
}

impl<'repo, A> std::fmt::Debug for Oid<'repo, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'repo, A> AsRef<oid> for Oid<'repo, A> {
    fn as_ref(&self) -> &oid {
        &self.inner
    }
}

impl<'repo, A> From<Oid<'repo, A>> for ObjectId {
    fn from(v: Oid<'repo, A>) -> Self {
        v.inner
    }
}

impl<'repo, A> Oid<'repo, A>
where
    A: easy::Access + Sized,
{
    // NOTE: Can't access other object data that is attached to the same cache.
    /// Find the [`ObjectRef`] associated with this object id, and assume it exists.
    pub fn object(&self) -> Result<ObjectRef<'repo, A>, find::existing::Error> {
        self.access.find_object(self.inner)
    }

    // NOTE: Can't access other object data that is attached to the same cache.
    /// Try find the [`ObjectRef`] associated with this object id, it might not be available locally.
    pub fn try_object(&self) -> Result<Option<ObjectRef<'repo, A>>, find::Error> {
        self.access.try_find_object(self.inner)
    }
}

impl<'repo, A> Oid<'repo, A>
where
    A: easy::Access + Sized,
{
    pub(crate) fn from_id(id: impl Into<ObjectId>, access: &'repo A) -> Self {
        Oid {
            inner: id.into(),
            access,
        }
    }

    /// Turn this instance into its bare [ObjectId].
    pub fn detach(self) -> ObjectId {
        self.inner
    }
}
