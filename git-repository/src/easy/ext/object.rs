use std::ops::DerefMut;

use git_hash::ObjectId;
use git_odb::{Find, FindExt};

use crate::{
    easy,
    easy::{object, ObjectRef, Oid},
};

pub trait ObjectAccessExt: easy::Access + Sized {
    // NOTE: in order to get the actual kind of object, is must be fully decoded from storage in case of packs
    // even though partial decoding is possible for loose objects, it won't matter much here.
    fn find_object(&self, id: impl Into<ObjectId>) -> Result<ObjectRef<'_, Self>, object::find::existing::Error> {
        let state = self.state();
        let id = id.into();
        let kind = {
            let mut buf = self.state().try_borrow_mut_buf()?;
            let obj = self
                .repo()?
                .odb
                .find(&id, &mut buf, state.try_borrow_mut_pack_cache()?.deref_mut())?;
            obj.kind
        };

        ObjectRef::from_current_buf(id, kind, self).map_err(Into::into)
    }

    fn try_find_object(&self, id: impl Into<ObjectId>) -> Result<Option<ObjectRef<'_, Self>>, object::find::Error> {
        let state = self.state();
        let id = id.into();
        self.repo()?
            .odb
            .try_find(
                &id,
                state.try_borrow_mut_buf()?.deref_mut(),
                state.try_borrow_mut_pack_cache()?.deref_mut(),
            )?
            .map(|obj| {
                let kind = obj.kind;
                drop(obj);
                ObjectRef::from_current_buf(id, kind, self).map_err(Into::into)
            })
            .transpose()
    }

    fn write_object(&self, object: &git_object::Object) -> Result<Oid<'_, Self>, object::write::Error> {
        use git_odb::Write;

        use crate::ext::ObjectIdExt;

        let repo = self.repo()?;
        repo.odb
            .write(object, repo.hash_kind)
            .map(|oid| oid.attach(self))
            .map_err(Into::into)
    }
}

impl<A> ObjectAccessExt for A where A: easy::Access + Sized {}
