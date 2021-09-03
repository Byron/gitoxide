use std::{convert::TryInto, ops::DerefMut};

use bstr::BString;
use git_hash::ObjectId;
use git_odb::{Find, FindExt};
use git_ref::{
    transaction::{LogChange, PreviousValue, RefLog},
    FullName,
};

use crate::{
    easy,
    easy::{commit, object, ObjectRef, Oid},
    ext::ObjectIdExt,
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

        let repo = self.repo()?;
        repo.odb
            .write(object, repo.hash_kind)
            .map(|oid| oid.attach(self))
            .map_err(Into::into)
    }

    // docs notes
    // Fails immediately if lock can't be acquired as first parent depends on it
    // Writes without message encoding
    fn commit<Name, E>(
        &self,
        reference: Name,
        message: impl Into<BString>,
        author: impl Into<git_actor::Signature>,
        committer: impl Into<git_actor::Signature>,
        tree: impl Into<ObjectId>,
        parents: impl IntoIterator<Item = impl Into<ObjectId>>,
    ) -> Result<Oid<'_, Self>, commit::Error>
    where
        Name: TryInto<FullName, Error = E>,
        commit::Error: From<E>,
    {
        use git_ref::{
            transaction::{Change, RefEdit},
            Target,
        };

        use crate::easy::ext::ReferenceAccessExt;

        let reference = reference.try_into()?;
        let commit: git_object::Object = git_object::Commit {
            message: message.into(),
            tree: tree.into(),
            author: author.into(),
            committer: committer.into(),
            encoding: None,
            parents: parents.into_iter().map(|id| id.into()).collect(),
            extra_headers: Default::default(),
        }
        .into();

        let commit_id = self.write_object(&commit)?;
        let commit = commit.into_commit();
        self.edit_reference(
            RefEdit {
                change: Change::Update {
                    log: LogChange {
                        mode: RefLog::AndReference,
                        force_create_reflog: false,
                        message: crate::reference::log::message("commit", &commit),
                    },
                    previous: match commit.parents.get(0).map(|p| Target::Peeled(*p)) {
                        Some(previous) => PreviousValue::ExistingMustMatch(previous),
                        None => PreviousValue::MustNotExist,
                    },
                    new: Target::Peeled(commit_id.id),
                },
                name: reference,
                deref: true,
            },
            git_lock::acquire::Fail::Immediately,
            Some(&commit.committer),
        )?;
        Ok(commit_id)
    }
}

impl<A> ObjectAccessExt for A where A: easy::Access + Sized {}
