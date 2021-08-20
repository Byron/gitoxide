use crate::easy::object::find;
use crate::Oid;
use crate::{
    easy,
    hash::{oid, ObjectId},
    Object, ObjectRef,
};

impl<'repo, A, B> PartialEq<Oid<'repo, A>> for Oid<'repo, B> {
    fn eq(&self, other: &Oid<'repo, A>) -> bool {
        self.id == other.id
    }
}

impl<'repo, A> PartialEq<ObjectId> for Oid<'repo, A> {
    fn eq(&self, other: &ObjectId) -> bool {
        &self.id == other
    }
}

impl<'repo, A> PartialEq<oid> for Oid<'repo, A> {
    fn eq(&self, other: &oid) -> bool {
        self.id == other
    }
}

impl<'repo, A, B> PartialEq<ObjectRef<'repo, A>> for Oid<'repo, B> {
    fn eq(&self, other: &ObjectRef<'repo, A>) -> bool {
        self.id == other.id
    }
}

impl<'repo, A> PartialEq<Object> for Oid<'repo, A> {
    fn eq(&self, other: &Object) -> bool {
        self.id == other.id
    }
}

impl<'repo, A> std::fmt::Debug for Oid<'repo, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl<'repo, A> AsRef<oid> for Oid<'repo, A> {
    fn as_ref(&self) -> &oid {
        &self.id
    }
}

impl<'repo, A> From<Oid<'repo, A>> for ObjectId {
    fn from(v: Oid<'repo, A>) -> Self {
        v.id
    }
}

impl<'repo, A> Oid<'repo, A>
where
    A: easy::Access + Sized,
{
    // NOTE: Can't access other object data that is attached to the same cache.
    pub fn object(&self) -> Result<ObjectRef<'repo, A>, find::existing::Error> {
        crate::ext::access::object::find_object(self.access, self.id)
    }

    // NOTE: Can't access other object data that is attached to the same cache.
    pub fn try_object(&self) -> Result<Option<ObjectRef<'repo, A>>, find::Error> {
        crate::ext::access::object::try_find_object(self.access, self.id)
    }
}

impl<'repo, A> Oid<'repo, A>
where
    A: easy::Access + Sized,
{
    pub(crate) fn from_id(id: impl Into<ObjectId>, access: &'repo A) -> Self {
        Oid { id: id.into(), access }
    }

    pub fn detach(self) -> ObjectId {
        self.id
    }
}
