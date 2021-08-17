use crate::{
    hash::{oid, ObjectId},
    Access, Object,
};

impl<'repo, A, B> PartialEq<Object<'repo, A>> for Object<'repo, B> {
    fn eq(&self, other: &Object<'repo, A>) -> bool {
        self.id == other.id
    }
}

impl<'repo, A> std::fmt::Debug for Object<'repo, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl<'repo, A> Object<'repo, A>
where
    A: Access + Sized,
{
    pub(crate) fn try_from_oid(oid: impl Into<ObjectId>, access: &'repo A) -> Result<Self, ()> {
        Ok(Object { id: oid.into(), access })
    }

    pub fn id(&self) -> &oid {
        &self.id
    }
}
