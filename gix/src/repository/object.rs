#![allow(clippy::result_large_err)]
use std::convert::TryInto;

use gix_hash::ObjectId;
use gix_odb::{Find, FindExt, Write};
use gix_ref::{
    transaction::{LogChange, PreviousValue, RefLog},
    FullName,
};

use crate::{commit, ext::ObjectIdExt, object, tag, Id, Object, Reference, Tree};

/// Methods related to object creation.
impl crate::Repository {
    /// Find the object with `id` in the object database or return an error if it could not be found.
    ///
    /// There are various legitimate reasons for an object to not be present, which is why
    /// [`try_find_object(…)`][crate::Repository::try_find_object()] might be preferable instead.
    ///
    /// # Performance Note
    ///
    /// In order to get the kind of the object, is must be fully decoded from storage if it is packed with deltas.
    /// Loose object could be partially decoded, even though that's not implemented.
    pub fn find_object(&self, id: impl Into<ObjectId>) -> Result<Object<'_>, object::find::existing::Error> {
        let id = id.into();
        if id == gix_hash::ObjectId::empty_tree(self.object_hash()) {
            return Ok(Object {
                id,
                kind: gix_object::Kind::Tree,
                data: Vec::new(),
                repo: self,
            });
        }
        let mut buf = self.free_buf();
        let kind = self.objects.find(id, &mut buf)?.kind;
        Ok(Object::from_data(id, kind, buf, self))
    }

    /// Try to find the object with `id` or return `None` it it wasn't found.
    pub fn try_find_object(&self, id: impl Into<ObjectId>) -> Result<Option<Object<'_>>, object::find::Error> {
        let id = id.into();
        if id == gix_hash::ObjectId::empty_tree(self.object_hash()) {
            return Ok(Some(Object {
                id,
                kind: gix_object::Kind::Tree,
                data: Vec::new(),
                repo: self,
            }));
        }

        let mut buf = self.free_buf();
        match self.objects.try_find(id, &mut buf)? {
            Some(obj) => {
                let kind = obj.kind;
                Ok(Some(Object::from_data(id, kind, buf, self)))
            }
            None => Ok(None),
        }
    }

    /// Write the given object into the object database and return its object id.
    pub fn write_object(&self, object: impl gix_object::WriteTo) -> Result<Id<'_>, object::write::Error> {
        self.objects
            .write(object)
            .map(|oid| oid.attach(self))
            .map_err(Into::into)
    }

    /// Write a blob from the given `bytes`.
    pub fn write_blob(&self, bytes: impl AsRef<[u8]>) -> Result<Id<'_>, object::write::Error> {
        self.objects
            .write_buf(gix_object::Kind::Blob, bytes.as_ref())
            .map(|oid| oid.attach(self))
    }

    /// Write a blob from the given `Read` implementation.
    pub fn write_blob_stream(
        &self,
        mut bytes: impl std::io::Read + std::io::Seek,
    ) -> Result<Id<'_>, object::write::Error> {
        let current = bytes.stream_position()?;
        let len = bytes.seek(std::io::SeekFrom::End(0))? - current;
        bytes.seek(std::io::SeekFrom::Start(current))?;

        self.objects
            .write_stream(gix_object::Kind::Blob, len, bytes)
            .map(|oid| oid.attach(self))
    }

    /// Create a tag reference named `name` (without `refs/tags/` prefix) pointing to a newly created tag object
    /// which in turn points to `target` and return the newly created reference.
    ///
    /// It will be created with `constraint` which is most commonly to [only create it][PreviousValue::MustNotExist]
    /// or to [force overwriting a possibly existing tag](PreviousValue::Any).
    pub fn tag(
        &self,
        name: impl AsRef<str>,
        target: impl AsRef<gix_hash::oid>,
        target_kind: gix_object::Kind,
        tagger: Option<gix_actor::SignatureRef<'_>>,
        message: impl AsRef<str>,
        constraint: PreviousValue,
    ) -> Result<Reference<'_>, tag::Error> {
        let tag = gix_object::Tag {
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

    /// Similar to [`commit(…)`][crate::Repository::commit()], but allows to create the commit with `committer` and `author` specified.
    ///
    /// This forces setting the commit time and author time by hand. Note that typically, committer and author are the same.
    pub fn commit_as<'a, 'c, Name, E>(
        &self,
        committer: impl Into<gix_actor::SignatureRef<'c>>,
        author: impl Into<gix_actor::SignatureRef<'a>>,
        reference: Name,
        message: impl AsRef<str>,
        tree: impl Into<ObjectId>,
        parents: impl IntoIterator<Item = impl Into<ObjectId>>,
    ) -> Result<Id<'_>, commit::Error>
    where
        Name: TryInto<FullName, Error = E>,
        commit::Error: From<E>,
    {
        use gix_ref::{
            transaction::{Change, RefEdit},
            Target,
        };

        // TODO: possibly use CommitRef to save a few allocations (but will have to allocate for object ids anyway.
        //       This can be made vastly more efficient though if we wanted to, so we lie in the API
        let reference = reference.try_into()?;
        let commit = gix_object::Commit {
            message: message.as_ref().into(),
            tree: tree.into(),
            author: author.into().to_owned(),
            committer: committer.into().to_owned(),
            encoding: None,
            parents: parents.into_iter().map(|id| id.into()).collect(),
            extra_headers: Default::default(),
        };

        let commit_id = self.write_object(&commit)?;
        self.edit_reference(RefEdit {
            change: Change::Update {
                log: LogChange {
                    mode: RefLog::AndReference,
                    force_create_reflog: false,
                    message: crate::reference::log::message("commit", commit.message.as_ref(), commit.parents.len()),
                },
                expected: match commit.parents.first().map(|p| Target::Peeled(*p)) {
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
        })?;
        Ok(commit_id)
    }

    /// Create a new commit object with `message` referring to `tree` with `parents`, and point `reference`
    /// to it. The commit is written without message encoding field, which can be assumed to be UTF-8.
    /// `author` and `committer` fields are pre-set from the configuration, which can be altered
    /// [temporarily][crate::Repository::config_snapshot_mut()] before the call if required.
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
        message: impl AsRef<str>,
        tree: impl Into<ObjectId>,
        parents: impl IntoIterator<Item = impl Into<ObjectId>>,
    ) -> Result<Id<'_>, commit::Error>
    where
        Name: TryInto<FullName, Error = E>,
        commit::Error: From<E>,
    {
        let author = self.author().ok_or(commit::Error::AuthorMissing)??;
        let committer = self.committer().ok_or(commit::Error::CommitterMissing)??;
        self.commit_as(committer, author, reference, message, tree, parents)
    }

    /// Return an empty tree object, suitable for [getting changes](crate::Tree::changes()).
    ///
    /// Note that it is special and doesn't physically exist in the object database even though it can be returned.
    /// This means that this object can be used in an uninitialized, empty repository which would report to have no objects at all.
    pub fn empty_tree(&self) -> Tree<'_> {
        self.find_object(gix_hash::ObjectId::empty_tree(self.object_hash()))
            .expect("always present")
            .into_tree()
    }
}
