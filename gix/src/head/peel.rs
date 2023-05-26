use crate::{
    ext::{ObjectIdExt, ReferenceExt},
    Head,
};

mod error {
    use crate::{object, reference};

    /// The error returned by [`Head::peel_to_id_in_place()`][super::Head::peel_to_id_in_place()] and [`Head::into_fully_peeled_id()`][super::Head::into_fully_peeled_id()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        FindExistingObject(#[from] object::find::existing::Error),
        #[error(transparent)]
        PeelReference(#[from] reference::peel::Error),
    }
}

pub use error::Error;

use crate::head::Kind;

///
pub mod to_commit {
    use crate::object;

    /// The error returned by [`Head::peel_to_commit_in_place()`][super::Head::peel_to_commit_in_place()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Peel(#[from] super::Error),
        #[error("Branch '{name}' does not have any commits")]
        Unborn { name: gix_ref::FullName },
        #[error(transparent)]
        ObjectKind(#[from] object::try_into::Error),
    }
}

impl<'repo> Head<'repo> {
    // TODO: tests
    /// Peel this instance to make obtaining its final target id possible, while returning an error on unborn heads.
    pub fn peeled(mut self) -> Result<Self, Error> {
        self.peel_to_id_in_place().transpose()?;
        Ok(self)
    }

    // TODO: tests
    /// Follow the symbolic reference of this head until its target object and peel it by following tag objects until there is no
    /// more object to follow, and return that object id.
    ///
    /// Returns `None` if the head is unborn.
    pub fn peel_to_id_in_place(&mut self) -> Option<Result<crate::Id<'repo>, Error>> {
        Some(match &mut self.kind {
            Kind::Unborn(_name) => return None,
            Kind::Detached {
                peeled: Some(peeled), ..
            } => Ok((*peeled).attach(self.repo)),
            Kind::Detached { peeled: None, target } => {
                match target
                    .attach(self.repo)
                    .object()
                    .map_err(Into::into)
                    .and_then(|obj| obj.peel_tags_to_end().map_err(Into::into))
                    .map(|peeled| peeled.id)
                {
                    Ok(peeled) => {
                        self.kind = Kind::Detached {
                            peeled: Some(peeled),
                            target: *target,
                        };
                        Ok(peeled.attach(self.repo))
                    }
                    Err(err) => Err(err),
                }
            }
            Kind::Symbolic(r) => {
                let mut nr = r.clone().attach(self.repo);
                let peeled = nr.peel_to_id_in_place().map_err(Into::into);
                *r = nr.detach();
                peeled
            }
        })
    }

    // TODO: tests
    // TODO: something similar in `crate::Reference`
    /// Follow the symbolic reference of this head until its target object and peel it by following tag objects until there is no
    /// more object to follow, transform the id into a commit if possible and return that.
    ///
    /// Returns an error if the head is unborn or if it doesn't point to a commit.
    pub fn peel_to_commit_in_place(&mut self) -> Result<crate::Commit<'repo>, to_commit::Error> {
        let id = self.peel_to_id_in_place().ok_or_else(|| to_commit::Error::Unborn {
            name: self.referent_name().expect("unborn").to_owned(),
        })??;
        id.object()
            .map_err(|err| to_commit::Error::Peel(Error::FindExistingObject(err)))
            .and_then(|object| object.try_into_commit().map_err(Into::into))
    }

    /// Consume this instance and transform it into the final object that it points to, or `None` if the `HEAD`
    /// reference is yet to be born.
    pub fn into_fully_peeled_id(self) -> Option<Result<crate::Id<'repo>, Error>> {
        Some(match self.kind {
            Kind::Unborn(_name) => return None,
            Kind::Detached {
                peeled: Some(peeled), ..
            } => Ok(peeled.attach(self.repo)),
            Kind::Detached { peeled: None, target } => target
                .attach(self.repo)
                .object()
                .map_err(Into::into)
                .and_then(|obj| obj.peel_tags_to_end().map_err(Into::into))
                .map(|obj| obj.id.attach(self.repo)),
            Kind::Symbolic(r) => r.attach(self.repo).peel_to_id_in_place().map_err(Into::into),
        })
    }
}
