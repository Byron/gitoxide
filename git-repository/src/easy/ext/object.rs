use std::ops::DerefMut;

use git_hash::ObjectId;

use crate::{
    easy,
    easy::{object, ObjectRef},
};
use git_odb::{Find, FindExt};

pub fn find_object<A: easy::Access + Sized>(
    access: &A,
    id: impl Into<ObjectId>,
) -> Result<easy::ObjectRef<'_, A>, object::find::existing::Error> {
    let state = access.state();
    let id = id.into();
    let kind = {
        let mut buf = access.state().try_borrow_mut_buf()?;
        let obj = access
            .repo()?
            .odb
            .find_existing(&id, &mut buf, state.try_borrow_mut_pack_cache()?.deref_mut())?;
        obj.kind
    };

    ObjectRef::from_current_buf(id, kind, access).map_err(Into::into)
}

pub fn try_find_object<A: easy::Access + Sized>(
    access: &A,
    id: impl Into<ObjectId>,
) -> Result<Option<easy::ObjectRef<'_, A>>, object::find::Error> {
    let state = access.state();
    let id = id.into();
    access
        .repo()?
        .odb
        .find(
            &id,
            state.try_borrow_mut_buf()?.deref_mut(),
            state.try_borrow_mut_pack_cache()?.deref_mut(),
        )?
        .map(|obj| {
            let kind = obj.kind;
            drop(obj);
            ObjectRef::from_current_buf(id, kind, access).map_err(Into::into)
        })
        .transpose()
}

pub trait ObjectAccessExt: easy::Access + Sized {
    // NOTE: in order to get the actual kind of object, is must be fully decoded from storage in case of packs
    // even though partial decoding is possible for loose objects, it won't matter much here.
    fn find_object(&self, id: impl Into<ObjectId>) -> Result<ObjectRef<'_, Self>, object::find::existing::Error> {
        find_object(self, id)
    }

    fn try_find_object(&self, id: impl Into<ObjectId>) -> Result<Option<ObjectRef<'_, Self>>, object::find::Error> {
        try_find_object(self, id)
    }
}
