use std::convert::TryInto;

use git_hash::{oid, ObjectId};
use git_odb::{Find, FindExt};
use git_ref::{
    transaction::{LogChange, PreviousValue, RefLog},
    FullName,
};

use crate::{commit, ext::ObjectIdExt, object, tag, Id, Object, Reference};

/// Methods related to object creation.
impl crate::Repository {
    // TODO: tests, actual integration of rev-spec parsing when available.
    /// Parse a revision specification and turn it into the full id to the object it describes, similar to `git rev-parse`.
    /// NOTE that currently this only parses full hex names.
    pub fn rev_parse(&self, spec: impl AsRef<str>) -> Result<Id<'_>, crate::rev_spec::parse::Error> {
        Ok(git_hash::ObjectId::from_hex(spec.as_ref().as_bytes())?.attach(self))
    }

    /// Find the object with `id` in the object database or return an error if it could not be found.
    ///
    /// There are various legitimate reasons for an object to not be present, which is why
    /// [`try_find_object(…)`][crate::Repository::try_find_object()] might be preferable instead.
    ///
    /// # Important
    ///
    /// As a shared buffer is written to back the object data, the returned `ObjectRef` will prevent other
    /// `find_object()` operations from succeeding while alive.
    /// To bypass this limit, clone this `sync::Handle` instance.
    ///
    /// # Performance Note
    ///
    /// In order to get the kind of the object, is must be fully decoded from storage if it is packed with deltas.
    /// Loose object could be partially decoded, even though that's not implemented.
    pub fn find_object(&self, id: impl Into<ObjectId>) -> Result<Object<'_>, object::find::existing::OdbError> {
        let id = id.into();
        let mut buf = self.free_buf();
        let kind = self.objects.find(&id, &mut buf)?.kind;
        Ok(Object::from_data(id, kind, buf, self))
    }

    /// Try to find the object with `id` or return `None` it it wasn't found.
    ///
    /// # Important
    ///
    /// As a shared buffer is written to back the object data, the returned `ObjectRef` will prevent other
    /// `try_find_object()` operations from succeeding while alive.
    /// To bypass this limit, clone this `sync::Handle` instance.
    pub fn try_find_object(&self, id: impl Into<ObjectId>) -> Result<Option<Object<'_>>, object::find::OdbError> {
        let state = self;
        let id = id.into();

        let mut buf = state.free_buf();
        match self.objects.try_find(&id, &mut buf)? {
            Some(obj) => {
                let kind = obj.kind;
                drop(obj);
                Ok(Some(Object::from_data(id, kind, buf, self)))
            }
            None => Ok(None),
        }
    }

    /// Write the given object into the object database and return its object id.
    pub fn write_object(&self, object: impl git_object::WriteTo) -> Result<Id<'_>, object::write::Error> {
        use git_odb::Write;

        let state = self;
        state
            .objects
            .write(object)
            .map(|oid| oid.attach(self))
            .map_err(Into::into)
    }

    /// Create a tag reference named `name` (without `refs/tags/` prefix) pointing to a newly created tag object
    /// which in turn points to `target` and return the newly created reference.
    ///
    /// It will be created with `constraint` which is most commonly to [only create it][PreviousValue::MustNotExist]
    /// or to [force overwriting a possibly existing tag](PreviousValue::Any).
    pub fn tag(
        &self,
        name: impl AsRef<str>,
        target: impl AsRef<oid>,
        target_kind: git_object::Kind,
        tagger: Option<git_actor::SignatureRef<'_>>,
        message: impl AsRef<str>,
        constraint: PreviousValue,
    ) -> Result<Reference<'_>, tag::Error> {
        // NOTE: This could be more efficient if we use a TagRef instead.
        let tag = git_object::Tag {
            target: target.as_ref().into(),
            target_kind,
            name: name.as_ref().into(),
            tagger: tagger.map(|t| t.to_owned()),
            message: message.as_ref().into(),
            pgp_signature: None,
        };
        let tag_id = self.write_object(&tag)?;
        self.tag_reference(name, tag_id, constraint).map_err(Into::into)
    }

    /// Create a new commit object with `author`, `committer` and `message` referring to `tree` with `parents`, and point `reference`
    /// to it. The commit is written without message encoding field, which can be assumed to be UTF-8.
    ///
    /// `reference` will be created if it doesn't exist, and can be `"HEAD"` to automatically write-through to the symbolic reference
    /// that `HEAD` points to if it is not detached. For this reason, detached head states cannot be created unless the `HEAD` is detached
    /// already. The reflog will be written as canonical git would do, like `<operation> (<detail>): <summary>`.
    ///
    /// The first parent id in `parents` is expected to be the current target of `reference` and the operation will fail if it is not.
    /// If there is no parent, the `reference` is expected to not exist yet.
    ///
    /// The method fails immediately if a `reference` lock can't be acquired.
    pub fn commit<Name, E>(
        &self,
        reference: Name,
        author: git_actor::SignatureRef<'_>,
        committer: git_actor::SignatureRef<'_>,
        message: impl AsRef<str>,
        tree: impl Into<ObjectId>,
        parents: impl IntoIterator<Item = impl Into<ObjectId>>,
    ) -> Result<Id<'_>, commit::Error>
    where
        Name: TryInto<FullName, Error = E>,
        commit::Error: From<E>,
    {
        use git_ref::{
            transaction::{Change, RefEdit},
            Target,
        };

        // TODO: possibly use CommitRef to save a few allocations (but will have to allocate for object ids anyway.
        //       This can be made vastly more efficient though if we wanted to, so we lie in the API
        let reference = reference.try_into()?;
        let commit = git_object::Commit {
            message: message.as_ref().into(),
            tree: tree.into(),
            author: author.to_owned(),
            committer: committer.to_owned(),
            encoding: None,
            parents: parents.into_iter().map(|id| id.into()).collect(),
            extra_headers: Default::default(),
        };

        let commit_id = self.write_object(&commit)?;
        self.edit_reference(
            RefEdit {
                change: Change::Update {
                    log: LogChange {
                        mode: RefLog::AndReference,
                        force_create_reflog: false,
                        message: crate::reference::log::message(
                            "commit",
                            commit.message.as_ref(),
                            commit.parents.len(),
                        ),
                    },
                    expected: match commit.parents.get(0).map(|p| Target::Peeled(*p)) {
                        Some(previous) => {
                            if reference.as_bstr() == "HEAD" {
                                PreviousValue::MustExistAndMatch(previous)
                            } else {
                                PreviousValue::ExistingMustMatch(previous)
                            }
                        }
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
