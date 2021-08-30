#![allow(missing_docs)]
use std::ops::DerefMut;

use git_hash::ObjectId;
use git_odb::Find;
use git_ref as refs;

use crate::{
    easy,
    easy::{Oid, Reference},
};

pub(crate) enum Backing {
    OwnedPacked {
        /// The validated full name of the reference.
        name: refs::FullName,
        /// The target object id of the reference, hex encoded.
        target: ObjectId,
        /// The fully peeled object id, hex encoded, that the ref is ultimately pointing to
        /// i.e. when all indirections are removed.
        object: Option<ObjectId>,
    },
    LooseFile(refs::file::loose::Reference),
}

pub mod edit {
    use git_ref as refs;

    use crate::easy;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        FileTransactionPrepare(#[from] refs::file::transaction::prepare::Error),
        #[error(transparent)]
        FileTransactionCommit(#[from] refs::file::transaction::commit::Error),
        #[error(transparent)]
        NameValidation(#[from] git_validate::reference::name::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }
}

pub mod peel_to_oid_in_place {
    use git_ref as refs;

    use crate::easy;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        LoosePeelToId(#[from] refs::file::loose::reference::peel::to_id::Error),
        #[error(transparent)]
        PackedRefsOpen(#[from] refs::packed::buffer::open::Error),
        #[error("BUG: Part of interior state could not be borrowed.")]
        BorrowState(#[from] easy::borrow::state::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }
}

// TODO: think about how to detach a Reference. It should essentially be a 'Raw' reference that should exist in `git-ref` rather than here.
impl<'repo, A> Reference<'repo, A>
where
    A: easy::Access + Sized,
{
    pub(crate) fn from_file_ref(reference: refs::file::Reference<'_>, access: &'repo A) -> Self {
        Reference {
            backing: match reference {
                refs::file::Reference::Packed(p) => Backing::OwnedPacked {
                    name: p.name.into(),
                    target: p.target(),
                    object: p
                        .object
                        .map(|hex| ObjectId::from_hex(hex).expect("a hash kind we know")),
                },
                refs::file::Reference::Loose(l) => Backing::LooseFile(l),
            }
            .into(),
            access,
        }
    }
    pub fn target(&self) -> refs::Target {
        match self.backing.as_ref().expect("always set") {
            Backing::OwnedPacked { target, .. } => refs::Target::Peeled(target.to_owned()),
            Backing::LooseFile(r) => r.target.clone(),
        }
    }

    pub fn name(&self) -> refs::FullNameRef<'_> {
        match self.backing.as_ref().expect("always set") {
            Backing::OwnedPacked { name, .. } => name,
            Backing::LooseFile(r) => &r.name,
        }
        .to_ref()
    }

    pub fn peel_to_oid_in_place(&mut self) -> Result<Oid<'repo, A>, peel_to_oid_in_place::Error> {
        let repo = self.access.repo()?;
        match self.backing.take().expect("a ref must be set") {
            Backing::LooseFile(mut r) => {
                let state = self.access.state();
                let mut pack_cache = state.try_borrow_mut_pack_cache()?;
                let oid = r
                    .peel_to_id_in_place(
                        &repo.refs,
                        state.assure_packed_refs_uptodate(&repo.refs)?.as_ref(),
                        |oid, buf| {
                            repo.odb
                                .try_find(oid, buf, pack_cache.deref_mut())
                                .map(|po| po.map(|o| (o.kind, o.data)))
                        },
                    )?
                    .to_owned();
                self.backing = Backing::LooseFile(r).into();
                Ok(Oid::from_id(oid, self.access))
            }
            Backing::OwnedPacked {
                mut target,
                mut object,
                name,
            } => {
                if let Some(peeled_id) = object.take() {
                    target = peeled_id;
                }
                self.backing = Backing::OwnedPacked {
                    name,
                    target,
                    object: None,
                }
                .into();
                Ok(Oid::from_id(target, self.access))
            }
        }
    }
}

pub mod find {
    use git_ref as refs;
    use quick_error::quick_error;

    use crate::easy;

    pub mod existing {
        use quick_error::quick_error;

        use crate::easy::reference::find;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Find(err: find::Error) {
                    display("An error occurred when trying to find a reference")
                    from()
                    source(err)
                }
                NotFound {
                    display("The reference did not exist even though that was expected")
                }
            }
        }
    }

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Find(err: refs::file::find::Error) {
                display("An error occurred when trying to find a reference")
                from()
                source(err)
            }
            PackedRefsOpen(err: refs::packed::buffer::open::Error) {
                display("The packed-refs file could not be opened")
                from()
                source(err)
            }
            BorrowState(err: easy::borrow::state::Error) {
                display("BUG: Part of interior state could not be borrowed.")
                from()
                source(err)
            }
            BorrowRepo(err: easy::borrow::repo::Error) {
                display("BUG: The repository could not be borrowed")
                from()
            }
        }
    }
}
