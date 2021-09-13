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
    easy::{ext::RepositoryAccessExt, reference, Reference},
    ext::ReferenceExt,
};

const DEFAULT_LOCK_MODE: git_lock::acquire::Fail = git_lock::acquire::Fail::Immediately;

/// Obtain and alter references comfortably
pub trait ReferenceAccessExt: easy::Access + Sized {
    /// Create a lightweight tag with given `name` (and without `refs/tags/` prefix) pointing to the given `target`, and return it as reference.
    ///
    /// It will be created with `constraint` which is most commonly to [only create it][PreviousValue::MustNotExist]
    /// or to [force overwriting a possibly existing tag](PreviousValue::Any).
    fn tag(
        &self,
        name: impl AsRef<str>,
        target: impl Into<ObjectId>,
        constraint: PreviousValue,
    ) -> Result<Reference<'_, Self>, reference::edit::Error> {
        let id = target.into();
        let mut edits = self.edit_reference(
            RefEdit {
                change: Change::Update {
                    log: Default::default(),
                    expected: constraint,
                    new: Target::Peeled(id),
                },
                name: format!("refs/tags/{}", name.as_ref()).try_into()?,
                deref: false,
            },
            DEFAULT_LOCK_MODE,
            None,
        )?;
        assert_eq!(edits.len(), 1, "reference splits should ever happen");
        let edit = edits.pop().expect("exactly one item");
        Ok(easy::Reference {
            inner: git_ref::Reference {
                name: edit.name,
                target: id.into(),
                peeled: None,
            },
            access: self,
        })
    }

    /// Returns the currently set namespace for references, or `None` if it is not set.
    ///
    /// Namespaces allow to partition references.
    fn namespace(&self) -> Result<Option<git_ref::Namespace>, easy::borrow::repo::Error> {
        self.repo().map(|repo| repo.deref().refs.namespace.clone())
    }

    /// Remove the currently set reference namespace and return it.
    fn clear_namespace(&mut self) -> Result<Option<git_ref::Namespace>, easy::borrow::repo::Error> {
        self.repo_mut().map(|mut repo| repo.deref_mut().refs.namespace.take())
    }

    /// Set the reference namespace to the given value, like `"foo"` or `"foo/bar"`.
    fn set_namespace<'a, Name, E>(
        &mut self,
        namespace: Name,
    ) -> Result<Option<git_ref::Namespace>, easy::reference::namespace::set::Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        git_validate::refname::Error: From<E>,
    {
        let namespace = git_ref::namespace::expand(namespace)?;
        Ok(self.repo_mut()?.deref_mut().refs.namespace.replace(namespace))
    }

    // TODO: more tests or usage
    /// Create a new reference with `name`, like `refs/heads/branch`, pointing to `target`, adhering to `constraint`
    /// during creation and writing `log_message` into the reflog. Note that a ref-log will be written even if `log_message` is empty.
    ///
    /// The newly created Reference is returned.
    fn reference<Name, E>(
        &self,
        name: Name,
        target: impl Into<ObjectId>,
        constraint: PreviousValue,
        log_message: impl Into<BString>,
    ) -> Result<Reference<'_, Self>, reference::edit::Error>
    where
        Name: TryInto<FullName, Error = E>,
        reference::edit::Error: From<E>,
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

    /// Edit a single reference as described in `edit`, handle locks via `lock_mode` and write reference logs as `log_committer`.
    ///
    /// One or more `RefEdit`s  are returned - symbolic reference splits can cause more edits to be performed. All edits have the previous
    /// reference values set to the ones encountered at rest after acquiring the respective reference's lock.
    fn edit_reference(
        &self,
        edit: RefEdit,
        lock_mode: lock::acquire::Fail,
        log_committer: Option<&actor::Signature>,
    ) -> Result<Vec<RefEdit>, reference::edit::Error> {
        self.edit_references(Some(edit), lock_mode, log_committer)
    }

    /// Edit one or more references as described by their `edits`, with `lock_mode` deciding on how to handle competing
    /// transactions. `log_committer` is the name appearing in reference logs.
    ///
    /// Returns all reference edits, which might be more than where provided due the splitting of symbolic references, and
    /// whose previous (_old_) values are the ones seen on in storage after the reference was locked.
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
                committer_storage = self.committer()?;
                &committer_storage
            }
        };
        let repo = self.repo()?;
        repo.refs
            .transaction()
            .prepare(edits, lock_mode)?
            .commit(committer)
            .map_err(Into::into)
    }

    /// Return the repository head, an abstraction to help dealing with the `HEAD` reference.
    ///
    /// The `HEAD` reference can be in various states, for more information, the documentation of [`Head`][easy::Head].
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

    /// Find the reference with the given partial or full `name`, like `main`, `HEAD`, `heads/branch` or `origin/other`,
    /// or return an error if it wasn't found.
    ///
    /// Consider [`try_find_reference(…)`][ReferenceAccessExt::try_find_reference()] if the reference might not exist
    /// without that being considered an error.
    fn find_reference<'a, Name, E>(&self, name: Name) -> Result<Reference<'_, Self>, reference::find::existing::Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        git_ref::file::find::Error: From<E>,
    {
        self.try_find_reference(name)?
            .ok_or(reference::find::existing::Error::NotFound)
    }

    /// Return a platform for iterating references.
    ///
    /// Common kinds of iteration are [all][easy::reference::iter::State::all()] or [prefixed][easy::reference::iter::State::prefixed()]
    /// references.
    fn references(&self) -> Result<easy::reference::iter::Platform<'_, Self>, easy::reference::iter::Error> {
        let state = self.state();
        let repo = self.repo()?;
        let packed_refs = state.assure_packed_refs_uptodate(&repo.refs)?;
        Ok(easy::reference::iter::Platform {
            repo,
            packed_refs,
            access: self,
        })
    }

    /// Try to find the reference named `name`, like `main`, `heads/branch`, `HEAD` or `origin/other`, and return it.
    ///
    /// Otherwise return `None` if the reference wasn't found.
    /// If the reference is expected to exist, use [`find_reference()`][ReferenceAccessExt::find_reference()].
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
