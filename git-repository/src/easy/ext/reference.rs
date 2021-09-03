use std::convert::TryInto;

use git_actor as actor;
use git_hash::ObjectId;
use git_lock as lock;
use git_ref::{
    transaction::{Change, Create, RefEdit},
    PartialNameRef, Target,
};

use crate::{
    easy,
    easy::{reference, Reference},
};

/// Obtain and alter references comfortably
pub trait ReferenceAccessExt: easy::Access + Sized {
    fn tag(
        &self,
        name: impl AsRef<str>,
        target: impl Into<ObjectId>,
        lock_mode: lock::acquire::Fail,
        force: bool,
    ) -> Result<Vec<RefEdit>, reference::edit::Error> {
        self.edit_reference(
            RefEdit {
                change: Change::Update {
                    log: Default::default(),
                    previous: if force {
                        Create::OrUpdate { previous: None }
                    } else {
                        Create::Only
                    },
                    new: Target::Peeled(target.into()),
                },
                name: format!("refs/tags/{}", name.as_ref()).try_into()?,
                deref: false,
            },
            lock_mode,
            None,
        )
    }

    fn edit_reference(
        &self,
        edit: RefEdit,
        lock_mode: lock::acquire::Fail,
        log_committer: Option<&actor::Signature>,
    ) -> Result<Vec<RefEdit>, reference::edit::Error> {
        self.edit_references(Some(edit), lock_mode, log_committer)
    }

    fn edit_references(
        &self,
        edits: impl IntoIterator<Item = RefEdit>,
        lock_mode: lock::acquire::Fail,
        log_committer: Option<&actor::Signature>,
    ) -> Result<Vec<RefEdit>, reference::edit::Error> {
        let committer_storage;
        let committer = match log_committer {
            Some(c) => c,
            None => {
                // TODO: actually read the committer information from git-config, probably it should be provided here
                committer_storage = actor::Signature::empty();
                &committer_storage
            }
        };
        self.repo()?
            .refs
            .transaction()
            .prepare(edits, lock_mode)?
            .commit(committer)
            .map_err(Into::into)
    }

    fn find_reference<'a, Name, E>(&self, name: Name) -> Result<Reference<'_, Self>, reference::find::existing::Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        git_ref::file::find::Error: From<E>,
    {
        self.try_find_reference(name)?
            .ok_or(reference::find::existing::Error::NotFound)
    }

    fn try_find_reference<'a, Name, E>(&self, name: Name) -> Result<Option<Reference<'_, Self>>, reference::find::Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        git_ref::file::find::Error: From<E>,
    {
        let state = self.state();
        let repo = self.repo()?;
        match repo
            .refs
            .try_find(name, state.assure_packed_refs_uptodate(&repo.refs)?.as_ref())
        {
            Ok(r) => match r {
                Some(r) => Ok(Some(Reference::from_file_ref(r, self))),
                None => Ok(None),
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl<A> ReferenceAccessExt for A where A: easy::Access + Sized {}
