use std::{
    convert::TryInto,
    ops::{Deref, DerefMut},
};

use bstr::BString;
use git_actor as actor;
use git_hash::ObjectId;
use git_lock as lock;
use git_ref::{
    transaction::{Change, LogChange, PreviousValue, RefEdit, RefLog},
    FullName, PartialNameRef, Target,
};

use crate::{
    easy,
    easy::{ext::ConfigAccessExt, reference, Reference},
    ext::ReferenceExt,
};

const DEFAULT_LOCK_MODE: git_lock::acquire::Fail = git_lock::acquire::Fail::Immediately;

/// Obtain and alter references comfortably
pub trait ReferenceAccessExt: easy::Access + Sized {
    fn tag(
        &self,
        name: impl AsRef<str>,
        target: impl Into<ObjectId>,
        constraint: PreviousValue,
    ) -> Result<Vec<RefEdit>, reference::edit::Error> {
        self.edit_reference(
            RefEdit {
                change: Change::Update {
                    log: Default::default(),
                    expected: constraint,
                    new: Target::Peeled(target.into()),
                },
                name: format!("refs/tags/{}", name.as_ref()).try_into()?,
                deref: false,
            },
            DEFAULT_LOCK_MODE,
            None,
        )
    }

    fn namespace(&self) -> Result<Option<git_ref::Namespace>, easy::borrow::repo::Error> {
        self.repo().map(|repo| repo.deref().namespace.clone())
    }

    fn clear_namespace(&mut self) -> Result<Option<git_ref::Namespace>, easy::borrow::repo::Error> {
        self.repo_mut().map(|mut repo| repo.deref_mut().namespace.take())
    }

    fn set_namespace<'a, Name, E>(
        &mut self,
        namespace: Name,
    ) -> Result<Option<git_ref::Namespace>, easy::reference::namespace::set::Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        git_validate::refname::Error: From<E>,
    {
        let namespace = git_ref::namespace::expand(namespace)?;
        Ok(self.repo_mut()?.deref_mut().namespace.replace(namespace))
    }

    // TODO: more tests or usage
    fn reference<Name, E>(
        &self,
        name: Name,
        target: impl Into<ObjectId>,
        constraint: PreviousValue,
        log_message: impl Into<BString>,
    ) -> Result<Reference<'_, Self>, reference::create::Error>
    where
        Name: TryInto<FullName, Error = E>,
        reference::create::Error: From<E>,
    {
        let name = name.try_into()?;
        let id = target.into();
        let mut edits = self.edit_reference(
            RefEdit {
                change: Change::Update {
                    log: LogChange {
                        mode: RefLog::AndReference,
                        force_create_reflog: false,
                        message: log_message.into(),
                    },
                    expected: constraint,
                    new: Target::Peeled(id),
                },
                name,
                deref: false,
            },
            DEFAULT_LOCK_MODE,
            None,
        )?;
        assert_eq!(
            edits.len(),
            1,
            "only one reference can be created, splits aren't possible"
        );

        Ok(git_ref::Reference {
            name: edits.pop().expect("exactly one edit").name,
            target: Target::Peeled(id),
            peeled: None,
        }
        .attach(self))
    }

    fn edit_reference(
        &self,
        edit: RefEdit,
        lock_mode: lock::acquire::Fail,
        log_committer: Option<&actor::Signature>,
    ) -> Result<Vec<RefEdit>, reference::edit::Error> {
        self.edit_references(Some(edit), lock_mode, log_committer)
    }

    // NOTE: Returned edits don't hide the namespace.
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
                committer_storage = self.committer();
                &committer_storage
            }
        };
        let repo = self.repo()?;
        let transaction = repo.refs.transaction();
        match &repo.namespace {
            Some(namespace) => transaction.namespace(namespace.to_owned()),
            None => transaction,
        }
        .prepare(edits, lock_mode)?
        .commit(committer)
        .map_err(Into::into)
    }

    fn head(&self) -> Result<easy::Head<'_, Self>, reference::find::existing::Error> {
        let head = self.find_reference("HEAD")?;
        Ok(match head.inner.target {
            Target::Symbolic(branch) => match self.find_reference(branch.to_partial()) {
                Ok(r) => easy::head::Kind::Symbolic(r.detach()),
                Err(reference::find::existing::Error::NotFound) => easy::head::Kind::Unborn(branch),
                Err(err) => return Err(err),
            },
            Target::Peeled(target) => easy::head::Kind::Detached {
                target,
                peeled: head.inner.peeled,
            },
        }
        .attach(self))
    }

    fn find_reference<'a, Name, E>(&self, name: Name) -> Result<Reference<'_, Self>, reference::find::existing::Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        git_ref::file::find::Error: From<E>,
    {
        self.try_find_reference(name)?
            .ok_or(reference::find::existing::Error::NotFound)
    }

    fn iter_references(&self) -> Result<easy::iter::references::State<'_, Self>, easy::iter::references::Error> {
        let state = self.state();
        let repo = self.repo()?;
        let packed_refs = state.assure_packed_refs_uptodate(&repo.refs)?;
        Ok(easy::iter::references::State {
            repo,
            packed_refs,
            access: self,
        })
    }

    fn try_find_reference<'a, Name, E>(&self, name: Name) -> Result<Option<Reference<'_, Self>>, reference::find::Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        git_ref::file::find::Error: From<E>,
    {
        let state = self.state();
        let repo = self.repo()?;
        match repo.refs.try_find(
            name,
            state.assure_packed_refs_uptodate(&repo.refs)?.packed_refs.as_ref(),
        ) {
            Ok(r) => match r {
                Some(r) => Ok(Some(Reference::from_ref(r, self))),
                None => Ok(None),
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl<A> ReferenceAccessExt for A where A: easy::Access + Sized {}
