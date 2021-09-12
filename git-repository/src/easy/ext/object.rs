use std::{convert::TryInto, ops::DerefMut};

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

/// Methods related to object creation.
pub trait ObjectAccessExt: easy::Access + Sized {
    /// Find the object with `id` in the object database or return an error if it could not be found.
    ///
    /// There are various legitimate reasons for an object to not be present, which is why
    /// [`try_find_object(…)`][ObjectAccessExt::try_find_object()] might be preferable instead.
    ///
    /// # Important
    ///
    /// As a shared buffer is written to back the object data, the returned `ObjectRef` will prevent other
    /// `find_object()` operations from succeeding while alive.
    /// To bypass this limit, clone this `easy::Access` instance.
    ///
    /// # Performance Note
    ///
    /// In order to get the kind of the object, is must be fully decoded from storage if it is packed with deltas.
    /// Loose object could be partially decoded, even though that's not implemented.
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

    /// Try to find the object with `id` or return `None` it it wasn't found.
    ///
    /// # Important
    ///
    /// As a shared buffer is written to back the object data, the returned `ObjectRef` will prevent other
    /// `try_find_object()` operations from succeeding while alive.
    /// To bypass this limit, clone this `easy::Access` instance.
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

    /// Write the given object into the object database and return its object id.
    fn write_object(&self, object: impl git_object::WriteTo) -> Result<Oid<'_, Self>, object::write::Error> {
        use git_odb::Write;

        let repo = self.repo()?;
        repo.odb
            .write(object, repo.hash_kind)
            .map(|oid| oid.attach(self))
            .map_err(Into::into)
    }

    /// Create a new commit object with `author`, `committer` and `message` referring to `tree` with `parents`, and point `reference`
    /// to it. The commit is written without message encoding field, which can be assumed to be UTF-8.
    ///
    /// `reference` will be created if it doesn't exist, and can be `"HEAD"` to automatically write-through to the symbolic reference
    /// that `HEAD` points to if it is not detached. For this reason, detached head states cannot be created unless the `HEAD` is detached
    /// already.
    ///
    /// The first parent id in `parents` is expected to be the current target of `reference` and the operation will fail if it is not.
    /// If there is no parent, the `reference` is expected to not exist yet.
    ///
    /// The method fails immediately if a `reference` lock can't be acquired.
    fn commit<'a, Name, E>(
        &self,
        reference: Name,
        author: &git_actor::SignatureRef<'a>,
        committer: &git_actor::SignatureRef<'a>,
        message: impl AsRef<str>,
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

        // TODO: possibly use CommitRef to save a few allocations (but will have to allocate for object ids anyway.
        //       This can be made vastly more efficient though if we wanted to, so we lie in the API
        let reference = reference.try_into()?;
        let commit: git_object::Object = git_object::Commit {
            message: message.as_ref().into(),
            tree: tree.into(),
            author: author.to_owned(),
            committer: committer.to_owned(),
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
                    expected: match commit.parents.get(0).map(|p| Target::Peeled(*p)) {
                        Some(previous) => PreviousValue::ExistingMustMatch(previous),
                        None => PreviousValue::MustNotExist,
                    },
                    new: Target::Peeled(commit_id.inner),
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
